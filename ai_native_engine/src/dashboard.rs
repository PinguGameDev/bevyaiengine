use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use std::collections::VecDeque;

use crate::entity_registry::EntityRegistry;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }
        
        app.init_resource::<DashboardState>()
            .init_resource::<McpMessageLog>()
            .init_resource::<SystemLog>()
            .add_systems(EguiPrimaryContextPass, dashboard_ui_system);
    }
}

#[derive(Resource, Default)]
pub struct DashboardState {
    pub selected_entity: Option<String>,
    pub show_entity_list: bool,
    pub show_property_inspector: bool,
    pub show_system_log: bool,
    pub show_mcp_log: bool,
}

#[derive(Resource)]
pub struct McpMessageLog {
    messages: VecDeque<McpMessage>,
    max_messages: usize,
}

impl Default for McpMessageLog {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 100,
        }
    }
}

impl McpMessageLog {
    pub fn push(&mut self, message: McpMessage) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &McpMessage> {
        self.messages.iter()
    }
}

#[derive(Clone)]
pub struct McpMessage {
    pub timestamp: f64,
    pub direction: McpDirection,
    pub method: String,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub enum McpDirection {
    Incoming,
    Outgoing,
}

#[derive(Resource)]
pub struct SystemLog {
    messages: VecDeque<LogMessage>,
    max_messages: usize,
}

impl Default for SystemLog {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 200,
        }
    }
}

impl SystemLog {
    pub fn push(&mut self, level: LogLevel, message: String) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(LogMessage {
            timestamp: 0.0, // Will be set by system
            level,
            message,
        });
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &LogMessage> {
        self.messages.iter()
    }
}

#[derive(Clone)]
pub struct LogMessage {
    pub timestamp: f64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

fn dashboard_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<DashboardState>,
    registry: Res<EntityRegistry>,
    mcp_log: Res<McpMessageLog>,
    system_log: Res<SystemLog>,
    time: Res<Time>,
    transforms: Query<(&Name, &Transform)>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    
    // Top menu bar
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut state.show_entity_list, "Entity List");
                ui.checkbox(&mut state.show_property_inspector, "Property Inspector");
                ui.checkbox(&mut state.show_system_log, "System Log");
                ui.checkbox(&mut state.show_mcp_log, "MCP Log");
            });
        });
    });
    
    // Left panel - Entity List
    if state.show_entity_list {
        egui::SidePanel::left("entity_list")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Entities");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (name, _entity) in registry.iter() {
                        let is_selected = state.selected_entity.as_ref() == Some(name);
                        if ui.selectable_label(is_selected, name).clicked() {
                            state.selected_entity = Some(name.clone());
                        }
                    }
                });
            });
    }
    
    // Right panel - Property Inspector
    if state.show_property_inspector {
        egui::SidePanel::right("property_inspector")
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();
                
                if let Some(selected_name) = &state.selected_entity {
                    ui.label(format!("Selected: {}", selected_name));
                    ui.separator();
                    
                    // Find the entity's transform
                    for (name, transform) in transforms.iter() {
                        if name.as_str() == selected_name {
                            ui.label("Transform:");
                            ui.indent("transform", |ui| {
                                ui.label(format!("Position: ({:.2}, {:.2}, {:.2})", 
                                    transform.translation.x, 
                                    transform.translation.y, 
                                    transform.translation.z));
                                
                                let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
                                ui.label(format!("Rotation: ({:.2}, {:.2}, {:.2})", 
                                    yaw.to_degrees(), 
                                    pitch.to_degrees(), 
                                    roll.to_degrees()));
                                
                                ui.label(format!("Scale: ({:.2}, {:.2}, {:.2})", 
                                    transform.scale.x, 
                                    transform.scale.y, 
                                    transform.scale.z));
                            });
                            break;
                        }
                    }
                } else {
                    ui.label("No entity selected");
                }
            });
    }
    
    // Bottom panel - System Log
    if state.show_system_log {
        egui::TopBottomPanel::bottom("system_log")
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.heading("System Log");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for msg in system_log.iter() {
                        let color = match msg.level {
                            LogLevel::Info => egui::Color32::WHITE,
                            LogLevel::Warn => egui::Color32::YELLOW,
                            LogLevel::Error => egui::Color32::RED,
                        };
                        ui.colored_label(color, &msg.message);
                    }
                });
            });
    }
    
    // Bottom panel - MCP Log
    if state.show_mcp_log {
        egui::TopBottomPanel::bottom("mcp_log")
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.heading("MCP Log");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for msg in mcp_log.iter() {
                        let color = match msg.direction {
                            McpDirection::Incoming => egui::Color32::LIGHT_GREEN,
                            McpDirection::Outgoing => egui::Color32::LIGHT_BLUE,
                        };
                        let direction_str = match msg.direction {
                            McpDirection::Incoming => "→",
                            McpDirection::Outgoing => "←",
                        };
                        ui.colored_label(
                            color,
                            format!("{} {} - {}", direction_str, msg.method, msg.content)
                        );
                    }
                });
            });
    }
    
    Ok(())
}
