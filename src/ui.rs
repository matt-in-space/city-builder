use bevy::prelude::*;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::input::EguiWantsInput;

use crate::camera::CityCamera;
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
            GameSpeed::Paused   => 0.0,
            GameSpeed::Normal   => 1.0,
            GameSpeed::Fast     => 2.0,
            GameSpeed::VeryFast => 4.0,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            GameSpeed::Paused   => "Paused",
            GameSpeed::Normal   => "Normal",
            GameSpeed::Fast     => "Fast",
            GameSpeed::VeryFast => "Very Fast",
        }
    }
}

/// Tracks in-game date and simulation speed.
#[derive(Resource)]
pub struct GameTime {
    pub speed: GameSpeed,
    pub month: u32,
    pub year: u32,
    /// Accumulated game-time seconds within the current month.
    pub month_progress: f32,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            speed: GameSpeed::Normal,
            month: 1,
            year: 1920,
            month_progress: 0.0,
        }
    }
}

/// Real seconds per game month at 1x speed.
const SECONDS_PER_MONTH: f32 = 10.0;

/// Placeholder city budget.
#[derive(Resource)]
pub struct CityBudget {
    pub funds: f64,
}

impl Default for CityBudget {
    fn default() -> Self {
        Self { funds: 10_000.0 }
    }
}

/// Advance game date based on current speed.
pub fn advance_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    let dt = time.delta_secs() * game_time.speed.multiplier();
    game_time.month_progress += dt;

    while game_time.month_progress >= SECONDS_PER_MONTH {
        game_time.month_progress -= SECONDS_PER_MONTH;
        game_time.month += 1;
        if game_time.month > 12 {
            game_time.month = 1;
            game_time.year += 1;
        }
    }
}

/// Keyboard shortcuts for game speed: Space = pause toggle, 1/2/3 = speed levels.
pub fn speed_controls(
    keys: Res<ButtonInput<KeyCode>>,
    egui_input: Res<EguiWantsInput>,
    mut game_time: ResMut<GameTime>,
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

    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    let Ok(window) = window.single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else { return };

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

fn month_label(month: u32) -> &'static str {
    match month {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    }
}

/// Draw the HUD, toolbar, info panel, and notifications.
pub fn draw_ui(
    mut contexts: EguiContexts,
    game_time: Res<GameTime>,
    budget: Res<CityBudget>,
    mut active_tool: ResMut<ActiveTool>,
    mut placement: ResMut<RoadPlacementState>,
    cursor_pos: Res<CursorWorldPosition>,
    heightmap: Res<Heightmap>,
    config: Res<TerrainConfig>,
    road_network: Res<RoadNetwork>,
    notifications: Res<Notifications>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    // --- Top bar: date, speed, funds, population ---
    egui::TopBottomPanel::top("hud").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("{} {}", month_label(game_time.month), game_time.year));
            ui.separator();
            ui.label(format!("Speed: {}", game_time.speed.label()));
            ui.separator();
            ui.label(format!("${:.0}", budget.funds));
            ui.separator();
            ui.label("Pop: 0");
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
                    if ui.selectable_label(*active_tool == tool, label).clicked() {
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

                // Show nearby road node info
                if let Some(node_id) = road_network.nearest_node(pos, 5.0) {
                    if let Some(node) = road_network.node(node_id) {
                        ui.separator();
                        ui.label(format!("Road node ({} connections)", node.segments.len()));
                    }
                }
            } else {
                ui.label("--");
            }
        });

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
