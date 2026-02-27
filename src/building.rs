use bevy::prelude::*;

use crate::economy::{BuildingCategory, BuildingDef, BUILDING_DEFS};
use crate::resources::ResourceMap;
use crate::road::{sample_catmull_rom, RoadNetwork, SegmentId};
use crate::terrain::{Heightmap, TerrainConfig};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// All buildings are gray cubes of this size.
const CUBE_HALF: Vec3 = Vec3::new(2.0, 1.5, 2.0); // 4x3x4

/// Distance from road centerline to lot center (setback + half lot depth).
const SETBACK: f32 = 3.5;

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct Building {
    pub def_index: usize,
    pub lot: Entity,
}

#[derive(Component)]
pub struct Lot {
    pub center: Vec2,
    pub rotation: f32,
    pub half_extents: Vec2,
    pub building: Entity,
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

#[derive(Resource)]
pub struct SpawnTimer {
    pub accumulator: f32,
    pub interval: f32,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            accumulator: 0.0,
            interval: 2.0,
        }
    }
}


// ---------------------------------------------------------------------------
// OBB overlap detection (2D, XZ plane)
// ---------------------------------------------------------------------------

fn lot_corners(center: Vec2, half_extents: Vec2, rotation: f32) -> [Vec2; 4] {
    let (sin, cos) = rotation.sin_cos();
    let ax = Vec2::new(cos, sin);
    let ay = Vec2::new(-sin, cos);
    let dx = ax * half_extents.x;
    let dy = ay * half_extents.y;
    [
        center - dx - dy,
        center + dx - dy,
        center + dx + dy,
        center - dx + dy,
    ]
}

fn project_corners(corners: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for &c in corners {
        let d = c.dot(axis);
        min = min.min(d);
        max = max.max(d);
    }
    (min, max)
}

