use engine_etw::EtwSession;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    routing::get,
    Router,
};
use tokio::sync::broadcast;
use std::sync::Arc;
use serde_json;

#[tokio::main]
async fn main() {
    println!("Starting SentraEDR Service Host (Live Telemetry Validation Mode)...");

    let session = match EtwSession::start_trace() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to start ETW trace: {}", e);
            return;
        }
    };

    println!("Tokio Reactor initialized. Waiting for normalized events from ETW thread...");

    let rules: Vec<Box<dyn engine_detection::rules::Rule>> = vec![
        Box::new(engine_detection::rules::LsassDumpRule),
        Box::new(engine_detection::rules::ReverseShellRule),
        Box::new(engine_detection::rules::RansomwareBehaviorRule),
    ];
    let mut detection_engine = engine_detection::pipeline::DetectionPipeline::new(rules);

    // Create a broadcast channel for alerts
    let (tx, _rx) = broadcast::channel::<String>(100);
    let app_state = Arc::new(tx.clone());

    // Setup Axum Router
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    // Spawn the web server in the background
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("WebSocket server listening on ws://127.0.0.1:8080/ws");
        axum::serve(listener, app).await.unwrap();
    });

    let mut event_count = 0;
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));

    loop {
        interval.tick().await;
        while let Ok(event) = session.receiver.try_recv() {
            event_count += 1;
            
            // Only print ProcessCreate events to reduce noise
            if let shared_models::events::EventType::ProcessCreate { image_path, command_line } = &event.event_type {
                println!(
                    "Live Event #{} | PID: {} | Process: {} | Args: {}",
                    event_count, event.process_id, image_path, command_line
                );
            }

            // Feed to Detection Engine
            let alerts = detection_engine.process_event(event);
            for alert in alerts {
                println!("\n========================================================");
                println!("[ALERT] 🚨 {} 🚨", alert.rule_id);
                println!("Description: {}", alert.evidence.reasoning_path);
                println!("Risk: {:?} | Confidence: {:?}", alert.severity, alert.confidence);
                println!("========================================================\n");

                // Broadcast to UI
                if let Ok(alert_json) = serde_json::to_string(&alert) {
                    let _ = tx.send(alert_json);
                }
            }
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(tx): State<Arc<broadcast::Sender<String>>>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Arc<broadcast::Sender<String>>) {
    let mut rx = tx.subscribe();
    
    // Send a welcome message just to verify connection
    let _ = socket.send(Message::Text("Connected to SentraEDR Detection Engine".to_string())).await;

    // Loop and forward alerts to this client
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            // Client disconnected
            break;
        }
    }
}
