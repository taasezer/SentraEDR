use engine_etw::EtwSession;

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
        Box::new(engine_detection::rules::SuspiciousProcessRule),
    ];
    let mut detection_engine = engine_detection::pipeline::DetectionPipeline::new(rules);

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
            }
        }
    }
}
