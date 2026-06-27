# ADR 0004: Network Provider Abstraction

## Status
Accepted

## Context
Network events can be sourced from various technologies depending on the deployment environment and performance constraints (ETW, Npcap, WinDivert, Kernel drivers). Hardcoding the network analyzer to one source prevents the engine from scaling or being tested offline.

## Decision
We will define a `NetworkProvider` trait that abstracts the transport mechanism. 
The analyzer will consume network events transparently, regardless of whether they originated from an ETW TCP/IP provider or a low-level packet capture driver.

## Alternatives Considered
- **Direct ETW Binding:** Highly performant, but limits visibility into packet payloads if later phases require L7 inspection, and breaks offline testing.
- **Strict Npcap Dependency:** Forces third-party driver installation on all endpoints, violating our lightweight and native architectural goals.

## Trade-offs
- *Pros:* Decouples transport from analysis. Allows future integration of WinDivert for inline inspection without rewriting the analyzer.
- *Cons:* Requires a strict translation layer (`ConnectionIdentity`) between raw packets/events and the analyzer, adding a minor abstraction overhead.

## Consequences
The engine must assume all underlying providers are purely observational. If a provider has blocking capabilities (e.g., WinDivert), those capabilities must NOT be exposed through the `NetworkProvider` trait during this phase.
