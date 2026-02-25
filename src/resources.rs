use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::terrain::{Biome, BiomeMap, Heightmap, TerrainConfig};

/// Map resource types that can be harvested by industries.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceType {
    Timber,
    FertileLand,
    Coal,
    Clay,
    Stone,
}

impl ResourceType {
    /// Color tint applied to terrain where this resource is present.
    pub fn color(&self) -> [f32; 4] {
        match self {
            ResourceType::Timber     => [0.10, 0.30, 0.05, 1.0], // deep forest green
            ResourceType::FertileLand => [0.22, 0.38, 0.10, 1.0], // rich agricultural green
            ResourceType::Coal       => [0.18, 0.16, 0.14, 1.0], // dark sooty
            ResourceType::Clay       => [0.52, 0.28, 0.18, 1.0], // reddish-brown
            ResourceType::Stone      => [0.62, 0.60, 0.58, 1.0], // pale gray
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ResourceType::Timber => "Timber",
            ResourceType::FertileLand => "Fertile Land",
            ResourceType::Coal => "Coal",
            ResourceType::Clay => "Clay",
            ResourceType::Stone => "Stone",
        }
    }
}

/// Per-cell resource data.
#[derive(Clone, Copy)]
pub struct ResourceCell {
    pub resource: ResourceType,
    /// Richness from 0.0 to 1.0 — how much of this resource is here.
    pub richness: f32,
}

/// Spatial grid of resources overlaid on the terrain, same resolution as heightmap.
#[derive(Resource)]
pub struct ResourceMap {
    pub cells: Vec<Option<ResourceCell>>,
    pub resolution: u32,
}

impl ResourceMap {
    pub fn get(&self, row: u32, col: u32) -> Option<ResourceCell> {
        let row = row.min(self.resolution - 1);
        let col = col.min(self.resolution - 1);
        self.cells[(row * self.resolution + col) as usize]
    }

    /// Sample the resource at a world-space (x, z) position.
    pub fn sample_world(&self, x: f32, z: f32, map_size: f32) -> Option<ResourceCell> {
        let half = map_size / 2.0;
        let cell_size = map_size / self.resolution as f32;
        let col = ((x + half) / cell_size).clamp(0.0, (self.resolution - 1) as f32) as u32;
        let row = ((z + half) / cell_size).clamp(0.0, (self.resolution - 1) as f32) as u32;
        self.get(row, col)
    }
}

/// Check if any cell within `radius` cells is below water level.
fn near_water(heightmap: &Heightmap, row: u32, col: u32, radius: u32, water_level: f32) -> bool {
    let res = heightmap.resolution;
    let r_start = row.saturating_sub(radius);
    let r_end = (row + radius + 1).min(res);
    let c_start = col.saturating_sub(radius);
    let c_end = (col + radius + 1).min(res);

    for r in r_start..r_end {
        for c in c_start..c_end {
            if heightmap.get(r, c) < water_level {
                return true;
            }
        }
    }
    false
}

/// Compute terrain flatness at a grid cell (same approach as biome generation).
fn cell_flatness(heightmap: &Heightmap, row: u32, col: u32, cell_size: f32) -> f32 {
    let res = heightmap.resolution;
    let height = heightmap.get(row, col);
    let h_left = if col > 0 { heightmap.get(row, col - 1) } else { height };
    let h_right = if col < res - 1 { heightmap.get(row, col + 1) } else { height };
    let h_down = if row > 0 { heightmap.get(row - 1, col) } else { height };
    let h_up = if row < res - 1 { heightmap.get(row + 1, col) } else { height };

    Vec3::new(h_left - h_right, 2.0 * cell_size, h_down - h_up)
        .normalize()
        .y
}

