use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{error, info};

pub fn start_file_watcher(watch_dir: PathBuf) -> mpsc::Receiver<PathBuf> {
    let (tx, rx) = mpsc::channel(100);

    std::thread::spawn(move || {
        let tx_clone = tx.clone();
        
        // Setup the notify watcher
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    if let EventKind::Create(_) | EventKind::Modify(notify::event::ModifyKind::Name(_)) = event.kind {
                        for path in event.paths {
                            let _ = tx_clone.blocking_send(path);
                        }
                    }
                }
                Err(e) => error!("File watcher error: {:?}", e),
            }
        });

        match watcher {
            Ok(mut w) => {
                info!("Starting File System Watcher on {:?}", watch_dir);
                if let Err(e) = w.watch(&watch_dir, RecursiveMode::Recursive) {
                    error!("Failed to watch directory: {:?}", e);
                    return;
                }
                
                // Keep the thread alive, and the watcher alive
                // We use a simple message loop that blocks forever
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                }
            }
            Err(e) => {
                error!("Failed to create recommended_watcher: {:?}", e);
            }
        }
    });

    rx
}
