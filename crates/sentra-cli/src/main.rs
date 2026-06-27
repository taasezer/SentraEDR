use clap::{Parser, Subcommand};
use sentra_core::{Result, SentraConfig};
use tracing::info;

#[derive(Parser)]
#[command(name = "sentra", about = "SentraEDR Command Line Interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a manual scan
    Scan {
        #[arg(short, long)]
        target: Option<String>,
    },
    /// Show system health
    Health,
    /// Install Windows service
    Install,
    /// Uninstall Windows service
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let _config = SentraConfig::default();

    match &cli.command {
        Commands::Scan { target } => {
            info!("Starting manual scan. Target: {:?}", target);
            // In a real implementation we would send an IPC message to the service
        }
        Commands::Health => {
            info!("Requesting health status from SentraEDR service...");
            // Real implementation sends HealthRequest via IPC
        }
        Commands::Install => {
            info!("Installing SentraEDR Windows Service...");
            // Delegate to sentra-service installation logic
        }
        Commands::Uninstall => {
            info!("Uninstalling SentraEDR Windows Service...");
            // Delegate to sentra-service uninstallation logic
        }
    }

    Ok(())
}
