# UI Dependency Review

## Constraints Validated
- `ui-api-client` -> `core-eventbus` (ALLOWED): Subscribes to public interfaces.
- `ui-api-client` -> `engine-etw` (FORBIDDEN): Verified zero imports.
- `ui-core` -> `infrastructure-storage` (FORBIDDEN): Verified zero imports.

**Decision: PASS**
The `ui` workspace strictly maintains its client boundaries and knows nothing about the core runtime or engine implementations.
