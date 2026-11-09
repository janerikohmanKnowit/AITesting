# AITesting

A spec-driven development harness for Codex and Claude Code, powered by
[OpenSpec](https://github.com/Fission-AI/OpenSpec). Describe a feature; the AI
writes a proposal, behavioral spec, design, and task list, then implements and
verifies it. Everything stays in the repository for review.

## Start here

Requires Node.js 22.12+ (Node 24 recommended), npm, curl, and a Rust-compatible
C compiler/linker. Setup installs the pinned Rust toolchain when needed.

```sh
bash scripts/setup.sh
npm run verify
```

No application server or API key is required. Use this repository in your
existing Codex or Claude Code session; OpenSpec itself is not an AI model or an
autonomous coding service. Restart your agent session after adding the repo so
it discovers the new skills/commands. Plain-language requests also work through
[AGENTS.md](AGENTS.md).

For a complete feature, tell the AI:

> Use deliver-feature to add a Rust command-line calculator that supports adding
> and subtracting integers. Write the spec and task plan first, then implement
> it with tests and run all verification checks. Record assumptions and results.

- **Codex:** select `deliver-feature` in Skills, or invoke `$deliver-feature`.
- **Claude Code:** `/deliver-feature <your feature description>`.
- **Planning only:** use `$openspec-propose` or `/opsx:propose`. Review the result,
  then request implementation with `$openspec-apply-change` or `/opsx:apply`.

The example prompt above is a future feature idea; the included implementation
is the small slug library described below.

## What gets written

```text
openspec/changes/<feature>/
  proposal.md                 Problem, scope, assumptions, non-goals
  specs/<capability>/spec.md  Requirements and WHEN/THEN acceptance scenarios
  design.md                   Technical decisions and verification strategy
  tasks.md                    Small, checkable implementation steps
  verification.md             Scenario coverage and actual command results
```

Completed changes stay active for your review. After verification, ask the AI
to archive the change, or run:

```sh
npm run spec -- archive <feature> --yes
npm run verify
```

Archiving merges delta specs into `openspec/specs/` and retains the change under
`openspec/changes/archive/`. It does not commit, push, or publish anything.

## Try the worked example

[`bootstrap-spec-harness`](openspec/changes/bootstrap-spec-harness/proposal.md)
shows the artifacts for this setup, including a dependency-free Rust library at
[`examples/rust-slug`](examples/rust-slug/src/lib.rs). Its tests cover case,
numbers, punctuation, empty input, and an explicit Unicode policy. It is a
learning example; future features do not have to be slug-related or use Rust.

```sh
npm run spec -- list
npm run spec -- status --change bootstrap-spec-harness
npm run verify
```

Verification runs strict OpenSpec validation, Rust formatting, Clippy with
warnings denied, integration tests and doctests, and a locked build. GitHub
Actions uses the same command. These checks support review; spec validation
alone checks document structure and cannot prove the AI interpreted your intent.

See [the workflow guide](docs/spec-driven-workflow.md) for manual commands,
resuming work, and toolchain details. OpenSpec is pinned in `package-lock.json`;
Rust is pinned in `rust-toolchain.toml`. No competing AI framework is needed.
