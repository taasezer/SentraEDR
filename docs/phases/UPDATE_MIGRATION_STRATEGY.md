# Update and Migration Strategy

## Forward Compatibility
- **Configuration Migration:** SentraEDR version bumps will support automatic `.json` configuration patching using a defined set of migration hooks inside `core-runtime`.
- **Schema Migration:** Future versions of SQLite schemas will execute via an atomic `.sql` migration path during startup before the `StorageProvider` marks itself as `Ready`.
- **Rule Migration:** Rules deployed via the SDK maintain a hard version pin. Deprecated rules are automatically sandboxed and reported in the dashboard if an agent is upgraded.

Status: **DEFINED**
