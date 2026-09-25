use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::dashboard::DashboardState;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraFocusTarget>()
            .init_resource::<MouseDelta>()
            .add_systems(
                Update,
                (
                    update_mouse_delta_system,
                    camera_input_system,
                    camera_transform_system,
                    focus_on_selection_system,
                ),
            );
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    
    pub target_distance: f32,
    pub target_yaw: f32,
    pub target_pitch: f32,
    pub target_position: Vec3,
    
    pub sensitivity: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
    pub fly_speed: f32,
    pub smoothing: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    pub mode: CameraMode,
    pub is_flying: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 10.0,
            yaw: 0.0,
            pitch: 0.3,
            
            target_distance: 10.0,
            target_yaw: 0.0,
            target_pitch: 0.3,
            target_position: Vec3::ZERO,
            
            sensitivity: 0.005,
            zoom_speed: 0.1,
            pan_speed: 0.01,
            fly_speed: 5.0,
            smoothing: 0.15,
            min_distance: 0.5,
            max_distance: 1000.0,
            
            mode: CameraMode::Orbit,
            is_flying: false,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CameraMode {
    Orbit,
    Fly,
    Pan,
}

#[derive(Resource, Default)]
pub struct CameraFocusTarget {
    pub target: Option<Vec3>,
}

#[derive(Resource, Default)]
pub struct MouseDelta {
    pub delta: Vec2,
    pub scroll: f32,
}

fn update_mouse_delta_system(
    mut mouse_delta: ResMut<MouseDelta>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut scroll_events: MessageReader<MouseWheel>,
) {
    mouse_delta.delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        mouse_delta.delta += event.delta;
    }
    
    mouse_delta.scroll = 0.0;
    for event in scroll_events.read() {
        mouse_delta.scroll += event.y;
    }
}

fn camera_input_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_delta: Res<MouseDelta>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrbitCamera>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    
    let rmb_pressed = mouse_button.pressed(MouseButton::Right);
    let mmb_pressed = mouse_button.pressed(MouseButton::Middle);
    let alt_pressed = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
    let ctrl_pressed = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let shift_pressed = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    
    if rmb_pressed && !alt_pressed {
        camera.is_flying = true;
        camera.mode = CameraMode::Fly;
    } else if rmb_pressed && alt_pressed {
        camera.is_flying = false;
        camera.mode = CameraMode::Orbit;
    } else if mmb_pressed {
        camera.is_flying = false;
        camera.mode = CameraMode::Pan;
    } else if !rmb_pressed && !mmb_pressed {
        camera.is_flying = false;
        camera.mode = CameraMode::Orbit;
    }
    
    match camera.mode {
        CameraMode::Orbit => {
            if rmb_pressed || (alt_pressed && mouse_button.pressed(MouseButton::Left)) {
                camera.target_yaw -= mouse_delta.delta.x * camera.sensitivity;
                camera.target_pitch -= mouse_delta.delta.y * camera.sensitivity;
                camera.target_pitch = camera.target_pitch.clamp(
                    -89.0_f32.to_radians(),
                    89.0_f32.to_radians(),
                );
            }
            
            if mouse_delta.scroll != 0.0 {
                let zoom_amount = mouse_delta.scroll * camera.zoom_speed * camera.target_distance;
                camera.target_distance = (camera.target_distance - zoom_amount)
                    .clamp(camera.min_distance, camera.max_distance);
            }
            
            if ctrl_pressed && rmb_pressed {
                let zoom_amount = mouse_delta.delta.y * camera.zoom_speed * camera.target_distance * 0.01;
                camera.target_distance = (camera.target_distance - zoom_amount)
                    .clamp(camera.min_distance, camera.max_distance);
            }
        }
        
        CameraMode::Pan => {
            if mmb_pressed {
                let pan_multiplier = camera.distance * camera.pan_speed;
                
                let right = Vec3::new(camera.yaw.cos(), 0.0, -camera.yaw.sin());
                let up = Vec3::Y;
                
                camera.target_position -= right * mouse_delta.delta.x * pan_multiplier;
                camera.target_position += up * mouse_delta.delta.y * pan_multiplier;
            }
        }
        
        CameraMode::Fly => {
            if rmb_pressed {
                camera.target_yaw -= mouse_delta.delta.x * camera.sensitivity;
                camera.target_pitch -= mouse_delta.delta.y * camera.sensitivity;
                camera.target_pitch = camera.target_pitch.clamp(
                    -89.0_f32.to_radians(),
                    89.0_f32.to_radians(),
                );
            }
            
            let forward = Vec3::new(
                camera.yaw.sin() * camera.pitch.cos(),
                camera.pitch.sin(),
                -camera.yaw.cos() * camera.pitch.cos(),
            );
            let right = Vec3::new(
                camera.yaw.cos(),
                0.0,
                camera.yaw.sin(),
            );
            let up = Vec3::Y;
            
            let mut velocity = Vec3::ZERO;
            
            if keyboard.pressed(KeyCode::KeyW) { velocity += forward; }
            if keyboard.pressed(KeyCode::KeyS) { velocity -= forward; }
            if keyboard.pressed(KeyCode::KeyA) { velocity -= right; }
            if keyboard.pressed(KeyCode::KeyD) { velocity += right; }
            if keyboard.pressed(KeyCode::KeyE) || keyboard.pressed(KeyCode::Space) { velocity += up; }
            if keyboard.pressed(KeyCode::KeyQ) { velocity -= up; }
            
            if velocity.length_squared() > 0.0 {
                velocity = velocity.normalize();
                
                let mut speed = camera.fly_speed;
                if ctrl_pressed { speed *= 0.1; }
                if shift_pressed { speed *= 3.0; }
                
                camera.target_position += velocity * speed * time.delta_secs();
            }
        }
    }
}

fn camera_transform_system(mut camera_query: Query<(&mut OrbitCamera, &mut Transform)>) {
    let Ok((mut camera, mut transform)) = camera_query.single_mut() else {
        return;
    };
    
    let smoothing = camera.smoothing;
    
    camera.distance = camera.distance.lerp(camera.target_distance, smoothing);
    camera.yaw = camera.yaw.lerp(camera.target_yaw, smoothing);
    camera.pitch = camera.pitch.lerp(camera.target_pitch, smoothing);
    camera.target = camera.target.lerp(camera.target_position, smoothing);
    
    let x = camera.target.x + camera.distance * camera.pitch.cos() * camera.yaw.sin();
    let y = camera.target.y + camera.distance * camera.pitch.sin();
    let z = camera.target.z + camera.distance * camera.pitch.cos() * camera.yaw.cos();
    
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(camera.target, Vec3::Y);
}

fn focus_on_selection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    dashboard_state: Res<DashboardState>,
    transforms: Query<(&Name, &Transform)>,
    mut focus_target: ResMut<CameraFocusTarget>,
    mut camera_query: Query<&mut OrbitCamera>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        if let Some(selected_name) = &dashboard_state.selected_entity {
            for (name, transform) in transforms.iter() {
                if name.as_str() == selected_name {
                    focus_target.target = Some(transform.translation);
                    break;
                }
            }
        }
    }
    
    if let Some(target) = focus_target.target {
        if let Ok(mut camera) = camera_query.single_mut() {
            camera.target_position = target;
            focus_target.target = None;
        }
    }
}
