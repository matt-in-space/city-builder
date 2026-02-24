use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
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
    /// Y elevation of the water surface. Terrain below this appears submerged.
    pub water_level: f32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            map_size: 500.0,
            resolution: 256,
            height_scale: 30.0,
            water_level: 10.0,
        }
    }
}

/// Terrain biome classification for each grid cell.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    /// Submerged terrain near the shoreline
    Sand,
    /// Flat or gentle terrain at low-to-mid elevation
    Grass,
    /// Moderate slopes or mid-high elevation
    Dirt,
    /// Steep slopes or high elevation
    Rock,
}

impl Biome {
    /// Vertex color for this biome type.
    pub fn color(&self) -> [f32; 4] {
        match self {
            Biome::Sand  => [0.76, 0.70, 0.50, 1.0], // warm sandy tan
            Biome::Grass => [0.30, 0.50, 0.20, 1.0], // green
            Biome::Dirt  => [0.55, 0.40, 0.25, 1.0], // earthy brown
            Biome::Rock  => [0.50, 0.48, 0.45, 1.0], // gray stone
        }
    }
}

/// Per-cell biome classification for the terrain, same resolution as the heightmap.
#[derive(Resource)]
pub struct BiomeMap {
    pub biomes: Vec<Biome>,
    pub resolution: u32,
}

impl BiomeMap {
    pub fn get(&self, row: u32, col: u32) -> Biome {
        let row = row.min(self.resolution - 1);
        let col = col.min(self.resolution - 1);
        self.biomes[(row * self.resolution + col) as usize]
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

/// Classify each grid cell into a biome based on elevation, slope, and water level.
///
/// Rules:
/// - Below or just above water level → Sand (shoreline)
/// - Steep slope (normal Y < 0.85) → Rock (cliffs)
/// - High elevation (top 30%) → Rock
/// - Mid elevation or moderate slope → Dirt
/// - Everything else → Grass
pub fn generate_biome_map(
    mut commands: Commands,
    config: Res<TerrainConfig>,
    heightmap: Res<Heightmap>,
) {
    let res = heightmap.resolution;
    let cell_size = config.map_size / res as f32;
    let mut biomes = Vec::with_capacity((res * res) as usize);

    for row in 0..res {
        for col in 0..res {
            let height = heightmap.get(row, col);

            // Compute slope from neighbor heights (same as normal calculation)
            let h_left = if col > 0 { heightmap.get(row, col - 1) } else { height };
            let h_right = if col < res - 1 { heightmap.get(row, col + 1) } else { height };
            let h_down = if row > 0 { heightmap.get(row - 1, col) } else { height };
            let h_up = if row < res - 1 { heightmap.get(row + 1, col) } else { height };

            let normal = Vec3::new(
                h_left - h_right,
                2.0 * cell_size,
                h_down - h_up,
            )
            .normalize();

            // normal.y: 1.0 = flat, 0.0 = vertical cliff
            let flatness = normal.y;
            let elevation_t = height / config.height_scale;
            let shore_margin = 2.0; // world units above water = sand

            let biome = if height < config.water_level + shore_margin {
                Biome::Sand
            } else if flatness < 0.85 {
                Biome::Rock
            } else if elevation_t > 0.7 {
                Biome::Rock
            } else if flatness < 0.93 || elevation_t > 0.5 {
                Biome::Dirt
            } else {
                Biome::Grass
            };

            biomes.push(biome);
        }
    }

    commands.insert_resource(BiomeMap {
        biomes,
        resolution: res,
    });
}

/// Build a terrain mesh from the heightmap and spawn it into the world.
///
/// For each grid point, creates a vertex at (x, height, z). Connects
/// adjacent vertices into triangles (2 per grid cell). Computes per-vertex
/// normals from neighboring heights so lighting follows the terrain slope.
pub fn spawn_terrain_mesh(
    mut commands: Commands,
    config: Res<TerrainConfig>,
    heightmap: Res<Heightmap>,
    biome_map: Res<BiomeMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let res = heightmap.resolution;
    let cell_size = config.map_size / res as f32;
    // Center the terrain on the origin
    let half = config.map_size / 2.0;

    let vertex_count = (res * res) as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertex_count);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertex_count);

    // --- Vertices & colors ---
    for row in 0..res {
        for col in 0..res {
            let x = col as f32 * cell_size - half;
            let z = row as f32 * cell_size - half;
            let y = heightmap.get(row, col);

            positions.push([x, y, z]);
            uvs.push([col as f32 / (res - 1) as f32, row as f32 / (res - 1) as f32]);
            colors.push(biome_map.get(row, col).color());
        }
    }

    // --- Normals ---
    // For each vertex, approximate the surface normal by sampling the height
    // of its neighbors. The differences in height along X and Z give us two
    // tangent vectors; their cross product is the normal.
    for row in 0..res {
        for col in 0..res {
            let h_left = if col > 0 { heightmap.get(row, col - 1) } else { heightmap.get(row, col) };
            let h_right = if col < res - 1 { heightmap.get(row, col + 1) } else { heightmap.get(row, col) };
            let h_down = if row > 0 { heightmap.get(row - 1, col) } else { heightmap.get(row, col) };
            let h_up = if row < res - 1 { heightmap.get(row + 1, col) } else { heightmap.get(row, col) };

            // Tangent along X: (2*cell_size, h_right - h_left, 0)
            // Tangent along Z: (0, h_up - h_down, 2*cell_size)
            // Normal = cross(tangent_z, tangent_x) to get outward-facing Y-up
            let normal = Vec3::new(
                h_left - h_right,      // dx component
                2.0 * cell_size,       // y component (always positive = up)
                h_down - h_up,         // dz component
            )
            .normalize();

            normals.push(normal.into());
        }
    }

    // --- Indices ---
    // Two triangles per grid cell, winding counter-clockwise
    let cell_count = ((res - 1) * (res - 1)) as usize;
    let mut indices: Vec<u32> = Vec::with_capacity(cell_count * 6);

    for row in 0..(res - 1) {
        for col in 0..(res - 1) {
            let top_left = row * res + col;
            let top_right = top_left + 1;
            let bottom_left = (row + 1) * res + col;
            let bottom_right = bottom_left + 1;

            // Triangle 1: top-left, bottom-left, top-right
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);

            // Triangle 2: top-right, bottom-left, bottom-right
            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    // --- Build mesh ---
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(indices));

    // White base color so vertex colors drive the appearance
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
}

/// Spawn a flat blue plane at `water_level` to represent bodies of water.
///
/// Sized to cover the full terrain. Semi-transparent so you can see the
/// terrain underneath in shallow areas.
pub fn spawn_water_plane(
    mut commands: Commands,
    config: Res<TerrainConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(config.map_size, config.map_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.3, 0.5, 0.7),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(0.0, config.water_level, 0.0),
    ));
}
