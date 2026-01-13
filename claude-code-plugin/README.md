# Claude Code Plugin for Leptos MCP Server

This directory contains the Claude Code plugin configuration for the Leptos MCP server.

## Installation

### 1. Build the MCP Server

From the repository root:

```bash
cargo build --release
```

### 2. Configure the Binary Path

Edit `plugin/.mcp.json` and replace `/ABSOLUTE/PATH/TO/` with the actual path to your cloned repository:

```json
{
  "leptos": {
    "command": "/Users/yourusername/leptos-mcp-server/target/release/leptos-mcp-server"
  }
}
```

### 3. Add Marketplace

```bash
claude plugin marketplace add /path/to/leptos-mcp-server/claude-code-plugin
```

### 4. Install Plugin

```bash
claude plugin install leptos-mcp@leptos-mcp-server
```

### 5. Verify

```bash
claude mcp list
# Should show: plugin:leptos-mcp:leptos: ... - ✓ Connected
```

## Available Tools

- `mcp__plugin_leptos-mcp_leptos__list-sections` - List documentation sections
- `mcp__plugin_leptos-mcp_leptos__get-documentation` - Get specific documentation
- `mcp__plugin_leptos-mcp_leptos__leptos-autofixer` - Analyze and fix Leptos code
