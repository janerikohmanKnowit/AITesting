# Bootstrap the spec-driven harness

## Why

The repository has no implementation or workflow. Give a host AI a repeatable
path from a plain-language request to specs, tasks, code, tests, and evidence.

## What Changes

Install pinned OpenSpec tooling, Codex/Claude integrations, an end-to-end skill,
Rust verification gates, and CI. Include a tiny Rust slug library as a worked
example, with explicit ASCII behavior and no external Rust dependencies.

## Capabilities

### New Capabilities
- `spec-workflow`: reproducible specification and verification workflow.
- `slug-generation`: a small tested example for tracing requirements to code.

### Modified Capabilities
None.

## Impact

Adds Node development tooling and a Rust workspace. No running service, paid AI
API, or credentials are required. Autonomous agent loops, a real product, Unicode
transliteration, and installing several competing frameworks are out of scope.
