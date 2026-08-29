use serde_json::{Value, from_str};
use std::io::BufRead;

/// Reads a single LSP message from stdin.
///
/// LSP uses JSON-RPC 2.0 with headers (standard across all clients: Neovim, VSCode, Emacs etc.):
/// ```text
/// Content-Length:`29\r\n
/// Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n
/// \r\n
/// {"jsonrpc":"2.0","id":1}
/// ```
///
/// This function:
/// 1. SKips all headers (reads until empty line)
/// 2. Reads the JSON body (next line)
/// 3. Returns the JSON string
///
/// Works with: Neovim (nvim-lspconfig), VSCode, Emacs (lsp-mode), Sublime etc.
/// `
fn read_lsp_message(reader: &mut dyn BufRead) -> Option<String> {
    // Read headers until empty line
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;

        if line.trim().is_empty() {
            break; // End of headers
        }
    }

    // Read the body (next line)
    let mut body = String::new();
    reader.read_line(&mut body).ok()?;
    Some(body)
}

pub fn parse_and_dispatch(json_str: &str) -> Option<String> {
    let json_value: Value = from_str(json_str).ok()?;
    let method_field = json_value.get("method")?.as_str()?;
    Some(method_field.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_read_lsp_message() {
        let input = "Content-Length: 29\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":1}";
        let mut cursor = Cursor::new(input);
        let message = read_lsp_message(&mut cursor);
        assert_eq!(message, Some(r#"{"jsonrpc":"2.0","id":1}"#.to_string()));
    }

    #[test]
    fn test_parse_and_dispatch_extract_method() {
        let json = r#"{"jsonrpc": "2.0", "method": "initialize", "id": 1}"#;
        let result = parse_and_dispatch(json);
        assert_eq!(result, Some("initialize".to_string()));
    }
}
