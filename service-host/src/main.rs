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

    let mut event_count = 0;

    // Loop asynchronously reading from the bounded crossbeam channel
    // Because crossbeam is blocking, we must yield to Tokio or use try_recv inside a loop.
    // In true production we would use `tokio::task::spawn_blocking` to consume the crossbeam channel,
    // or wrap it in a stream. For this validation, we will poll it using try_recv.

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));

    loop {
        interval.tick().await;
        while let Ok(event) = session.receiver.try_recv() {
            event_count += 1;
            println!(
                "Received Live OS Event #{} | ProcessId: {} | Details: {:?}",
                event_count, event.process_id, event.event_type
            );
        }
    }
}