fn obb_overlap(
    center_a: Vec2,
    half_a: Vec2,
    rot_a: f32,
    center_b: Vec2,
    half_b: Vec2,
    rot_b: f32,
) -> bool {
    let corners_a = lot_corners(center_a, half_a, rot_a);
    let corners_b = lot_corners(center_b, half_b, rot_b);

    let axes = [
        Vec2::new(rot_a.cos(), rot_a.sin()),
        Vec2::new(-rot_a.sin(), rot_a.cos()),
        Vec2::new(rot_b.cos(), rot_b.sin()),
        Vec2::new(-rot_b.sin(), rot_b.cos()),
    ];

    for axis in &axes {
        let (min_a, max_a) = project_corners(&corners_a, *axis);
        let (min_b, max_b) = project_corners(&corners_b, *axis);
        if max_a < min_b || max_b < min_a {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Lot validation
// ---------------------------------------------------------------------------

const ROAD_CLEARANCE: f32 = 1.5;

fn validate_placement(
    center: Vec2,
    half_extents: Vec2,
    rotation: f32,
    lots: &[(Vec2, Vec2, f32)],
    road_network: &RoadNetwork,
    heightmap: &Heightmap,
    config: &TerrainConfig,
) -> bool {
    // 1. Check against existing lots
    for &(lc, lh, lr) in lots {
        if obb_overlap(center, half_extents, rotation, lc, lh, lr) {
            return false;
        }
    }

    // 2. Check road corridor overlap
    let corners = lot_corners(center, half_extents, rotation);
    for segment in road_network.segments().values() {
        let Some(na) = road_network.node(segment.nodes[0]) else {
            continue;
        };
        let Some(nb) = road_network.node(segment.nodes[1]) else {
            continue;
        };
        let mut path = vec![na.position];
        path.extend_from_slice(&segment.control_points);
        path.push(nb.position);
        let samples = sample_catmull_rom(&path, 4);
        let half_width = segment.width / 2.0 + ROAD_CLEARANCE;

        for sample in &samples {
            let sp = Vec2::new(sample.x, sample.z);
            for &corner in &corners {
                if corner.distance(sp) < half_width {
                    return false;
                }
            }
            if point_in_obb(sp, center, half_extents, rotation) {
                return false;
            }
        }
    }

    // 3. Check water
    let corners = lot_corners(center, half_extents, rotation);
    for &corner in &corners {
        let h = heightmap.sample_world(corner.x, corner.y, config.map_size);
        if h < config.water_level {
            return false;
        }
    }

    // 4. Check steepness
    let mut min_h = f32::MAX;
    let mut max_h = f32::MIN;
    for &corner in &corners {
        let h = heightmap.sample_world(corner.x, corner.y, config.map_size);
        min_h = min_h.min(h);
        max_h = max_h.max(h);
    }
    if max_h - min_h > 3.0 {
        return false;
    }

    true
}

fn point_in_obb(point: Vec2, center: Vec2, half_extents: Vec2, rotation: f32) -> bool {
    let d = point - center;
    let (sin, cos) = rotation.sin_cos();
    let local_x = d.x * cos + d.y * sin;
    let local_y = -d.x * sin + d.y * cos;
    local_x.abs() <= half_extents.x && local_y.abs() <= half_extents.y
}

// ---------------------------------------------------------------------------
// Candidate finding
// ---------------------------------------------------------------------------

pub struct Candidate {
    pub position: Vec3,
    pub lot_center: Vec2,
    pub rotation: f32,
    pub _segment_id: SegmentId,
}

pub fn find_candidates(
    def: &BuildingDef,
    road_network: &RoadNetwork,
    heightmap: &Heightmap,
    config: &TerrainConfig,
    existing_lots: &[(Vec2, Vec2, f32)],
) -> Vec<Candidate> {
    let half_extents = def.lot_half_extents;
    let min_spacing = half_extents.x * 2.0;
    let mut candidates = Vec::new();

    for (&seg_id, segment) in road_network.segments() {
        let Some(na) = road_network.node(segment.nodes[0]) else {
            continue;
        };
        let Some(nb) = road_network.node(segment.nodes[1]) else {
            continue;
        };

        let mut path = vec![na.position];
        path.extend_from_slice(&segment.control_points);
        path.push(nb.position);

        let samples = sample_catmull_rom(&path, 4);
        if samples.len() < 2 {
            continue;
        }

        let mut last_left: Option<Vec2> = None;
        let mut last_right: Option<Vec2> = None;

        for i in 0..samples.len() - 1 {
            let p0 = samples[i];
            let p1 = samples[i + 1];
            let tangent = Vec2::new(p1.x - p0.x, p1.z - p0.z);
            if tangent.length_squared() < 0.01 {
                continue;
            }
            let tangent = tangent.normalize();
            let perp = Vec2::new(-tangent.y, tangent.x);
            let road_center = Vec2::new(p0.x, p0.z);

            let rotation = tangent.y.atan2(tangent.x);
            let offset = SETBACK + half_extents.y;

            for side in [-1.0_f32, 1.0] {
                let lot_center = road_center + perp * side * offset;
                let last = if side < 0.0 {
                    &mut last_left
                } else {
                    &mut last_right
                };

                if let Some(prev) = *last
                    && lot_center.distance(prev) < min_spacing
                {
                    continue;
                }

                let rot = if side < 0.0 {
                    rotation + std::f32::consts::PI
                } else {
                    rotation
                };

                if validate_placement(
                    lot_center,
                    half_extents,
                    rot,
                    existing_lots,
                    road_network,
                    heightmap,
                    config,
                ) {
                    let y = heightmap.sample_world(lot_center.x, lot_center.y, config.map_size);
                    candidates.push(Candidate {
                        position: Vec3::new(lot_center.x, y, lot_center.y),
                        lot_center,
                        rotation: rot,
                        _segment_id: seg_id,
                    });
                    *last = Some(lot_center);
                }
            }
        }
    }

    candidates
}

// ---------------------------------------------------------------------------
// Scoring
// ---------------------------------------------------------------------------

pub fn score_candidate(
    candidate: &Candidate,
    def: &BuildingDef,
    buildings: &[(Vec3, usize)],
    heightmap: &Heightmap,
    config: &TerrainConfig,
    resource_map: &ResourceMap,
) -> f32 {
    let pos = candidate.position;
    let pos2 = Vec2::new(pos.x, pos.z);
    let mut score: f32 = 0.0;

    // Terrain flatness (0-2)
    let flatness = heightmap.sample_flatness(pos.x, pos.z, config.map_size);
    score += flatness * 2.0;

    match def.category {
        BuildingCategory::Producer => {
            // Resource richness (0-8)
            if let Some(cell) = resource_map.sample_world(pos.x, pos.z, config.map_size)
                && def.extracts_from == Some(cell.resource)
            {
                score += cell.richness * 8.0;
            }

            // Penalty near residential
            let res_nearby = buildings
                .iter()
                .filter(|(bp, def_idx)| {
                    BUILDING_DEFS[*def_idx].category == BuildingCategory::Residential
                        && Vec2::new(bp.x, bp.z).distance(pos2) < 20.0
                })
                .count();
            score -= res_nearby as f32 * 2.0;
        }
        BuildingCategory::Residential => {
            // Proximity to producers (0-6)
            let producer_count = buildings
                .iter()
                .filter(|(bp, def_idx)| {
                    BUILDING_DEFS[*def_idx].category == BuildingCategory::Producer
                        && Vec2::new(bp.x, bp.z).distance(pos2) < 50.0
                })
                .count();
            score += (producer_count as f32 * 2.0).min(6.0);

            // Residential clustering (0-2)
            let res_nearby = buildings
                .iter()
                .filter(|(bp, def_idx)| {
                    BUILDING_DEFS[*def_idx].category == BuildingCategory::Residential
                        && Vec2::new(bp.x, bp.z).distance(pos2) < 20.0
                })
                .count();
            score += (res_nearby as f32 * 0.5).min(2.0);
        }
    }

    score
}

// ---------------------------------------------------------------------------
// Building spawning
// ---------------------------------------------------------------------------

pub fn spawn_building(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    candidate: &Candidate,
    def_index: usize,
    def: &BuildingDef,
) {
    let mesh_handle = meshes.add(Cuboid::new(
        CUBE_HALF.x * 2.0,
        CUBE_HALF.y * 2.0,
        CUBE_HALF.z * 2.0,
    ));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        perceptual_roughness: 0.9,
        ..default()
    });

    let transform = Transform::from_translation(
        candidate.position + Vec3::new(0.0, CUBE_HALF.y, 0.0),
    )
    .with_rotation(Quat::from_rotation_y(candidate.rotation));

    let lot_entity = commands.spawn_empty().id();
    let building_entity = commands
        .spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material),
            transform,
            Building {
                def_index,
                lot: lot_entity,
            },
        ))
        .id();

    commands.entity(lot_entity).insert(Lot {
        center: candidate.lot_center,
        rotation: candidate.rotation,
        half_extents: def.lot_half_extents,
        building: building_entity,
    });
}

// ---------------------------------------------------------------------------
// Debug visualization
// ---------------------------------------------------------------------------

pub fn draw_lot_debug(
    visible: Res<crate::ui::DebugVisible>,
    lots: Query<&Lot>,
    heightmap: Res<Heightmap>,
    config: Res<TerrainConfig>,
    mut gizmos: Gizmos,
) {
    if !visible.0 {
        return;
    }

    for lot in &lots {
        let corners = lot_corners(lot.center, lot.half_extents, lot.rotation);
        for i in 0..4 {
            let a = corners[i];
            let b = corners[(i + 1) % 4];
            let ya = heightmap.sample_world(a.x, a.y, config.map_size) + 0.3;
            let yb = heightmap.sample_world(b.x, b.y, config.map_size) + 0.3;
            gizmos.line(
                Vec3::new(a.x, ya, a.y),
                Vec3::new(b.x, yb, b.y),
                Color::WHITE,
            );
        }
    }
}
