use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod building;
mod camera;
mod economy;
mod resources;
mod road;
mod terrain;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gilt and Iron".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .init_resource::<terrain::TerrainConfig>()
        .init_resource::<road::RoadNetwork>()
        .init_resource::<road::ActiveTool>()
        .init_resource::<road::RoadPlacementState>()
        .init_resource::<ui::GameTime>()
        .init_resource::<ui::CursorWorldPosition>()
        .init_resource::<ui::Notifications>()
        .init_resource::<economy::EconomyDebug>()
        .init_resource::<ui::DebugVisible>()
        .init_resource::<building::SpawnTimer>()
        .add_systems(Startup, (terrain::generate_heightmap, terrain::generate_biome_map, resources::generate_resource_map, terrain::spawn_terrain_mesh, terrain::spawn_water_plane, setup).chain())
        .add_systems(Update, (
            camera::camera_controls,
            ui::speed_controls,
            ui::update_cursor_position,
            ui::tick_notifications,
            road::toggle_road_tool,
            road::road_placement_input,
            road::generate_road_meshes,
            road::draw_road_debug,
            economy::evaluate_and_spawn,
            resources::draw_resource_debug,
            building::draw_lot_debug,
        ))
        .add_systems(EguiPrimaryContextPass, ui::draw_ui)
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
