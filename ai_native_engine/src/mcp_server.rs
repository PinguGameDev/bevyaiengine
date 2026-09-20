use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct McpCommand {
    pub command: String,
    pub args: Vec<String>,
    pub response_tx: tokio::sync::oneshot::Sender<String>,
}

#[derive(Resource)]
pub struct McpChannel {
    pub command_rx: Arc<tokio::sync::Mutex<mpsc::Receiver<McpCommand>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Value,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: Value,
}

#[derive(Serialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
}

pub struct McpPlugin;

impl Plugin for McpPlugin {
    fn build(&self, app: &mut App) {
        let (command_tx, command_rx) = mpsc::channel(100);

        app.insert_resource(McpChannel {
            command_rx: Arc::new(tokio::sync::Mutex::new(command_rx)),
        });

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(e) = run_mcp_server(command_tx).await {
                    eprintln!("MCP server error: {}", e);
                }
            });
        });
    }
}

async fn run_mcp_server(command_tx: mpsc::Sender<McpCommand>) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    eprintln!("MCP server started on stdio");

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let request: Result<JsonRpcRequest, _> = serde_json::from_str(&line);
        
        let response = match request {
            Ok(req) => handle_request(req, &command_tx).await,
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32700,
                    message: format!("Parse error: {}", e),
                }),
                id: json!(null),
            },
        };

        let response_str = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", response_str)?;
        stdout.flush()?;
    }

    Ok(())
}

async fn handle_request(request: JsonRpcRequest, command_tx: &mpsc::Sender<McpCommand>) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "ai-native-engine",
                        "version": "0.1.0"
                    }
                })),
                error: None,
                id: request.id,
            }
        }
        "tools/list" => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({
                    "tools": [
                        {
                            "name": "create_entity",
                            "description": "Create a named empty entity at the origin",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"}
                                },
                                "required": ["name"]
                            }
                        },
                        {
                            "name": "delete_entity",
                            "description": "Delete an entity by name",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"}
                                },
                                "required": ["name"]
                            }
                        },
                        {
                            "name": "set_transform",
                            "description": "Move an entity to a new position",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"}
                                },
                                "required": ["name", "x", "y", "z"]
                            }
                        },
                        {
                            "name": "rotate_entity",
                            "description": "Rotate an entity (degrees)",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"}
                                },
                                "required": ["name", "x", "y", "z"]
                            }
                        },
                        {
                            "name": "scale_entity",
                            "description": "Scale an entity",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"}
                                },
                                "required": ["name", "x", "y", "z"]
                            }
                        },
                        {
                            "name": "create_cube",
                            "description": "Create a colored cube at a position",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"},
                                    "color": {"type": "string"}
                                },
                                "required": ["name", "x", "y", "z", "color"]
                            }
                        },
                        {
                            "name": "create_light",
                            "description": "Create a point light at a position",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"},
                                    "intensity": {"type": "number"}
                                },
                                "required": ["name", "x", "y", "z", "intensity"]
                            }
                        },
                        {
                            "name": "create_camera",
                            "description": "Create a camera at a position",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string"},
                                    "x": {"type": "number"},
                                    "y": {"type": "number"},
                                    "z": {"type": "number"}
                                },
                                "required": ["name", "x", "y", "z"]
                            }
                        },
                        {
                            "name": "list_entities",
                            "description": "List all entities in the scene",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "screenshot",
                            "description": "Capture the current rendered frame",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                })),
                error: None,
                id: request.id,
            }
        }
        "tools/call" => {
            handle_tool_call(request, command_tx).await
        }
        _ => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                }),
                id: request.id,
            }
        }
    }
}

async fn handle_tool_call(request: JsonRpcRequest, command_tx: &mpsc::Sender<McpCommand>) -> JsonRpcResponse {
    let params = request.params.unwrap_or(json!({}));
    
    let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

    let (command, args) = match tool_name {
        "create_entity" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            ("create_entity", vec![name])
        }
        "delete_entity" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            ("delete_entity", vec![name])
        }
        "set_transform" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            ("set_transform", vec![name, x, y, z])
        }
        "rotate_entity" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            ("rotate_entity", vec![name, x, y, z])
        }
        "scale_entity" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(1.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(1.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(1.0).to_string();
            ("scale_entity", vec![name, x, y, z])
        }
        "create_cube" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let color = arguments.get("color").and_then(|n| n.as_str()).unwrap_or("gray").to_string();
            ("create_cube", vec![name, x, y, z, color])
        }
        "create_light" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let intensity = arguments.get("intensity").and_then(|n| n.as_f64()).unwrap_or(1_000_000.0).to_string();
            ("create_light", vec![name, x, y, z, intensity])
        }
        "create_camera" => {
            let name = arguments.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
            let x = arguments.get("x").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let y = arguments.get("y").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            let z = arguments.get("z").and_then(|n| n.as_f64()).unwrap_or(0.0).to_string();
            ("create_camera", vec![name, x, y, z])
        }
        "list_entities" => ("list_entities", vec![]),
        "screenshot" => ("screenshot", vec![]),
        _ => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: format!("Unknown tool: {}", tool_name),
                }),
                id: request.id,
            };
        }
    };

    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
    let cmd = McpCommand {
        command: command.to_string(),
        args,
        response_tx,
    };

    if command_tx.send(cmd).await.is_err() {
        return JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32603,
                message: "Failed to send command to engine".to_string(),
            }),
            id: request.id,
        };
    }

    match response_rx.await {
        Ok(result) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "content": [
                    {
                        "type": "text",
                        "text": result
                    }
                ]
            })),
            error: None,
            id: request.id,
        },
        Err(_) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32603,
                message: "Failed to receive response from engine".to_string(),
            }),
            id: request.id,
        },
    }
}
