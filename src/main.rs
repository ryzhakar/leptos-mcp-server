//! Leptos MCP Server
//!
//! A Model Context Protocol server that provides Leptos documentation
//! and code assistance tools for AI agents.
//!
//! Implements MCP protocol via JSON-RPC over stdio.

mod docs;
mod protocol;
mod tools;

use anyhow::Result;
use protocol::McpServer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging (to stderr for MCP compatibility)
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("leptos_mcp=info".parse()?))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting Leptos MCP Server...");

    // Create and run MCP server
    let server = McpServer::new();
    server.run().await?;

    Ok(())
}