/// Generate resource deposits based on terrain features.
///
/// Runs once at startup after heightmap and biome map generation.
/// Each cell gets at most one resource type, prioritized by specificity:
/// Coal > Clay > Stone > Fertile Land > Timber.
pub fn generate_resource_map(
    mut commands: Commands,
    config: Res<TerrainConfig>,
    heightmap: Res<Heightmap>,
    biome_map: Res<BiomeMap>,
) {
    let res = heightmap.resolution;
    let cell_size = config.map_size / res as f32;

    // Separate noise generators per resource for uncorrelated clustering
    let timber_noise = Fbm::<Perlin>::new(100)
        .set_octaves(4)
        .set_frequency(4.0 / config.map_size as f64)
        .set_persistence(0.5);

    let coal_noise = Fbm::<Perlin>::new(200)
        .set_octaves(3)
        .set_frequency(8.0 / config.map_size as f64)
        .set_persistence(0.6);

    let clay_noise = Fbm::<Perlin>::new(300)
        .set_octaves(3)
        .set_frequency(6.0 / config.map_size as f64)
        .set_persistence(0.5);

    let stone_noise = Fbm::<Perlin>::new(400)
        .set_octaves(3)
        .set_frequency(6.0 / config.map_size as f64)
        .set_persistence(0.5);

    let fertility_noise = Fbm::<Perlin>::new(500)
        .set_octaves(3)
        .set_frequency(3.0 / config.map_size as f64)
        .set_persistence(0.5);

    let mut cells = Vec::with_capacity((res * res) as usize);

    for row in 0..res {
        for col in 0..res {
            let height = heightmap.get(row, col);
            let biome = biome_map.get(row, col);
            let elevation_t = height / config.height_scale;
            let flatness = cell_flatness(&heightmap, row, col, cell_size);

            let x = col as f64 * cell_size as f64;
            let z = row as f64 * cell_size as f64;

            let is_underwater = height < config.water_level;
            let is_near_water = near_water(&heightmap, row, col, 5, config.water_level);

            // Pre-sample all noise values
            let cn = coal_noise.get([x, z]);
            let cln = clay_noise.get([x, z]);
            let sn = stone_noise.get([x, z]);
            let fn_val = fertility_noise.get([x, z]);
            let tn = timber_noise.get([x, z]);

            let cell = if is_underwater {
                None
            }
            // Coal: clustered deposits in hilly/rocky terrain
            else if (biome == Biome::Dirt || biome == Biome::Rock)
                && elevation_t > 0.4
                && cn > 0.5
            {
                let richness = ((cn - 0.5) / 0.5).clamp(0.0, 1.0) as f32;
                Some(ResourceCell { resource: ResourceType::Coal, richness })
            }
            // Clay: near water, low elevation
            else if is_near_water
                && height > config.water_level
                && height < config.water_level + 4.0
                && cln > 0.3
            {
                let richness = ((cln - 0.3) / 0.7).clamp(0.0, 1.0) as f32;
                Some(ResourceCell { resource: ResourceType::Clay, richness })
            }
            // Stone: rocky, steep terrain
            else if biome == Biome::Rock && sn > 0.3 {
                let richness = ((sn - 0.3) / 0.7).clamp(0.0, 1.0) as f32;
                Some(ResourceCell { resource: ResourceType::Stone, richness })
            }
            // Fertile Land: very flat, low, near water
            else if flatness > 0.96
                && is_near_water
                && height > config.water_level
                && height < config.water_level + 6.0
                && fn_val > -0.2
            {
                let richness = ((fn_val + 0.2) / 1.2).clamp(0.3, 1.0) as f32;
                Some(ResourceCell { resource: ResourceType::FertileLand, richness })
            }
            // Timber: mid-elevation grassland/dirt, moderate slope
            else if (biome == Biome::Grass || biome == Biome::Dirt)
                && flatness > 0.88
                && height > config.water_level + 3.0
                && tn > 0.2
            {
                let richness = ((tn - 0.2) / 0.8).clamp(0.0, 1.0) as f32;
                Some(ResourceCell { resource: ResourceType::Timber, richness })
            }
            else {
                None
            };

            cells.push(cell);
        }
    }

    commands.insert_resource(ResourceMap { cells, resolution: res });
}
