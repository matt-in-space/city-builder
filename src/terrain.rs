use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

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

/// A grid of elevation values generated from noise.
///
/// Stored as a flat `Vec<f32>` in row-major order (row * resolution + col).
/// Values range from 0.0 to `height_scale`.
#[derive(Resource)]
pub struct Heightmap {
    pub heights: Vec<f32>,
    pub resolution: u32,
}

impl Heightmap {
    /// Sample the height at a grid coordinate, clamping to bounds.
    pub fn get(&self, row: u32, col: u32) -> f32 {
        let row = row.min(self.resolution - 1);
        let col = col.min(self.resolution - 1);
        self.heights[(row * self.resolution + col) as usize]
    }
}

/// Generate a heightmap from the terrain config using fBm Perlin noise.
///
/// This runs once at startup. It creates an `Fbm<Perlin>` noise generator,
/// samples it at each grid point, remaps the [-1,1] noise output to [0,1],
/// and scales by `height_scale`.
pub fn generate_heightmap(mut commands: Commands, config: Res<TerrainConfig>) {
    let noise = Fbm::<Perlin>::new(42)
        .set_octaves(6)
        .set_frequency(2.0 / config.map_size as f64) // ~2 hills across the map
        .set_persistence(0.5);

    let res = config.resolution;
    let cell_size = config.map_size / res as f32;
    let mut heights = Vec::with_capacity((res * res) as usize);

    for row in 0..res {
        for col in 0..res {
            // Convert grid position to world-space coordinates
            let x = col as f64 * cell_size as f64;
            let z = row as f64 * cell_size as f64;

            // Sample noise (returns roughly -1.0 to 1.0), remap to 0.0–1.0
            let sample = noise.get([x, z]);
            let normalized = ((sample + 1.0) / 2.0).clamp(0.0, 1.0) as f32;

            heights.push(normalized * config.height_scale);
        }
    }

    commands.insert_resource(Heightmap {
        heights,
        resolution: res,
    });
}
