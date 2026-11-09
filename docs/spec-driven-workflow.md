# Working from intent to verified code

The host assistant does the reasoning and editing. OpenSpec keeps the proposal,
behavior contract, design, and tasks organized. Repository instructions add
behavior-driven tests and verification evidence to that process.

## Modes

Use **deliver-feature** for a request to implement something. It writes the plan
before code and continues through implementation and checks. Reasonable choices
are recorded as assumptions. A material blocker still needs resolution.

Use **openspec-propose** for a planning-only session. This generated upstream
skill stops after planning. Review the documents, edit them if needed, and then
use **openspec-apply-change** for implementation. The deliver-feature skill uses
the OpenSpec CLI directly so it does not conflict with that planning boundary.

For either mode, read AGENTS.md. A test should fail for an incorrect behavior,
then pass after implementation; tests must not simply mirror internal code.
Map each scenario to an automated test or an explicit manual check. Record
failed, skipped, and unrun checks separately from passes in verification.md.

## CLI fallback

All commands below run from the repository root after setup. These CLI commands
create scaffolds or return instructions; **they do not call an AI or write your
feature for you**. The assistant authors the files using the returned templates.

```sh
npm run spec -- new change add-my-feature
npm run spec -- status --change add-my-feature --json
npm run spec -- instructions proposal --change add-my-feature --json
# Write proposal.md; then request and write specs and design, followed by tasks.
npm run spec -- instructions specs --change add-my-feature --json
npm run spec -- instructions design --change add-my-feature --json
npm run spec -- instructions tasks --change add-my-feature --json
npm run spec -- validate add-my-feature --strict --no-interactive
npm run spec -- instructions apply --change add-my-feature --json
# Implement the tasks, then:
npm run verify
```

Use `status` to discover readiness and artifact dependencies; it reports file
existence, not semantic completeness. Use `instructions apply` to resume an
existing change and keep task checkboxes accurate. Do not create a second change
for the same work just because a session ended. Main specs describe the accepted
system; active change specs describe proposed additions or modifications.

## Tool installation and updates

`bash scripts/setup.sh` runs frozen npm installation and installs Rust 1.91.1
with rustfmt and Clippy. Existing rustup installations are reused. Otherwise
Rust is installed under ignored `.tools/` without editing shell startup files.
The script preserves TLS verification. First-time installation needs access to
npm, sh.rustup.rs, and static.rust-lang.org; no credentials are required.

`npm run verify` automatically finds project-local Rust. For direct cargo use
with the local installation, run in a Bash session:

```sh
export CARGO_HOME="$PWD/.tools/cargo"
export RUSTUP_HOME="$PWD/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
cargo test --workspace --locked
```

If a cloud sandbox cannot write npm's default cache, use
`npm --cache "$PWD/.npm" ci`; setup already does this. Build outputs, tools,
and caches are ignored. Commit the Node and Rust lockfiles alongside source.
No service process needs to survive between cloud tasks, and no worktree is
needed: cloud tasks already have isolated checkouts.

To upgrade OpenSpec deliberately, change the dependency and lockfile together,
run its `update` command, and inspect generated agent instruction changes.
Keep custom delivery guidance in its separate skill. Re-run all checks.

The included CI workflow runs on pushes and pull requests; local validation does
not establish that a remote Actions run has executed. The assistant should leave
a final concise handoff listing changes, test evidence, and remaining limits.
