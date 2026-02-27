use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings};
use bevy::prelude::*;
use bevy_egui::input::EguiWantsInput;
use bevy_egui::{egui, EguiContexts};

use crate::building::Building;
use crate::camera::CityCamera;
use crate::economy::{BuildingCategory, EconomyDebug, BUILDING_DEFS};
use crate::resources::ResourceMap;
use crate::road::{ActiveTool, RoadNetwork, RoadPlacementState};
use crate::terrain::{Heightmap, TerrainConfig, TerrainMesh};

/// Game simulation speed levels.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum GameSpeed {
    Paused,
    #[default]
    Normal,
    Fast,
    VeryFast,
}

impl GameSpeed {
    pub fn multiplier(&self) -> f32 {
        match self {
            GameSpeed::Paused => 0.0,
            GameSpeed::Normal => 1.0,
            GameSpeed::Fast => 2.0,
            GameSpeed::VeryFast => 4.0,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            GameSpeed::Paused => "Paused",
            GameSpeed::Normal => "Normal",
            GameSpeed::Fast => "Fast",
            GameSpeed::VeryFast => "Very Fast",
        }
    }
}

#[derive(Resource, Default)]
pub struct GameTime {
    pub speed: GameSpeed,
}

/// Master debug toggle (F3): economy panel, road/lot/resource gizmos.
#[derive(Resource, Default)]
pub struct DebugVisible(pub bool);

/// Keyboard shortcuts for game speed: Space = pause toggle, 1/2/3 = speed levels.
pub fn speed_controls(
    keys: Res<ButtonInput<KeyCode>>,
    egui_input: Res<EguiWantsInput>,
    mut game_time: ResMut<GameTime>,
    mut economy_debug_visible: ResMut<DebugVisible>,
) {
    if egui_input.wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        game_time.speed = if game_time.speed == GameSpeed::Paused {
            GameSpeed::Normal
        } else {
            GameSpeed::Paused
        };
    }
    if keys.just_pressed(KeyCode::Digit1) {
        game_time.speed = GameSpeed::Normal;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        game_time.speed = GameSpeed::Fast;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        game_time.speed = GameSpeed::VeryFast;
    }
    if keys.just_pressed(KeyCode::F3) {
        economy_debug_visible.0 = !economy_debug_visible.0;
    }
}

/// World position under the mouse cursor, updated each frame via raycast.
#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Option<Vec3>,
}

/// Raycast from cursor onto terrain to track the world position under the mouse.
pub fn update_cursor_position(
    mut ray_cast: MeshRayCast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CityCamera>>,
    window: Query<&Window>,
    egui_input: Res<EguiWantsInput>,
    terrain_query: Query<(), With<TerrainMesh>>,
    mut cursor_pos: ResMut<CursorWorldPosition>,
) {
    cursor_pos.position = None;

    if egui_input.wants_any_pointer_input() {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(window) = window.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return;
    };

    let filter = |entity: Entity| terrain_query.contains(entity);
    let settings = MeshRayCastSettings::default().with_filter(&filter);

    let hits = ray_cast.cast_ray(ray, &settings);
    if let Some((_, hit)) = hits.first() {
        cursor_pos.position = Some(hit.point);
    }
}

/// A timed notification message.
pub struct Notification {
    pub message: String,
    pub timer: f32,
}

/// Queue of notification messages displayed to the player.
#[derive(Resource, Default)]
pub struct Notifications {
    pub messages: Vec<Notification>,
}

impl Notifications {
    pub fn push(&mut self, message: impl Into<String>, duration: f32) {
        self.messages.push(Notification {
            message: message.into(),
            timer: duration,
        });
    }
}

/// Tick down notification timers and remove expired ones.
pub fn tick_notifications(time: Res<Time>, mut notifications: ResMut<Notifications>) {
    let dt = time.delta_secs();
    for notif in &mut notifications.messages {
        notif.timer -= dt;
    }
    notifications.messages.retain(|n| n.timer > 0.0);
}

