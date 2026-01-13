# Leptos MCP Server

An MCP (Model Context Protocol) server providing comprehensive Leptos documentation and code analysis tools for AI agents.

## Features

| Tool                | Description                                                     |
| ------------------- | --------------------------------------------------------------- |
| `list-sections`     | List all available Leptos documentation sections with use cases |
| `get-documentation` | Retrieve specific documentation content by section name         |
| `leptos-autofixer`  | Analyze Leptos code and suggest fixes for common issues         |

## Documentation Sections

| Section              | Topics                                                             |
| -------------------- | ------------------------------------------------------------------ |
| **Getting Started**  | Project setup, installation, hello world                           |
| **Components**       | `#[component]`, props, children                                    |
| **Signals**          | `get()`, `set()`, `read()`, `write()`, `update()`, derived signals |
| **Views**            | `view!` macro, dynamic classes/styles/attributes                   |
| **Resources**        | `Resource`, `LocalResource`, `OnceResource`, async data loading    |
| **Actions**          | `ServerAction`, `ActionForm`, mutations                            |
| **Server Functions** | `#[server]`, extractors, Axum integration                          |
| **Routing**          | Router, routes, params, nested routing                             |
| **Forms**            | Controlled inputs, `prop:value`, validation                        |
| **Error Handling**   | `ErrorBoundary`, `ServerFnError`                                   |
| **Suspense**         | `<Suspense>`, `<Transition>`, loading states                       |

## Installation

```bash
cd leptos-mcp-server
cargo build --release
```

## Usage with Claude Code CLI

Claude Code requires plugins to wrap MCP servers. This repository includes the necessary plugin manifests in the `claude-code-plugin/` directory.

### Installation Steps

1. **Clone and build:**

```bash
git clone https://github.com/kneiht/leptos-mcp-server.git
cd leptos-mcp-server
cargo build --release
```

2. **Configure the binary path:**

Edit `claude-code-plugin/plugin/.mcp.json` and update the path to point to your built binary:

```bash
# Example: Change this
/ABSOLUTE/PATH/TO/leptos-mcp-server/target/release/leptos-mcp-server

# To your actual path (use pwd to get current directory)
/Users/yourusername/leptos-mcp-server/target/release/leptos-mcp-server
```

3. **Add marketplace and install:**

```bash
# Add the marketplace
claude plugin marketplace add ./claude-code-plugin

# Install the plugin
claude plugin install leptos-mcp@leptos-mcp-server
```

4. **Verify installation:**

```bash
claude mcp list
# Should show: plugin:leptos-mcp:leptos: ... - ✓ Connected
```

### Available Tools

Tools are accessible with the `mcp__plugin_leptos-mcp_leptos__` prefix:

- `list-sections` - List all Leptos documentation sections
- `get-documentation` - Retrieve specific documentation by section
- `leptos-autofixer` - Analyze and suggest fixes for Leptos code

See `claude-code-plugin/README.md` for more details.

## Usage with Claude Desktop / Antigravity

Add to your MCP config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Antigravity:** `~/.gemini/antigravity/mcp_config.json`

```json
{
  "mcpServers": {
    "leptos": {
      "command": "/absolute/path/to/leptos-mcp-server/target/release/leptos-mcp-server"
    }
  }
}
```

## Testing

```bash
# Test tools/list
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/release/leptos-mcp-server 2>/dev/null

# Test list-sections
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list-sections","arguments":{}}}' | ./target/release/leptos-mcp-server 2>/dev/null

# Test get-documentation
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get-documentation","arguments":{"section":"signals"}}}' | ./target/release/leptos-mcp-server 2>/dev/null
```

## Development

```bash
# Run in development
cargo run

# Check for errors
cargo check

# Build release
cargo build --release
```

## Protocol

This server implements MCP over stdio using newline-delimited JSON-RPC 2.0.

## License

MIT
