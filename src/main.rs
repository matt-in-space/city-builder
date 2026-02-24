use bevy::prelude::*;

mod camera;
mod road;
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
        .init_resource::<terrain::TerrainConfig>()
        .init_resource::<road::RoadNetwork>()
        .init_resource::<road::ActiveTool>()
        .init_resource::<road::RoadPlacementState>()
        .add_systems(Startup, (terrain::generate_heightmap, terrain::generate_biome_map, terrain::spawn_terrain_mesh, terrain::spawn_water_plane, setup).chain())
        .add_systems(Update, (
            camera::camera_controls,
            road::toggle_road_tool,
            road::road_placement_input,
            road::draw_road_placement_preview,
        ))
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

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-50.0, 80.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera::CityCamera,
    ));
}
