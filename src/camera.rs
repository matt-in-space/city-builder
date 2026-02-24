use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

/// Marker component for the city builder camera.
#[derive(Component)]
pub struct CityCamera;

/// Camera controller system for a city builder.
///
/// - **WASD**: move on the ground plane (XZ), independent of camera pitch
/// - **Scroll wheel**: zoom in/out along the camera's look direction
/// - **Right-click drag**: rotate camera (yaw + pitch)
/// - **Speed**: scales with camera height so you cover more ground when zoomed out
pub fn camera_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<&mut Transform, With<CityCamera>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // Speed scales with height — higher up = faster panning
    let height = transform.translation.y.max(5.0);
    let move_speed = height * 1.2;
    let zoom_speed = height * 0.3;
    let rotate_speed = 0.003;

    // --- WASD: move on ground plane ---
    // Get the camera's forward/right projected onto XZ (ignore pitch)
    let forward = transform.forward();
    let ground_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let ground_right = Vec3::new(-forward.z, 0.0, forward.x).normalize_or_zero();

    let mut movement = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        movement += ground_forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement -= ground_forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement += ground_right;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement -= ground_right;
    }

    if movement.length_squared() > 0.0 {
        transform.translation += movement.normalize() * move_speed * dt;
    }

    // --- Scroll: zoom along look direction ---
    let scroll = mouse_scroll.delta.y;
    if scroll.abs() > 0.0 {
        let look_dir = transform.forward();
        transform.translation += *look_dir * scroll * zoom_speed;
        // Don't let the camera go below ground
        transform.translation.y = transform.translation.y.max(2.0);
    }

    // --- Right-click drag: rotate ---
    if mouse_buttons.pressed(MouseButton::Right) {
        let delta = mouse_motion.delta;

        if delta.x.abs() > 0.0 {
            // Yaw: rotate around world Y axis
            let pos = transform.translation;
            transform.rotate_around(pos, Quat::from_rotation_y(-delta.x * rotate_speed));
        }

        if delta.y.abs() > 0.0 {
            // Pitch: rotate around camera's local X axis
            let right = transform.right();
            let pitch = Quat::from_axis_angle(*right, -delta.y * rotate_speed);

            // Clamp pitch so we don't flip upside down
            let new_rotation = pitch * transform.rotation;
            let forward_after = new_rotation * Vec3::NEG_Z;
            let pitch_angle = forward_after.y.asin();

            // Allow looking between nearly straight down (-85°) and slightly above horizon (5°)
            if pitch_angle > -1.48 && pitch_angle < 0.09 {
                transform.rotation = new_rotation;
            }
        }
    }
}
