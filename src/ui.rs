use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::input::EguiWantsInput;

use crate::road::{ActiveTool, RoadPlacementState};

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

fn month_label(month: u32) -> &'static str {
    match month {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    }
}

/// Draw the HUD (top bar) and tool selection toolbar (left panel).
pub fn draw_ui(
    mut contexts: EguiContexts,
    game_time: Res<GameTime>,
    budget: Res<CityBudget>,
    mut active_tool: ResMut<ActiveTool>,
    mut placement: ResMut<RoadPlacementState>,
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

    Ok(())
}
