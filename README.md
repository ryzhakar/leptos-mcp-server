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

Claude Code requires plugins to wrap MCP servers. Follow these steps:

### 1. Build the Server

```bash
git clone https://github.com/kneiht/leptos-mcp-server.git
cd leptos-mcp-server
cargo build --release
```

### 2. Create Plugin Structure

```bash
mkdir -p ~/.claude-plugins/leptos-marketplace/plugin/.claude-plugin
```

### 3. Create Plugin Manifest

Create `~/.claude-plugins/leptos-marketplace/plugin/.claude-plugin/plugin.json`:

```json
{
  "name": "leptos-mcp",
  "description": "Leptos documentation and code analysis tools via MCP",
  "author": {
    "name": "kneiht",
    "url": "https://github.com/kneiht/leptos-mcp-server"
  }
}
```

### 4. Create MCP Configuration

Create `~/.claude-plugins/leptos-marketplace/plugin/.mcp.json`:

```json
{
  "leptos": {
    "command": "/Users/YOUR_USERNAME/leptos-mcp-server/target/release/leptos-mcp-server"
  }
}
```

**Important:** Replace `/Users/YOUR_USERNAME/` with the actual path to your clone.

### 5. Create Marketplace Manifest

Create `~/.claude-plugins/leptos-marketplace/.claude-plugin/marketplace.json`:

```json
{
  "name": "local-leptos",
  "description": "Local marketplace for Leptos MCP server",
  "owner": {
    "name": "Local User"
  },
  "plugins": [
    {
      "name": "leptos-mcp",
      "description": "Leptos documentation and code analysis tools via MCP",
      "version": "1.0.0",
      "author": {
        "name": "kneiht",
        "url": "https://github.com/kneiht/leptos-mcp-server"
      },
      "source": "./plugin",
      "category": "development"
    }
  ]
}
```

### 6. Install Plugin

```bash
# Add the marketplace
claude plugin marketplace add ~/.claude-plugins/leptos-marketplace

# Install the plugin
claude plugin install leptos-mcp@local-leptos
```

### 7. Verify Installation

```bash
claude mcp list
# Should show: plugin:leptos-mcp:leptos: ... - ✓ Connected
```

### Tool Access

Tools will be available with the prefix `mcp__plugin_leptos-mcp_leptos__`:

- `mcp__plugin_leptos-mcp_leptos__list-sections`
- `mcp__plugin_leptos-mcp_leptos__get-documentation`
- `mcp__plugin_leptos-mcp_leptos__leptos-autofixer`

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
