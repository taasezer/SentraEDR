# Software Supply Chain Report

## Tooling Integration
- **Vulnerability Scanning:** Executed `cargo audit` cleanly across 21 crates.
- **Dependency Graph Control:** Implemented `cargo-deny` with strict constraints (no multiple version resolution paths for major dependencies like `tokio`, `serde`).
- **SBOM Generation:** Integrated CycloneDX SBOM generation into the CI artifact pipeline.
- **Licenses:** Audited via `cargo-deny`. Only MIT, Apache 2.0, and BSD-3 dependencies are permitted in the workspace.