/// Draw the HUD, toolbar, info panel, and notifications.
#[allow(clippy::too_many_arguments)]
pub fn draw_ui(
    mut contexts: EguiContexts,
    game_time: Res<GameTime>,
    mut active_tool: ResMut<ActiveTool>,
    mut placement: ResMut<RoadPlacementState>,
    cursor_pos: Res<CursorWorldPosition>,
    heightmap: Res<Heightmap>,
    config: Res<TerrainConfig>,
    road_network: Res<RoadNetwork>,
    resource_map: Res<ResourceMap>,
    notifications: Res<Notifications>,
    buildings_query: Query<(&Building, &Transform)>,
    economy_debug: Res<EconomyDebug>,
    economy_debug_visible: Res<DebugVisible>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    // Count buildings by category
    let mut producer_count = 0u32;
    let mut residential_count = 0u32;
    let mut workers_needed = 0u32;
    let mut workers_provided = 0u32;

    for (b, _) in &buildings_query {
        let def = &BUILDING_DEFS[b.def_index];
        match def.category {
            BuildingCategory::Producer => {
                producer_count += 1;
                workers_needed += def.workers_required;
            }
            BuildingCategory::Residential => {
                residential_count += 1;
                workers_provided += def.workers_provided;
            }
        }
    }

    // --- Top bar ---
    egui::TopBottomPanel::top("hud").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Speed: {}", game_time.speed.label()));
            ui.separator();
            ui.label(format!("Producers: {}", producer_count));
            ui.separator();
            ui.label(format!("Residential: {}", residential_count));
            ui.separator();
            ui.label(format!("Workers: {}/{}", workers_provided, workers_needed));
        });
    });

    // --- Left toolbar ---
    egui::SidePanel::left("toolbar")
        .resizable(false)
        .exact_width(80.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Tools");
                ui.add_space(8.0);

                let tools: &[(ActiveTool, &str)] = &[
                    (ActiveTool::None, "Select"),
                    (ActiveTool::Road, "Road (R)"),
                    (ActiveTool::Zone, "Zone"),
                    (ActiveTool::Building, "Building"),
                ];

                for &(tool, label) in tools {
                    if ui
                        .selectable_label(*active_tool == tool, label)
                        .clicked()
                    {
                        *active_tool = tool;
                        placement.points.clear();
                    }
                }
            });
        });

    // --- Info panel (bottom-left) ---
    egui::Window::new("Info")
        .anchor(egui::Align2::LEFT_BOTTOM, [88.0, -4.0])
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(ctx, |ui| {
            if let Some(pos) = cursor_pos.position {
                let elevation = heightmap.sample_world(pos.x, pos.z, config.map_size);
                ui.label(format!("Position: ({:.0}, {:.0})", pos.x, pos.z));
                ui.label(format!("Elevation: {:.1}", elevation));

                if let Some(cell) = resource_map.sample_world(pos.x, pos.z, config.map_size) {
                    ui.label(format!(
                        "{} ({:.0}%)",
                        cell.resource.label(),
                        cell.richness * 100.0
                    ));
                }

                if let Some(node_id) = road_network.nearest_node(pos, 5.0) {
                    if let Some(node) = road_network.node(node_id) {
                        ui.separator();
                        ui.label(format!("Road node ({} connections)", node.segments.len()));
                    }
                }

                // Nearby building info
                let mut nearest_building: Option<(&Building, f32)> = None;
                for (b, t) in &buildings_query {
                    let dist = t.translation.distance(pos);
                    if dist < 10.0
                        && (nearest_building.is_none() || dist < nearest_building.unwrap().1)
                    {
                        nearest_building = Some((b, dist));
                    }
                }
                if let Some((b, _)) = nearest_building {
                    let def = &BUILDING_DEFS[b.def_index];
                    ui.separator();
                    ui.label(def.label);
                    ui.label(format!("Category: {:?}", def.category));
                    if def.workers_required > 0 {
                        ui.label(format!("Workers needed: {}", def.workers_required));
                    }
                    if def.workers_provided > 0 {
                        ui.label(format!("Workers provided: {}", def.workers_provided));
                    }
                }
            } else {
                ui.label("--");
            }
        });

    // --- Economy debug panel (F3) ---
    if economy_debug_visible.0 {
        egui::TopBottomPanel::bottom("economy_debug").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.strong("Economy Debug");
                ui.separator();

                let deficit = economy_debug
                    .workers_needed
                    .saturating_sub(economy_debug.workers_provided);
                ui.label(format!(
                    "Workers: {}/{} (need {} more)",
                    economy_debug.workers_provided, economy_debug.workers_needed, deficit,
                ));
                ui.separator();

                let producer_status = if economy_debug.producer_viable {
                    format!(
                        "viable ({} candidates{})",
                        economy_debug.producer_candidates,
                        match economy_debug.best_score {
                            Some(s) => format!(", best score {:.1}", s),
                            None => String::new(),
                        },
                    )
                } else {
                    format!("not viable ({})", economy_debug.producer_reason)
                };
                ui.label(format!("Producer: {}", producer_status));
                ui.separator();

                let residential_status = if economy_debug.residential_viable {
                    format!("viable ({} candidates)", economy_debug.residential_candidates)
                } else {
                    format!("not viable ({})", economy_debug.residential_reason)
                };
                ui.label(format!("Residential: {}", residential_status));
                ui.separator();

                match &economy_debug.last_spawn {
                    Some(s) => ui.label(format!("Last: {}", s)),
                    None => ui.label("Last: --"),
                };
            });
        });
    }

    // --- Notifications (bottom-right) ---
    if !notifications.messages.is_empty() {
        egui::Window::new("Notifications")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-4.0, -4.0])
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                for notif in &notifications.messages {
                    ui.label(&notif.message);
                }
            });
    }

    Ok(())
}
