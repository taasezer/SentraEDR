# Trust and Signing Model

## Cryptographic Operations
- **Binary Signing:** All executables (`.exe`), `.dll` dependencies, and MSI installers are signed with an EV Code Signing Certificate during CI release.
- **Rule Pack Signing:** Rules distributed dynamically must be signed with a secondary detached signature (e.g., Ed25519) verified strictly by the `RuleRegistry` prior to load.
- **Plugin Signing:** Future plugins must conform to the same Authenticode or detached signature standard. Any unsigned plugin or rule will result in a hard `Panic` at Boot unless `DeveloperMode` is explicitly enabled.
