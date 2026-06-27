pub mod orchestrator;

use orchestrator::SentraOrchestrator;
use sentra_core::{Result, SentraConfig};
use std::ffi::OsString;
use tracing::info;
use windows_service::service_dispatcher;

define_windows_service!(ffi_service_main, sentra_service_main);

const SERVICE_NAME: &str = "SentraEDR";

fn sentra_service_main(_arguments: Vec<OsString>) {
    tracing_subscriber::fmt::init();
    info!("Starting SentraEDR Windows Service...");

    let config = SentraConfig::default();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    runtime.block_on(async {
        let mut orchestrator = SentraOrchestrator::new(config);
        if let Err(e) = orchestrator.run().await {
            tracing::error!("Orchestrator failed: {}", e);
        }
    });
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Check if we are running as a service
    if let Err(_e) = service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
        info!("Not running as a service. Starting in console mode...");
        let config = SentraConfig::default();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime");

        runtime.block_on(async {
            let mut orchestrator = SentraOrchestrator::new(config);
            if let Err(e) = orchestrator.run().await {
                tracing::error!("Orchestrator failed: {}", e);
            }
        });
    }

    Ok(())
}

#[macro_export]
macro_rules! define_windows_service {
    ($ffi_name:ident, $rust_name:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "system" fn $ffi_name(
            dw_num_services_args: u32,
            lp_service_arg_vectors: *mut *mut u16,
        ) {
            let args = unsafe {
                windows_service::os_string_array_to_vec(
                    dw_num_services_args,
                    lp_service_arg_vectors,
                )
            };
            $rust_name(args);
        }
    };
}
