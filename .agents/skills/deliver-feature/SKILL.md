---
name: deliver-feature
description: Take a feature request through OpenSpec artifacts, implementation, tests, and verification. Use for end-to-end feature or bug-fix work in this repository.
---

Read `AGENTS.md` and follow its Default workflow in order. The user's feature
request is the input; derive a descriptive kebab-case change name. Use OpenSpec's
CLI directly for planning rather than invoking the proposal-only skill. Obtain
artifact instructions and their dependency order from the CLI, and write the
artifacts before code. Briefly present the plan, then implement when authorized
by the request. Use test-first behavior checks and keep task checkboxes current.

Validate the specs and run `npm run verify`; write `verification.md` with
requirement/scenario-to-test mapping and actual command outcomes. Review the
implementation against every scenario. Fix missing coverage before claiming
done. Resume existing changes when appropriate rather than creating duplicates.
Leave completed work active for review unless archiving was requested. If the
request is explicitly planning-only, write artifacts and stop without coding.
