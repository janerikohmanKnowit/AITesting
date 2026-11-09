## Purpose

Provide a small deterministic slug transformation to demonstrate a complete path from acceptance scenarios to executable Rust tests.

## ADDED Requirements

### Requirement: Normalize ASCII words
The library SHALL lowercase ASCII letters, preserve digits, replace each run of other characters with one hyphen, and omit leading and trailing hyphens.

#### Scenario: Case and digits
- **WHEN** the input is `OpenSpec 42`
- **THEN** the result is `openspec-42`

#### Scenario: Mixed separator run
- **WHEN** the input is `hello, _<TAB>--world` with a literal tab replacing `<TAB>`
- **THEN** the result is `hello-world`

#### Scenario: Edge separators
- **WHEN** the input is ` -- Hello! `
- **THEN** the result is `hello`

#### Scenario: Existing slug
- **WHEN** the input is `already-a-slug-123`
- **THEN** the result is unchanged

### Requirement: Handle input without ASCII content
The library SHALL return no result when the input contains no ASCII letters or digits.

#### Scenario: Empty or separator-only input
- **WHEN** the input is empty, whitespace-only, punctuation-only, `你好`, or `🦀`
- **THEN** the result is absent

### Requirement: Explicit Unicode behavior
The library SHALL treat non-ASCII characters as separators without transliteration.

#### Scenario: Mixed Unicode and ASCII
- **WHEN** the input is `café你好RUST`
- **THEN** the result is `caf-rust`
