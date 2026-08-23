pub fn extract_quoted_string_at_position(line: &str, character: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    // Check if we are inside quotes by scanning left and right
    // 1. Find opening " before character: scan left from cursor until you hit "
    let start = chars[..=character].iter().rposition(|&c| c == '"')?;
    // 2. Find closing " after character: scan right from cursor until you hit "
    let end = chars[character..].iter().position(|&c| c == '"')? + character;
    // 3. Return what's between them: extract substring between start and end quotes, excluding quotes
    let result: String = chars[start + 1..end].iter().collect();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_quoted_string() {
        let line = r#"{{ ref("users") }}"#;
        assert_eq!(
            extract_quoted_string_at_position(line, 10),
            Some("users".to_string())
        );
    }
}
