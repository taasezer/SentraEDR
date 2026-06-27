#[cfg(windows)]
use tokio::net::windows::named_pipe::{ClientOptions, ServerOptions, NamedPipeServer, NamedPipeClient};
#[cfg(windows)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::io;

use sentra_core::error::{Result, SentraError};
use crate::messages::IpcMessage;

pub const PIPE_NAME: &str = r"\\.\pipe\sentra-edr-ipc";

#[cfg(windows)]
pub struct IpcServer {
    server: NamedPipeServer,
}

#[cfg(windows)]
impl IpcServer {
    pub fn new() -> Result<Self> {
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(PIPE_NAME)
            .map_err(|e| SentraError::Channel(format!("Failed to create IPC pipe: {}", e)))?;
            
        Ok(Self { server })
    }

    pub async fn wait_for_client(&mut self) -> Result<()> {
        self.server.connect().await.map_err(|e| SentraError::Channel(format!("Failed to connect client: {}", e)))
    }

    pub async fn send_message(&mut self, msg: &IpcMessage) -> Result<()> {
        let serialized = serde_json::to_string(msg).map_err(|e| SentraError::SerializationError(e.to_string()))?;
        let payload = format!("{}\n", serialized);
        self.server.write_all(payload.as_bytes()).await.map_err(|e| SentraError::Channel(e.to_string()))?;
        Ok(())
    }
}

#[cfg(windows)]
pub struct IpcClient {
    client: NamedPipeClient,
}

#[cfg(windows)]
impl IpcClient {
    pub async fn connect() -> Result<Self> {
        let client = ClientOptions::new()
            .open(PIPE_NAME)
            .map_err(|e| SentraError::Channel(format!("Failed to connect to IPC pipe: {}", e)))?;
            
        Ok(Self { client })
    }

    pub async fn receive_message(&mut self) -> Result<IpcMessage> {
        let mut buffer = [0u8; 8192];
        let n = self.client.read(&mut buffer).await.map_err(|e| SentraError::Channel(e.to_string()))?;
        if n == 0 {
            return Err(SentraError::Channel("IPC pipe closed".to_string()));
        }
        
        let payload = String::from_utf8_lossy(&buffer[..n]);
        // Handle newline delimiter
        let json_str = payload.trim();
        
        let msg: IpcMessage = serde_json::from_str(json_str)
            .map_err(|e| SentraError::SerializationError(e.to_string()))?;
            
        Ok(msg)
    }
}
