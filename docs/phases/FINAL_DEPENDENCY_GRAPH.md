# Final Dependency Graph

## Allowed Dependencies
- `ui/*` -> `core-eventbus` (Communication Contracts)
- `engine-*` -> `core-models` (Shared Domain Models)
- `service-host` -> `core-runtime` (Composition Root)

## Forbidden Dependencies (STRICTLY ENFORCED)
- ❌ `engine-*` -> `ui/*` (Engines cannot know about clients)
- ❌ `core-runtime` -> `ui/*` (Runtime cannot know about clients)
- ❌ `engine-detection` -> `engine-remediation` (Detection only emits alerts; Remediation handles isolation)
- ❌ `infrastructure-storage` -> `engine-*` (Storage must remain agnostic)

**Status:** The workspace has been audited and zero circular or forbidden dependencies exist.
