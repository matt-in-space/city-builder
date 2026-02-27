use bevy::prelude::*;

use crate::building::{find_candidates, score_candidate, spawn_building, Building, Lot, SpawnTimer};
use crate::resources::{ResourceMap, ResourceType};
use crate::road::{sample_catmull_rom, RoadNetwork};
use crate::terrain::{Heightmap, TerrainConfig};
use crate::ui::{GameTime, Notifications};

// ---------------------------------------------------------------------------
// Building definitions
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildingCategory {
    Producer,
    Residential,
}

pub struct BuildingDef {
    pub label: &'static str,
    pub category: BuildingCategory,
    pub lot_half_extents: Vec2,
    pub workers_required: u32,
    pub workers_provided: u32,
    pub extracts_from: Option<ResourceType>,
    pub extraction_radius: f32,
}

pub static BUILDING_DEFS: &[BuildingDef] = &[
    // 0: Timber Producer
    BuildingDef {
        label: "Logging Camp",
        category: BuildingCategory::Producer,
        lot_half_extents: Vec2::new(6.0, 5.0),
        workers_required: 5,
        workers_provided: 0,
        extracts_from: Some(ResourceType::Timber),
        extraction_radius: 60.0,
    },
    // 1: Residential
    BuildingDef {
        label: "Worker Cottage",
        category: BuildingCategory::Residential,
        lot_half_extents: Vec2::new(4.0, 4.0),
        workers_required: 0,
        workers_provided: 2,
        extracts_from: None,
        extraction_radius: 0.0,
    },
];

// ---------------------------------------------------------------------------
// Debug info resource
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct EconomyDebug {
    pub workers_needed: u32,
    pub workers_provided: u32,
    pub producer_viable: bool,
    pub producer_reason: &'static str,
    pub producer_candidates: usize,
    pub residential_viable: bool,
    pub residential_reason: &'static str,
    pub residential_candidates: usize,
    pub last_spawn: Option<String>,
    pub best_score: Option<f32>,
}

// ---------------------------------------------------------------------------
// Viability-based spawning
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub fn evaluate_and_spawn(
    mut commands: Commands,
    time: Res<Time>,
    game_time: Res<GameTime>,
    road_network: Res<RoadNetwork>,
    heightmap: Res<Heightmap>,
    config: Res<TerrainConfig>,
    resource_map: Res<ResourceMap>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    buildings_query: Query<(&Building, &Transform)>,
    lots_query: Query<&Lot>,
    mut notifications: ResMut<Notifications>,
    mut debug: ResMut<EconomyDebug>,
) {
    let dt = time.delta_secs() * game_time.speed.multiplier();
    spawn_timer.accumulator += dt;
    if spawn_timer.accumulator < spawn_timer.interval {
        return;
    }
    spawn_timer.accumulator -= spawn_timer.interval;

    // Clear per-tick debug fields
    debug.last_spawn = None;
    debug.best_score = None;
    debug.producer_candidates = 0;
    debug.residential_candidates = 0;

    if road_network.segments().is_empty() {
        debug.producer_viable = false;
        debug.producer_reason = "no roads";
        debug.residential_viable = false;
        debug.residential_reason = "no roads";
        return;
    }

    let existing_lots: Vec<(Vec2, Vec2, f32)> = lots_query
        .iter()
        .map(|lot| (lot.center, lot.half_extents, lot.rotation))
        .collect();

    let existing_buildings: Vec<(Vec3, usize)> = buildings_query
        .iter()
        .map(|(b, t)| (t.translation, b.def_index))
        .collect();

    // Compute global worker counts
    let mut workers_needed: u32 = 0;
    let mut workers_provided: u32 = 0;
    for &(_, def_idx) in &existing_buildings {
        let def = &BUILDING_DEFS[def_idx];
        workers_needed += def.workers_required;
        workers_provided += def.workers_provided;
    }
    debug.workers_needed = workers_needed;
    debug.workers_provided = workers_provided;

    // Priority: producer first, then residential
    let spawn_order: &[usize] = &[0, 1]; // Producer, Residential

    for &def_idx in spawn_order {
        let def = &BUILDING_DEFS[def_idx];

        let (viable, reason) = match def.category {
            BuildingCategory::Producer => {
                let (v, r) = is_producer_viable(
                    def,
                    &road_network,
                    &resource_map,
                    &config,
                    &existing_buildings,
                );
                debug.producer_viable = v;
                debug.producer_reason = r;
                (v, r)
            }
            BuildingCategory::Residential => {
                let v = workers_needed > workers_provided;
                let r = if v { "ok" } else { "workers satisfied" };
                debug.residential_viable = v;
                debug.residential_reason = r;
                (v, r)
            }
        };

        if !viable {
            let _ = reason; // used via debug above
            continue;
        }

        let candidates = find_candidates(
            def,
            &road_network,
            &heightmap,
            &config,
            &existing_lots,
        );

        match def.category {
            BuildingCategory::Producer => debug.producer_candidates = candidates.len(),
            BuildingCategory::Residential => debug.residential_candidates = candidates.len(),
        }

        if candidates.is_empty() {
            continue;
        }

        let best = candidates
            .iter()
            .map(|c| {
                let s = score_candidate(
                    c,
                    def,
                    &existing_buildings,
                    &heightmap,
                    &config,
                    &resource_map,
                );
                (c, s)
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let Some((candidate, score)) = best else {
            continue;
        };

        debug.best_score = Some(score);

        spawn_building(
            &mut commands,
            &mut meshes,
            &mut materials,
            candidate,
            def_idx,
            def,
        );

        debug.last_spawn = Some(format!(
            "{} at ({:.0}, {:.0})",
            def.label, candidate.position.x, candidate.position.z,
        ));

        notifications.push(format!("{} built", def.label), 3.0);
        return; // One building per tick
    }
}

// ---------------------------------------------------------------------------
// Viability checks
// ---------------------------------------------------------------------------

fn is_producer_viable(
    def: &BuildingDef,
    road_network: &RoadNetwork,
    resource_map: &ResourceMap,
    config: &TerrainConfig,
    existing_buildings: &[(Vec3, usize)],
) -> (bool, &'static str) {
    let extract_resource = match def.extracts_from {
        Some(r) => r,
        None => return (false, "no extraction resource"),
    };

    // Walk sampled points along every road segment, not just nodes,
    // so roads that pass through a resource zone are detected even when
    // the endpoint nodes sit outside the zone.
    let mut found_resource_on_road = false;
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

        for pos in sample_catmull_rom(&path, 4) {
            if let Some(cell) = resource_map.sample_world(pos.x, pos.z, config.map_size)
                && cell.resource == extract_resource
                && cell.richness > 0.2
            {
                found_resource_on_road = true;
                let has_nearby = existing_buildings.iter().any(|(b_pos, b_def_idx)| {
                    let b_def = &BUILDING_DEFS[*b_def_idx];
                    b_def.extracts_from == def.extracts_from
                        && b_pos.distance(pos) < def.extraction_radius
                });
                if !has_nearby {
                    return (true, "ok");
                }
            }
        }
    }
    if found_resource_on_road {
        (false, "extractor nearby")
    } else {
        (false, "no resource on road")
    }
}
