use lsp_types::{Location, Position, Range, Uri};
use std::path::PathBuf;
use walkdir::WalkDir;

// Find models/{filename} recursively
pub fn find_model_file(models_dir: &str, filename: &str) -> Option<PathBuf> {
    for entry in WalkDir::new(models_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == filename {
            return Some(entry.path().to_path_buf());
        }
    }
    None
}

// Main handler for the definition command
pub fn handle_definition(content: &str, line: u32, character: u32) -> Option<Location> {
    let line_str = content.lines().nth(line as usize)?;
    let ref_name = crate::parser::extract_quoted_string_at_position(line_str, character as usize)?;
    let model_path = find_model_file("models", &format!("{}.sql", ref_name));

    let absolute_path = model_path.unwrap().canonicalize().ok()?;
    // let absolute_path = model_path.unwrap().canonicalize().unwrap();
    println!("Absolute path: {}", absolute_path.display());
    let uri_string = format!("file://{}", absolute_path.display());
    println!("uri string: {}", uri_string);
    let uri = uri_string.parse::<Uri>().unwrap();
    Some(Location {
        uri,
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_definition_intermediate_steps() {
        let content =
            r#"{{ config(materialized="table") }} select id, name, email from {{ ref("users") }}"#;
        let line = 0u32;
        // let character = 80u32;
        // get "users" position
        let character = 75u32;
        println!("Character position: {}", character);
        // Get line
        let line_str = content.lines().nth(line as usize).unwrap();
        println!("Line str: {}", line_str);
        // Extract ref name
        let ref_name =
            crate::parser::extract_quoted_string_at_position(line_str, character as usize).unwrap();
        println!("ref name: {}", ref_name);
        let model_path = find_model_file("models", &format!("{}.sql", ref_name));
        println!("model path: {:?}", model_path);
        let absolute_path = model_path.unwrap().canonicalize().ok().unwrap();
        println!("Absolute path: {}", absolute_path.display());
        let uri_string = format!("file://{}", absolute_path.display());
        println!("uri: {}", uri_string);
    }

    #[test]
    fn test_handle_definition() {
        let content =
            r#"{{ config(materialized="table") }} select id, name, email from {{ ref("users") }}"#;
        let line = 0u32;
        let position_of_users_quote = content.find(r#"ref("users")"#).unwrap() + 5;
        let character = position_of_users_quote as u32;
        let location = handle_definition(content, line, character);
        assert!(location.is_some());
        let loc = location.unwrap();
        assert!(loc.uri.to_string().contains("users.sql"));
    }
}
