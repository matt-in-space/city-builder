use bevy::prelude::*;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};

mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "City Builder".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FreeCameraPlugin)
        .init_resource::<terrain::TerrainConfig>()
        .add_systems(Startup, (terrain::generate_heightmap, terrain::spawn_terrain_mesh, setup).chain())
        .run();
}

fn setup(mut commands: Commands) {
    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            0.5,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));

    // Camera with free camera controls (WASD + mouse)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-50.0, 80.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
}
