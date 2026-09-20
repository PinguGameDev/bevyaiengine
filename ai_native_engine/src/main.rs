mod entity_registry;
mod mcp_server;
mod scene;

use bevy::prelude::*;
use entity_registry::EntityRegistry;
use mcp_server::{McpChannel, McpPlugin};
use scene::{rotate_cube, setup_scene};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<EntityRegistry>()
        .add_plugins(McpPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, rotate_cube)
        .add_systems(Update, process_mcp_commands)
        .run();
}

fn process_mcp_commands(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut registry: ResMut<EntityRegistry>,
    mcp_channel: Option<Res<McpChannel>>,
    transforms: Query<&Transform>,
) {
    let Some(channel) = mcp_channel else { return };

    let mut rx = channel.command_rx.blocking_lock();

    while let Ok(cmd) = rx.try_recv() {
        let response = match cmd.command.as_str() {
            "create_entity" => {
                let name = cmd.args[0].clone();
                let entity = commands.spawn_empty().id();
                match registry.register(name.clone(), entity) {
                    Ok(_) => format!("Entity '{}' created", name),
                    Err(e) => e,
                }
            }
            "delete_entity" => {
                let name = &cmd.args[0];
                match registry.remove(name) {
                    Some(entity) => {
                        commands.entity(entity).despawn();
                        format!("Entity '{}' deleted", name)
                    }
                    None => format!("Entity '{}' not found", name),
                }
            }
            "set_transform" => {
                let name = &cmd.args[0];
                let x: f32 = cmd.args[1].parse().unwrap_or(0.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(0.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(0.0);
                match registry.get(name) {
                    Some(entity) => {
                        commands.entity(entity).insert(Transform::from_xyz(x, y, z));
                        format!("Entity '{}' moved to ({}, {}, {})", name, x, y, z)
                    }
                    None => format!("Entity '{}' not found", name),
                }
            }
            "rotate_entity" => {
                let name = &cmd.args[0];
                let x: f32 = cmd.args[1].parse().unwrap_or(0.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(0.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(0.0);
                match registry.get(name) {
                    Some(entity) => {
                        let current = transforms.get(entity).copied().unwrap_or_default();
                        let rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, x.to_radians(), y.to_radians(), z.to_radians());
                        let new_rotation = current.rotation * rotation;
                        commands.entity(entity).insert(Transform {
                            translation: current.translation,
                            rotation: new_rotation,
                            scale: current.scale,
                        });
                        format!("Entity '{}' rotated by ({}, {}, {}) degrees", name, x, y, z)
                    }
                    None => format!("Entity '{}' not found", name),
                }
            }
            "scale_entity" => {
                let name = &cmd.args[0];
                let x: f32 = cmd.args[1].parse().unwrap_or(1.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(1.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(1.0);
                match registry.get(name) {
                    Some(entity) => {
                        let current = transforms.get(entity).copied().unwrap_or_default();
                        let new_scale = current.scale * Vec3::new(x, y, z);
                        commands.entity(entity).insert(Transform {
                            translation: current.translation,
                            rotation: current.rotation,
                            scale: new_scale,
                        });
                        format!("Entity '{}' scaled by ({}, {}, {})", name, x, y, z)
                    }
                    None => format!("Entity '{}' not found", name),
                }
            }
            "create_cube" => {
                let name = cmd.args[0].clone();
                let x: f32 = cmd.args[1].parse().unwrap_or(0.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(0.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(0.0);
                let color = &cmd.args[4];

                let color_value = match color.to_lowercase().as_str() {
                    "red" => Color::srgb(0.8, 0.2, 0.2),
                    "green" => Color::srgb(0.2, 0.8, 0.2),
                    "blue" => Color::srgb(0.2, 0.2, 0.8),
                    "yellow" => Color::srgb(0.8, 0.8, 0.2),
                    "white" => Color::srgb(1.0, 1.0, 1.0),
                    _ => Color::srgb(0.5, 0.5, 0.5),
                };

                let entity = commands.spawn((
                    Mesh3d(meshes.add(Cuboid::default())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: color_value,
                        ..default()
                    })),
                    Transform::from_xyz(x, y, z),
                )).id();

                match registry.register(name.clone(), entity) {
                    Ok(_) => format!("Cube '{}' created at ({}, {}, {})", name, x, y, z),
                    Err(e) => e,
                }
            }
            "create_light" => {
                let name = cmd.args[0].clone();
                let x: f32 = cmd.args[1].parse().unwrap_or(0.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(0.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(0.0);
                let intensity: f32 = cmd.args[4].parse().unwrap_or(1_000_000.0);

                let entity = commands.spawn((
                    PointLight { intensity, ..default() },
                    Transform::from_xyz(x, y, z),
                )).id();

                match registry.register(name.clone(), entity) {
                    Ok(_) => format!("Light '{}' created at ({}, {}, {})", name, x, y, z),
                    Err(e) => e,
                }
            }
            "create_camera" => {
                let name = cmd.args[0].clone();
                let x: f32 = cmd.args[1].parse().unwrap_or(0.0);
                let y: f32 = cmd.args[2].parse().unwrap_or(0.0);
                let z: f32 = cmd.args[3].parse().unwrap_or(0.0);

                let entity = commands.spawn((
                    Camera3d::default(),
                    Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y),
                )).id();

                match registry.register(name.clone(), entity) {
                    Ok(_) => format!("Camera '{}' created at ({}, {}, {})", name, x, y, z),
                    Err(e) => e,
                }
            }
            "list_entities" => {
                let entities = registry.list();
                if entities.is_empty() {
                    "No entities in scene".to_string()
                } else {
                    format!("Entities: {}", entities.join(", "))
                }
            }
            "screenshot" => {
                "Screenshot functionality is not yet implemented. This feature will be added in a future update.".to_string()
            }
            _ => format!("Unknown command: {}", cmd.command),
        };

        let _ = cmd.response_tx.send(response);
    }
}
