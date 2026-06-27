# Windows Service Architecture

The `service-host` crate exclusively encapsulates the Windows Service Control Manager APIs (`RegisterServiceCtrlHandlerExW`). 
It handles `SERVICE_CONTROL_STOP` and `SERVICE_CONTROL_SHUTDOWN`, mapping them directly to the `CancellationToken` hierarchy inside the `RuntimeBuilder`. This design completely isolates the core Runtime from OS-specific lifecycle concepts.
