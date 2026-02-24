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

/// Build a terrain mesh from the heightmap and spawn it into the world.
///
/// For each grid point, creates a vertex at (x, height, z). Connects
/// adjacent vertices into triangles (2 per grid cell). Computes per-vertex
/// normals from neighboring heights so lighting follows the terrain slope.
pub fn spawn_terrain_mesh(
    mut commands: Commands,
    config: Res<TerrainConfig>,
    heightmap: Res<Heightmap>,
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

            // Color by elevation: green (low) → brown (mid) → gray (high)
            let t = (y / config.height_scale).clamp(0.0, 1.0);
            let color = if t < 0.5 {
                // Green to brown
                let s = t / 0.5;
                [
                    0.2 + s * 0.35,  // 0.20 → 0.55
                    0.45 - s * 0.15, // 0.45 → 0.30
                    0.15,            // low blue
                    1.0,
                ]
            } else {
                // Brown to gray
                let s = (t - 0.5) / 0.5;
                [
                    0.55 - s * 0.1, // 0.55 → 0.45
                    0.30 + s * 0.1, // 0.30 → 0.40
                    0.15 + s * 0.2, // 0.15 → 0.35
                    1.0,
                ]
            };
            colors.push(color);
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
