use rust_slug::slugify;

#[test]
fn normalizes_ascii_case_and_preserves_digits() {
    assert_eq!(slugify("OpenSpec 42"), Some("openspec-42".into()));
}

#[test]
fn collapses_mixed_separator_runs() {
    assert_eq!(slugify("hello, _\t--world"), Some("hello-world".into()));
}

#[test]
fn trims_edge_separators() {
    assert_eq!(slugify(" -- Hello! "), Some("hello".into()));
}

#[test]
fn rejects_inputs_without_ascii_letters_or_digits() {
    for input in ["", " \t\n", "---!", "你好", "🦀"] {
        assert_eq!(slugify(input), None, "input: {input:?}");
    }
}

#[test]
fn treats_non_ascii_as_separators_without_transliteration() {
    assert_eq!(slugify("café你好RUST"), Some("caf-rust".into()));
}

#[test]
fn preserves_an_existing_slug() {
    let slug = "already-a-slug-123";
    assert_eq!(slugify(slug).as_deref(), Some(slug));
}
