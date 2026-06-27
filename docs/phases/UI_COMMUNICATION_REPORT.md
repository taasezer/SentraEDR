# UI Communication Report

## IPC Surface
The Tauri frontend sends stringified JSON commands. `tauri-app` parses and strongly types them before passing them to `ui-api-client`.

## Approved Command Hooks
- `invoke_approve_remediation`: Translates into an `ApproveRemediationCommand` on the `CommandBus`.
- `invoke_update_config`: Translates into an `UpdateConfigurationCommand`.

## Event Subscriptions
- `subscribe_alerts`: Listens for `AlertFired` on `EventBus`.
- `subscribe_health`: Listens for `SystemHealthUpdate` on `EventBus`.
