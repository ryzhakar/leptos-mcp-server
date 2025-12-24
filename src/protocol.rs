//! MCP Protocol implementation
//!
//! JSON-RPC over stdio using newline-delimited JSON (NDJSON).

use crate::tools::LeptosTools;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, BufReader, Write};

/// MCP Server
pub struct McpServer {
    tools: LeptosTools,
}

/// JSON-RPC Request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC Response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            tools: LeptosTools::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        let mut stdout = io::stdout();

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    break;
                }
            };

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            // Parse JSON-RPC request
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Failed to parse request: {} - line: {}", e, line);
                    continue;
                }
            };

            // Notifications (no id) don't get a response per JSON-RPC spec
            if request.id.is_none() {
                // Just handle the notification silently
                self.handle_notification(&request.method);
                continue;
            }

            // Handle request and send response
            let response = self.handle_request(&request).await;
            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
        }

        Ok(())
    }

    fn handle_notification(&self, method: &str) {
        eprintln!("Received notification: {}", method);
        // Notifications don't require responses
    }

    async fn handle_request(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(Value::Null);

        eprintln!("Handling request: {}", request.method);

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(),
            "tools/list" => self.handle_list_tools(),
            "tools/call" => self.handle_call_tool(request.params.as_ref()),
            _ => {
                eprintln!("Unknown method: {}", request.method);
                Ok(json!({}))
            }
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(value),
                error: None,
            },
            Err(msg) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: msg,
                }),
            },
        }
    }

    fn handle_initialize(&self) -> Result<Value, String> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "leptos-mcp-server",
                "version": "0.1.0"
            }
        }))
    }

    fn handle_list_tools(&self) -> Result<Value, String> {
        Ok(json!({
            "tools": [
                {
                    "name": "list-sections",
                    "description": "List all available Leptos documentation sections with their use cases",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "get-documentation",
                    "description": "Get Leptos documentation for a specific section. Pass section name like 'signals', 'components', 'routing'",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "section": {
                                "type": "string",
                                "description": "Section name or path to retrieve"
                            }
                        },
                        "required": ["section"]
                    }
                },
                {
                    "name": "leptos-autofixer",
                    "description": "Analyze Leptos code and suggest fixes for common issues",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "code": {
                                "type": "string",
                                "description": "Leptos code to analyze"
                            }
                        },
                        "required": ["code"]
                    }
                }
            ]
        }))
    }

    fn handle_call_tool(&self, params: Option<&Value>) -> Result<Value, String> {
        let params = params.ok_or("Missing params")?;
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing tool name")?;
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let result = match name {
            "list-sections" => self.tools.list_sections(),
            "get-documentation" => {
                let section = arguments
                    .get("section")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.tools.get_documentation(section)
            }
            "leptos-autofixer" => {
                let code = arguments.get("code").and_then(|v| v.as_str()).unwrap_or("");
                self.tools.leptos_autofixer(code)
            }
            _ => return Err(format!("Unknown tool: {}", name)),
        };

        Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": result
                }
            ]
        }))
    }
}
