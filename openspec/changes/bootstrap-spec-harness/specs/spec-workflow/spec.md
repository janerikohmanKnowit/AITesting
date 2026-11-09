## Purpose

Give developers and host AI assistants a repeatable way to plan changes and verify their implementation with inspectable evidence.

## ADDED Requirements

### Requirement: Inspectable feature planning
The workflow SHALL retain a proposal, behavior scenarios, design, and checkable tasks before implementation. Planning-only requests SHALL stop after producing artifacts.

#### Scenario: End-to-end request
- **WHEN** a user requests implementation through the delivery workflow
- **THEN** the assistant writes planning artifacts before code and records verification evidence afterward

#### Scenario: Proposal-only request
- **WHEN** a user requests only a proposal
- **THEN** the assistant produces artifacts without implementing the change

### Requirement: Reproducible verification
The repository SHALL provide one verification command that fails when strict spec validation, formatting, linting, tests, or compilation fails.

#### Scenario: Verification run
- **WHEN** a developer runs `npm run verify` after setup
- **THEN** strict OpenSpec validation and the Rust checks run, and any failing command causes a nonzero exit
