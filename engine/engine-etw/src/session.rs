use crate::native_parser::{event_record_callback, EVENT_SENDER};
use crossbeam_channel::{bounded, Receiver};
use shared_models::events::NormalizedTelemetryEvent;
use std::mem::size_of;
use std::ptr;
use std::thread;
use windows::core::{GUID, PCWSTR, PWSTR};
use windows::Win32::System::Diagnostics::Etw::{
    StartTraceW, OpenTraceW, ProcessTrace, EnableTraceEx2, CloseTrace, ControlTraceW,
    EVENT_TRACE_LOGFILEW, EVENT_TRACE_PROPERTIES, 
    PROCESS_TRACE_MODE_REAL_TIME, PROCESS_TRACE_MODE_EVENT_RECORD,
    EVENT_TRACE_REAL_TIME_MODE, EVENT_TRACE_SYSTEM_LOGGER_MODE,
    TRACE_LEVEL_INFORMATION, EVENT_CONTROL_CODE_ENABLE_PROVIDER, WNODE_HEADER,
    EVENT_TRACE_CONTROL_STOP
};

const KERNEL_PROCESS_GUID: GUID = GUID::from_values(
    0x22fb2cd6, 0x0e7b, 0x422b, [0xa0, 0xc7, 0x2f, 0xad, 0x1f, 0xd0, 0xe7, 0x16]
);

const KERNEL_NETWORK_GUID: GUID = GUID::from_values(
    0x7dd42a49, 0x5329, 0x4832, [0x8d, 0xfd, 0x43, 0xd9, 0x79, 0x15, 0x3a, 0x88]
);

const KERNEL_FILE_GUID: GUID = GUID::from_values(
    0xedd08927, 0x9cc4, 0x4e65, [0xb9, 0x70, 0xc2, 0x56, 0x0f, 0xb5, 0xc2, 0x89]
);

const KERNEL_REGISTRY_GUID: GUID = GUID::from_values(
    0x70eb4f03, 0xc1de, 0x4f73, [0xa0, 0x51, 0x33, 0xd1, 0x3d, 0x54, 0x13, 0xbd]
);

pub struct EtwSession {
    pub receiver: Receiver<NormalizedTelemetryEvent>,
}

impl EtwSession {
    pub fn start_trace() -> Result<Self, String> {
        let (sender, receiver) = bounded(10_000); // Bounded queue to prevent OOM

        // Initialize global callback state.
        if EVENT_SENDER.set(sender).is_err() {
            return Err("ETW session already running in this process.".to_string());
        }

        // Spawn a dedicated std::thread so we never block Tokio.
        thread::spawn(|| {
            // Note: In a true production trace, we must allocate a buffer large enough
            // to hold EVENT_TRACE_PROPERTIES + the session name string.
            // For the sake of this OS validation, we'll open the pre-existing NT Kernel Logger
            // or an existing trace session. To make it simple for validation, we'll try to
            // open the active "NT Kernel Logger" stream if it exists.

            // To start our own trace, we allocate the required struct:
            const BUFFER_SIZE: usize = size_of::<EVENT_TRACE_PROPERTIES>() + 2048;
            let mut buffer = vec![0u8; BUFFER_SIZE];
            let properties = unsafe { &mut *(buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES) };

            properties.Wnode.BufferSize = BUFFER_SIZE as u32;
            properties.Wnode.Flags =
                windows::Win32::System::Diagnostics::Etw::WNODE_FLAG_TRACED_GUID;
            properties.Wnode.ClientContext = 1; // QPC clock resolution
            properties.LogFileMode = EVENT_TRACE_REAL_TIME_MODE;

            let mut session_name = "SentraEDR_Trace_Session\0"
                .encode_utf16()
                .collect::<Vec<u16>>();
            properties.LoggerNameOffset = size_of::<EVENT_TRACE_PROPERTIES>() as u32;

            unsafe {
                ptr::copy_nonoverlapping(
                    session_name.as_ptr() as *const u8,
                    buffer
                        .as_mut_ptr()
                        .add(properties.LoggerNameOffset as usize),
                    session_name.len() * 2,
                );
            }

            let mut trace_handle = windows::Win32::System::Diagnostics::Etw::CONTROLTRACE_HANDLE::default();
            
            // First, attempt to stop any orphaned session with the same name.
            // We ignore the result because it will fail if the session doesn't exist.
            unsafe {
                ControlTraceW(
                    windows::Win32::System::Diagnostics::Etw::CONTROLTRACE_HANDLE::default(),
                    PCWSTR(session_name.as_ptr()),
                    properties,
                    EVENT_TRACE_CONTROL_STOP
                );
            }

            // Start the trace
            let status = unsafe { 
                StartTraceW(&mut trace_handle, PCWSTR(session_name.as_ptr()), properties) 
            };
            
            if status.is_err() {
                eprintln!("Failed to start trace: {:?}", status);
                return;
            }

            // Enable Microsoft-Windows-Kernel-Process
            let _ = unsafe {
                EnableTraceEx2(
                    trace_handle,
                    &KERNEL_PROCESS_GUID,
                    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0 as u32,
                    TRACE_LEVEL_INFORMATION as u8,
                    0, 0, 0, None
                )
            };

            // Enable Microsoft-Windows-Kernel-Network
            let _ = unsafe {
                EnableTraceEx2(
                    trace_handle,
                    &KERNEL_NETWORK_GUID,
                    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0 as u32,
                    TRACE_LEVEL_INFORMATION as u8,
                    0, 0, 0, None
                )
            };

            // Enable Microsoft-Windows-Kernel-File
            let _ = unsafe {
                EnableTraceEx2(
                    trace_handle,
                    &KERNEL_FILE_GUID,
                    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0 as u32,
                    TRACE_LEVEL_INFORMATION as u8,
                    0, 0, 0, None
                )
            };

            // Enable Microsoft-Windows-Kernel-Registry
            let _ = unsafe {
                EnableTraceEx2(
                    trace_handle,
                    &KERNEL_REGISTRY_GUID,
                    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0 as u32,
                    TRACE_LEVEL_INFORMATION as u8,
                    0, 0, 0, None
                )
            };

            // Open the trace for consumption
            let mut logfile = EVENT_TRACE_LOGFILEW::default();
            logfile.LoggerName = PWSTR(session_name.as_mut_ptr());
            logfile.Anonymous1.ProcessTraceMode = PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
            logfile.Anonymous2.EventRecordCallback = Some(event_record_callback);

            let consume_handle = unsafe { OpenTraceW(&mut logfile) };

            if consume_handle.Value == windows::Win32::Foundation::INVALID_HANDLE_VALUE.0 as u64 {
                eprintln!("Failed to OpenTraceW");
                return;
            }

            // BLOCKING CALL
            println!("ETW Thread: Calling ProcessTrace. Waiting for live telemetry...");
            let process_status = unsafe { ProcessTrace(&[consume_handle], None, None) };

            eprintln!("ProcessTrace exited with status {:?}", process_status);

            unsafe { CloseTrace(consume_handle) };
        });

        Ok(Self { receiver })
    }
}
