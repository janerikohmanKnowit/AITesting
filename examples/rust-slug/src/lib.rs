//! A deliberately small behavior contract used to demonstrate spec-driven work.

/// Convert ASCII letters and digits into a lowercase, hyphen-separated slug.
/// Every run of other characters is a separator; edge separators are omitted.
/// Return `None` when no ASCII letters or digits remain. No transliteration.
///
/// ```
/// assert_eq!(rust_slug::slugify("Hello, OpenSpec!"), Some("hello-openspec".into()));
/// assert_eq!(rust_slug::slugify("---"), None);
/// ```
pub fn slugify(input: &str) -> Option<String> {
    let mut result = String::new();
    let mut separator = false;
    for character in input.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !result.is_empty() {
                result.push('-');
            }
            result.push(character.to_ascii_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
