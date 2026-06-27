# Final Security Model

## Trust Boundaries
- **Kernel / ETW:** Absolute trust boundary. Handled exclusively by `engine-etw` parsing Raw Memory Spans.
- **Agent / Core Runtime:** Orchestration boundary. Binds all components but runs zero business logic.
- **Rule SDK:** Low Trust boundary. Rules operate inside a strictly bounded CPU/Memory context and can never alter host state directly.
- **Storage:** High Trust boundary. Encrypted SQLite persistence layer.
- **UI:** Zero Trust boundary. Assumes all incoming IPC commands from the Tauri WebView could be compromised, hence rigid parsing and restricted capability mappings.

Status: **VALIDATED & COMPLETE**
