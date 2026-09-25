use bevy::prelude::*;
use crate::entity_registry::EntityRegistry;
use crate::camera_controller::OrbitCamera;
use crate::combat::Health;
use crate::gameplay_tags::{GameplayTags, TagRegistry, tags};

#[derive(Component)]
pub struct StarterCube;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut registry: ResMut<EntityRegistry>,
    tag_registry: Res<TagRegistry>,
) {
    let camera = commands.spawn((
        Name::new("starter_camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera {
            target: Vec3::ZERO,
            distance: 10.0,
            yaw: 0.0,
            pitch: 0.25,
            target_distance: 10.0,
            target_yaw: 0.0,
            target_pitch: 0.25,
            target_position: Vec3::ZERO,
            ..default()
        },
    )).id();
    let _ = registry.register("starter_camera".to_string(), camera);

    let light = commands.spawn((
        Name::new("starter_light"),
        PointLight {
            intensity: 1_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    )).id();
    let _ = registry.register("starter_light".to_string(), light);

    let bright_overhead_light = commands.spawn((
        Name::new("bright_overhead_light"),
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    )).id();
    let _ = registry.register("bright_overhead_light".to_string(), bright_overhead_light);

    let cube = commands.spawn((
        Name::new("starter_cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        StarterCube,
    )).id();
    let _ = registry.register("starter_cube".to_string(), cube);

    let colored_cubes = [
        ("blue_cube", Color::srgb(0.2, 0.2, 0.8), Vec3::new(-2.0, 0.5, 1.0)),
        ("green_cube", Color::srgb(0.2, 0.8, 0.2), Vec3::new(2.0, 0.5, 1.0)),
        ("yellow_cube", Color::srgb(0.8, 0.8, 0.2), Vec3::new(0.0, 0.5, -2.0)),
        ("purple_cube", Color::srgb(0.8, 0.2, 0.8), Vec3::new(-1.5, 0.5, -1.5)),
    ];
    for (name, color, pos) in colored_cubes {
        let entity = commands.spawn((
            Name::new(name),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(pos),
        )).id();
        let _ = registry.register(name.to_string(), entity);
    }

    // Spawn ARPG test entities with health and faction tags.
    let player_id = tag_registry.get_id(tags::FACTION_PLAYER).unwrap_or(0);
    let enemy_id = tag_registry.get_id(tags::FACTION_ENEMY).unwrap_or(0);

    let player = commands.spawn((
        Name::new("player"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 1.0),
            ..default()
        })),
        Transform::from_xyz(-1.5, 0.5, 0.0),
        Health::new(100.0),
        GameplayTags::from_ids(&[player_id]),
    )).id();
    let _ = registry.register("player".to_string(), player);

    let enemy = commands.spawn((
        Name::new("enemy"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.1),
            ..default()
        })),
        Transform::from_xyz(1.5, 0.5, 0.0),
        Health::new(50.0),
        GameplayTags::from_ids(&[enemy_id]),
    )).id();
    let _ = registry.register("enemy".to_string(), enemy);

    info!("AI-Native Engine starter scene loaded.");
}

pub fn rotate_cube(mut query: Query<&mut Transform, With<StarterCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}
