# Design

## Context
See proposal.md. The starting repository contains a README and license only.

## Goals / Non-Goals
Provide inspectable artifacts and one meaningful executable example. Avoid a
custom AI API client, background loops, and an unrelated application framework.

## Decisions
- Use OpenSpec 1.16.1 with its standard spec-driven schema and generated agent
  integrations. A local npm dependency and lockfile make installation repeatable;
  a global install would hide the selected version.
- Add an end-to-end skill: upstream propose intentionally stops at planning.
  This preserves proposal-only usage while supporting requests to implement.
- Use a dependency-free Rust library to exercise all gates cheaply. ASCII-only
  slugs make the Unicode behavior explicit rather than implying transliteration.
- Keep acceptance requirements in specs, choices here, steps in tasks, and actual
  evidence in verification.md. Structural validation cannot prove semantics.
- Use one portable Node verification runner locally and in GitHub Actions.

## Risks / Trade-offs
- Specs and tests can share a mistaken assumption → review scenario-to-test mapping.
- Generated instructions may change on upgrades → pin versions and inspect diffs.
- Rust toolchains require network on first setup → retain local tools and lockfiles.

## Verification Approach
Validate both capability specs strictly. Exercise each slug scenario through
integration tests and a public API doctest. Run formatting, warning-free Clippy,
tests, and a locked build. Demonstrate that a wrong implementation fails tests.
