# Working in this repository

Use OpenSpec for feature work. Read `openspec/config.yaml`, the relevant main
specs, and the selected change before coding. Use the existing checkout; each
cloud task is already isolated. Do not create a worktree unless requested.

## Default workflow

For a request to build or fix something, use the `deliver-feature` skill in
`.agents/skills/deliver-feature/SKILL.md`. It covers the whole lifecycle below.
For an explicit request for a proposal only, use `openspec-propose` and stop
after the artifacts. The upstream proposal skill intentionally stops at planning;
do not invoke it as the entry point for an end-to-end implementation request.

1. Inspect the relevant code and specs. State reasonable assumptions; ask only
   when missing information prevents useful progress. Respect requested scope.
2. Create a kebab-case change with `npm run spec -- new change <name>`.
   Use `status` and `instructions <artifact> --change <name> --json` to obtain
   the schema, paths, templates, and dependencies. Write proposal, delta specs,
   design, and tasks BEFORE implementation. Include observable acceptance
   scenarios, edge/error cases, non-goals, and a verification plan.
3. Run `npm run spec -- validate <name> --strict --no-interactive`.
   If the user asked for implementation, continue after briefly presenting the
   plan. If they asked only for planning, stop for review.
4. Implement one small task at a time. For behavior changes, write a meaningful
   failing test first; run it and record the failure before fixing it. Tests
   assert public behavior, including failures, rather than internal structure.
   Update specs/design if discovery changes the agreed behavior; do not silently
   weaken acceptance criteria to pass tests. Mark only completed tasks.
5. Run `npm run verify`. Review the diff for spec coverage, regressions,
   unnecessary dependencies, and security-sensitive input handling. Record
   scenario-to-test mapping, exact commands, results, limitations, and any
   unrun checks in the change's `verification.md`. A checkbox is not evidence.
6. Report results with links. Leave the change active for review unless the user
   requested archiving. Archive verified changes with
   `npm run spec -- archive <name> --yes`, then run `npm run verify` again.
   Do not commit, push, publish, or contact others unless requested.

## Tools and conventions

- `npm ci` installs the pinned OpenSpec CLI. Use `npm run spec -- ...` instead
  of requiring a global CLI. Inside skill examples, bare `openspec` means the
  local CLI: prepend `node_modules/.bin` to PATH or use `npm run spec --`.
- `bash scripts/setup.sh` prepares Node dependencies and the pinned Rust toolchain.
- Rust is the default for new implementation examples; use the best language
  for a requested application. Keep dependencies small and explain additions.
- Rust gates: formatting, Clippy with warnings denied, locked tests (including
  doctests), and locked build. Node is tooling, not an application backend.
- No API key is needed by OpenSpec. The host AI (Codex/Claude Code) writes the
  artifacts and code; the CLI manages their structure and validation. Never
  describe structural spec validation as proof the behavior is correct.
- Preserve user work, never print secrets, never disable TLS/checksum checks,
  and report blocked or skipped checks honestly. Do not add artificial approval
  steps for routine local development.
