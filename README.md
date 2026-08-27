# dbt-language-server
Build an LSP for dbt in Rust. Build something useful and learn Rust along the way.

## LSP Server Architecture

### Message Flow
1. Neovim sends LSP requests via stdin as JSON-RPC with headers
1. `lsp.rs::read_lsp_message` parses headers and extracts JSON
1. We parse the method and dispatch to appropriate handler
1. Handler (e.g., definition.rs) processes and returns response
1. Response sent back to Neovim via stdout

### LSP Message Format
See LSP specification: https://microsoft.github.io/language-server-protocol/
