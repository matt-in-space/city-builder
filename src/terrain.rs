use bevy::prelude::*;

/// Global configuration for terrain generation.
///
/// - `map_size`: side length of the terrain in world units (square map).
/// - `resolution`: number of vertices along each axis of the heightmap grid.
///   A 256x256 grid on a 500-unit map gives ~2 world units per cell.
/// - `height_scale`: maximum terrain elevation. Raw noise values (0.0–1.0)
///   are multiplied by this to produce world-space heights.
#[derive(Resource)]
pub struct TerrainConfig {
    pub map_size: f32,
    pub resolution: u32,
    pub height_scale: f32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            map_size: 500.0,
            resolution: 256,
            height_scale: 30.0,
        }
    }
}
