use windows::Win32::System::Diagnostics::Etw::{OpenTraceW, ProcessTrace, StartTraceW};

pub struct EtwSession {
    // Manages TRACEHANDLE and isolating the C-API ProcessTrace call into a dedicated thread
    // separate from the Tokio async reactor.
}
