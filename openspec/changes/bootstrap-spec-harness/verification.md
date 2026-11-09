# Verification evidence

Validated locally on 2026-11-09 with Node 24.22.1, npm 11.9.0,
OpenSpec 1.16.1, and Rust 1.91.1 on Linux.

## Commands and results

- `bash scripts/setup.sh`: passed; frozen npm installation, pinned Rust
  components, and locked build succeeded with the installed local toolchain.
- `npm run verify`: passed; one active OpenSpec change validated strictly,
  formatting passed, Clippy passed with warnings denied, six integration tests
  and one doctest passed, and the locked workspace build passed. The library
  unit-test target has zero tests; actual behavior is covered by integration
  tests and the doctest. No ignored or filtered behavior tests.
- `bash -n scripts/setup.sh`, `node --check scripts/verify.mjs`: passed.
- Wrong-implementation check: temporarily substituted an always-None stub and
  ran `cargo test --workspace --locked`. Five integration tests failed and one
  passed, exit 101. Restored the implementation and passed the full suite. This
  was a deliberate stub check, not a claim that the initial source was authored
  in a strict red/green sequence.
- Invalid-spec check: temporarily changed scenario heading levels, ran
  `npm run verify`, and observed exit 1 at strict validation before Cargo ran.
  Restored the spec afterward.
- Archive smoke check: copied openspec/ to a temporary directory, archived the
  completed bootstrap change there, and strictly validated the resulting two
  baseline specs. Both passed; the real checkout's change remains active.
- An initial verification-runner error (passing a URL to process.chdir) was
  fixed using fileURLToPath; the subsequent full run passed.

## Scenario coverage

| Capability / scenario | Evidence |
| --- | --- |
| Slug case and digits | `normalizes_ascii_case_and_preserves_digits` |
| Mixed separators | `collapses_mixed_separator_runs` |
| Edge separators | `trims_edge_separators` |
| Existing slug | `preserves_an_existing_slug` |
| No ASCII content | `rejects_inputs_without_ascii_letters_or_digits` (five inputs) |
| Mixed Unicode | `treats_non_ascii_as_separators_without_transliteration` |
| Public API usage | `slugify` doctest |
| End-to-end planning and evidence | Manual inspection: proposal, two delta specs, design, tasks, implementation, and this evidence file exist |
| Planning-only boundary | Manual inspection: generated upstream propose skill stops at planning; AGENTS.md routes planning-only requests there |
| Verification command failure | Invalid-spec injection returned 1 before Cargo; ordinary full run passed all gates |

## Limits

The agent routing and planning behavior are instructions, not mechanically
proven properties. A separate fresh Codex/Claude session has not been exercised.
GitHub Actions configuration is present but no remote Actions run was triggered.
The local environment has a working C linker; installing a linker on a machine
without one was not tested. Local Rust tools were installed from official HTTPS
sources through rustup with normal verification enabled. No cloud configuration
was changed or published, and no commits or pushes were made.
