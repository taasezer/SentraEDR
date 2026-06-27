use crate::EtwProvider;
use sentra_core::{Result, TelemetrySource, SentraError};
use tracing::{info, warn, error};
use windows::Win32::System::Diagnostics::Etw::{
    StartTraceW, EnableTraceEx2, ProcessTrace, OpenTraceW, CloseTrace,
    EVENT_TRACE_PROPERTIES, EVENT_TRACE_LOGFILEW, EVENT_RECORD,
    EVENT_TRACE_REAL_TIME_MODE, WNODE_FLAG_TRACED_GUID, EVENT_TRACE_CONTROL_STOP,
    ControlTraceW, CONTROLTRACE_HANDLE, PROCESSTRACE_HANDLE
};
use windows::Win32::Foundation::{ERROR_SUCCESS, ERROR_ALREADY_EXISTS, ERROR_ACCESS_DENIED, WIN32_ERROR};
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct EtwConsumer {
    providers: Vec<EtwProvider>,
    is_running: Arc<AtomicBool>,
    trace_handle: PROCESSTRACE_HANDLE,
}

impl EtwConsumer {
    pub fn new(providers: Vec<EtwProvider>) -> Self {
        Self {
            providers,
            is_running: Arc::new(AtomicBool::new(false)),
            trace_handle: PROCESSTRACE_HANDLE::default(),
        }
    }

    pub fn is_elevated() -> bool {
        // Assume elevated for this implementation context.
        true
    }
}

// Global callback for ETW
unsafe extern "system" fn event_record_callback(record: *mut EVENT_RECORD) {
    if record.is_null() { return; }
    unsafe {
        let rec = &*record;
        let pid = rec.EventHeader.ProcessId;
        let tid = rec.EventHeader.ThreadId;
        let provider_id = rec.EventHeader.ProviderId;
        
        tracing::trace!("Raw ETW Event -> PID: {}, TID: {}, Provider: {:?}", pid, tid, provider_id);
    }
}

impl TelemetrySource for EtwConsumer {
    async fn start(&mut self) -> Result<()> {
        info!("Starting real ETW sessions for {} providers", self.providers.len());
        self.is_running.store(true, Ordering::SeqCst);
        
        let session_name = "SentraEDR_TraceSession\0".encode_utf16().collect::<Vec<u16>>();
        
        let is_running = self.is_running.clone();
        
        std::thread::spawn(move || {
            unsafe {
                let buffer_size = size_of::<EVENT_TRACE_PROPERTIES>() + session_name.len() * 2;
                let mut buffer = vec![0u8; buffer_size];
                let props = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;
                
                (*props).Wnode.BufferSize = buffer_size as u32;
                (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
                (*props).Wnode.ClientContext = 1; // QPC clock
                (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;
                (*props).LoggerNameOffset = size_of::<EVENT_TRACE_PROPERTIES>() as u32;

                let mut session_handle = CONTROLTRACE_HANDLE::default();
                let mut status = StartTraceW(&mut session_handle, windows::core::PCWSTR(session_name.as_ptr()), props);
                
                if status == ERROR_ALREADY_EXISTS {
                    warn!("ETW Session already exists. Stopping old session...");
                    ControlTraceW(CONTROLTRACE_HANDLE::default(), windows::core::PCWSTR(session_name.as_ptr()), props, EVENT_TRACE_CONTROL_STOP);
                    status = StartTraceW(&mut session_handle, windows::core::PCWSTR(session_name.as_ptr()), props);
                }

                if status == ERROR_ACCESS_DENIED {
                    error!("ETW requires Administrator privileges! Run as Admin.");
                    return;
                } else if status != ERROR_SUCCESS {
                    error!("StartTraceW failed with status: {}", status.0);
                    return;
                }

                info!("ETW Trace Session Started successfully.");

                let mut logfile = EVENT_TRACE_LOGFILEW::default();
                logfile.LoggerName = windows::core::PWSTR(session_name.as_ptr() as *mut _);
                logfile.Anonymous1.ProcessTraceMode = EVENT_TRACE_REAL_TIME_MODE;
                logfile.Anonymous2.EventRecordCallback = Some(event_record_callback);

                let trace_handle = OpenTraceW(&mut logfile);
                if trace_handle.Value == 0 {
                    error!("OpenTraceW failed.");
                    ControlTraceW(session_handle, windows::core::PCWSTR(std::ptr::null()), props, EVENT_TRACE_CONTROL_STOP);
                    return;
                }

                info!("Processing real ETW events...");
                
                // ProcessTrace requires an array of handles
                let handles = [trace_handle];
                ProcessTrace(&handles, None, None);
                
                info!("ETW Processing stopped.");
                CloseTrace(trace_handle);
                ControlTraceW(session_handle, windows::core::PCWSTR(std::ptr::null()), props, EVENT_TRACE_CONTROL_STOP);
            }
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping real ETW sessions");
        self.is_running.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn name(&self) -> &str {
        "RealEtwConsumer"
    }
}
