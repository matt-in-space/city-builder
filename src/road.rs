use bevy::prelude::*;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings};
use std::collections::HashMap;

use crate::terrain::TerrainMesh;

/// Surface material of a road. Affects cost, speed, and visuals.
/// Only Dirt is used initially — the others exist for future upgrade progression.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum RoadType {
    #[default]
    Dirt,
    Gravel,
    Paved,
}

/// Unique identifier for a node in the road network.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(pub u32);

/// Unique identifier for a segment in the road network.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SegmentId(pub u32);

/// A junction or endpoint in the road network.
pub struct RoadNode {
    pub position: Vec3,
    /// Segments connected to this node.
    pub segments: Vec<SegmentId>,
}

/// A road connecting two nodes, with a spline path between them.
pub struct RoadSegment {
    /// The two endpoints of this road segment.
    pub nodes: [NodeId; 2],
    /// Interior control points for the spline curve (excluding the endpoint positions).
    /// An empty vec means a straight line between the two nodes.
    pub control_points: Vec<Vec3>,
    pub road_type: RoadType,
    /// Road width in world units.
    pub width: f32,
}

impl Default for RoadSegment {
    fn default() -> Self {
        Self {
            nodes: [NodeId(0), NodeId(0)],
            control_points: Vec::new(),
            road_type: RoadType::default(),
            width: 8.0,
        }
    }
}

/// The road network graph. Stores all nodes and segments, queryable by ID.
#[derive(Resource, Default)]
pub struct RoadNetwork {
    nodes: HashMap<NodeId, RoadNode>,
    segments: HashMap<SegmentId, RoadSegment>,
    next_node_id: u32,
    next_segment_id: u32,
}

impl RoadNetwork {
    /// Add a node at a position. Returns its ID.
    pub fn add_node(&mut self, position: Vec3) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        self.nodes.insert(id, RoadNode {
            position,
            segments: Vec::new(),
        });
        id
    }

    /// Add a segment between two existing nodes. Returns its ID.
    /// Registers the segment on both nodes.
    pub fn add_segment(
        &mut self,
        from: NodeId,
        to: NodeId,
        control_points: Vec<Vec3>,
        road_type: RoadType,
        width: f32,
    ) -> SegmentId {
        let id = SegmentId(self.next_segment_id);
        self.next_segment_id += 1;

        self.segments.insert(id, RoadSegment {
            nodes: [from, to],
            control_points,
            road_type,
            width,
        });

        if let Some(node) = self.nodes.get_mut(&from) {
            node.segments.push(id);
        }
        if let Some(node) = self.nodes.get_mut(&to) {
            node.segments.push(id);
        }

        id
    }

    pub fn node(&self, id: NodeId) -> Option<&RoadNode> {
        self.nodes.get(&id)
    }

    pub fn segment(&self, id: SegmentId) -> Option<&RoadSegment> {
        self.segments.get(&id)
    }

    pub fn nodes(&self) -> &HashMap<NodeId, RoadNode> {
        &self.nodes
    }

    pub fn segments(&self) -> &HashMap<SegmentId, RoadSegment> {
        &self.segments
    }

    /// Remove a segment and unregister it from its endpoint nodes.
    pub fn remove_segment(&mut self, id: SegmentId) {
        if let Some(segment) = self.segments.remove(&id) {
            for node_id in &segment.nodes {
                if let Some(node) = self.nodes.get_mut(node_id) {
                    node.segments.retain(|&s| s != id);
                }
            }
        }
    }

    /// Split an existing segment at a position, creating a new intersection node
    /// and two sub-segments that replace the original. Returns the new node ID.
    pub fn split_segment_at(&mut self, segment_id: SegmentId, position: Vec3) -> NodeId {
        let (nodes, road_type, width) = {
            let segment = &self.segments[&segment_id];
            (segment.nodes, segment.road_type, segment.width)
        };

        self.remove_segment(segment_id);

        let mid_node = self.add_node(position);
        self.add_segment(nodes[0], mid_node, Vec::new(), road_type, width);
        self.add_segment(mid_node, nodes[1], Vec::new(), road_type, width);

        mid_node
    }

    /// Find the nearest node within a radius. Used for snap-to-existing behavior.
    pub fn nearest_node(&self, position: Vec3, max_distance: f32) -> Option<NodeId> {
        let max_dist_sq = max_distance * max_distance;
        let mut best: Option<(NodeId, f32)> = None;

        for (&id, node) in &self.nodes {
            let dist_sq = node.position.distance_squared(position);
            if dist_sq < max_dist_sq {
                if best.is_none() || dist_sq < best.unwrap().1 {
                    best = Some((id, dist_sq));
                }
            }
        }

        best.map(|(id, _)| id)
    }
}

/// Evaluate a Catmull-Rom spline at parameter `t` (0..1) for the segment
/// between `p1` and `p2`, using `p0` and `p3` as context points.
fn catmull_rom_point(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

/// Sample points along a Catmull-Rom spline passing through the given positions.
/// Endpoints are duplicated as phantom control points so the curve starts/ends
/// exactly at the first/last position.
///
/// Returns `samples_per_segment * (points.len() - 1) + 1` evenly-spaced points.
pub fn sample_catmull_rom(points: &[Vec3], samples_per_segment: usize) -> Vec<Vec3> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let n = points.len();
    let mut result = Vec::with_capacity(samples_per_segment * (n - 1) + 1);

    for i in 0..(n - 1) {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < n { points[i + 2] } else { points[n - 1] };

        for s in 0..samples_per_segment {
            let t = s as f32 / samples_per_segment as f32;
            result.push(catmull_rom_point(p0, p1, p2, p3, t));
        }
    }

    result.push(*points.last().unwrap());
    result
}

/// Test if two line segments intersect in the XZ plane.
/// Returns (t, u) parameters along segments A and B respectively.
/// Excludes near-endpoint hits to avoid spurious splits at shared nodes.
fn segment_intersection_xz(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> Option<(f32, f32)> {
    let d1x = a2.x - a1.x;
    let d1z = a2.z - a1.z;
    let d2x = b2.x - b1.x;
    let d2z = b2.z - b1.z;

    let cross = d1x * d2z - d1z * d2x;
    if cross.abs() < 1e-6 {
        return None; // Parallel or collinear
    }

    let dx = b1.x - a1.x;
    let dz = b1.z - a1.z;
    let t = (dx * d2z - dz * d2x) / cross;
    let u = (dx * d1z - dz * d1x) / cross;

    // Exclude hits very close to endpoints to avoid double-splits at shared nodes
    let eps = 0.01;
    if t > eps && t < (1.0 - eps) && u > eps && u < (1.0 - eps) {
        Some((t, u))
    } else {
        None
    }
}

/// The currently active player tool.
#[derive(Resource, Default, PartialEq, Eq)]
pub enum ActiveTool {
    #[default]
    None,
    Road,
}

/// Distance (world units) within which a click snaps to an existing node.
const SNAP_RADIUS: f32 = 5.0;

/// Minimum distance between consecutive placed points to prevent micro-roads from misclicks.
const MIN_SEGMENT_LENGTH: f32 = 3.0;

/// Tracks in-progress road placement (points placed so far).
#[derive(Resource, Default)]
pub struct RoadPlacementState {
    pub points: Vec<Vec3>,
}

/// Toggle road placement tool with R key.
pub fn toggle_road_tool(
    keys: Res<ButtonInput<KeyCode>>,
    mut active_tool: ResMut<ActiveTool>,
    mut placement: ResMut<RoadPlacementState>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        if *active_tool == ActiveTool::Road {
            *active_tool = ActiveTool::None;
        } else {
            *active_tool = ActiveTool::Road;
        }
        placement.points.clear();
    }
}

/// Place road control points on the terrain via mouse click + raycast.
///
/// - Left click: place a point on the terrain
/// - Enter: confirm the road (creates nodes and a segment in the RoadNetwork)
/// - Escape: cancel placement
pub fn road_placement_input(
    mut ray_cast: MeshRayCast,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    terrain_query: Query<(), With<TerrainMesh>>,
    mut placement: ResMut<RoadPlacementState>,
    mut road_network: ResMut<RoadNetwork>,
    active_tool: Res<ActiveTool>,
) {
    if *active_tool != ActiveTool::Road {
        return;
    }

    // Cancel placement with Escape
    if keys.just_pressed(KeyCode::Escape) {
        placement.points.clear();
        return;
    }

    // Confirm road with Enter (need at least 2 points)
    if keys.just_pressed(KeyCode::Enter) && placement.points.len() >= 2 {
        let points = std::mem::take(&mut placement.points);
        let start_pos = points[0];
        let end_pos = *points.last().unwrap();

        let start_node = road_network.nearest_node(start_pos, SNAP_RADIUS)
            .unwrap_or_else(|| road_network.add_node(start_pos));
        let end_node = road_network.nearest_node(end_pos, SNAP_RADIUS)
            .unwrap_or_else(|| road_network.add_node(end_pos));

        let start_world = road_network.node(start_node).unwrap().position;
        let end_world = road_network.node(end_node).unwrap().position;

        // Find intersections between the new road line and existing segments
        let mut intersections: Vec<(SegmentId, Vec3)> = Vec::new();
        for (&seg_id, segment) in road_network.segments() {
            let a = road_network.node(segment.nodes[0]).unwrap().position;
            let b = road_network.node(segment.nodes[1]).unwrap().position;
            if let Some((t, _)) = segment_intersection_xz(start_world, end_world, a, b) {
                let point = start_world.lerp(end_world, t);
                intersections.push((seg_id, point));
            }
        }

        if intersections.is_empty() {
            // No crossings — create single segment with control points
            let control_points: Vec<Vec3> = if points.len() > 2 {
                points[1..points.len() - 1].to_vec()
            } else {
                Vec::new()
            };
            road_network.add_segment(start_node, end_node, control_points, RoadType::Dirt, 8.0);
        } else {
            // Sort intersections by distance along new road from start
            intersections.sort_by(|a, b| {
                let da = a.1.distance_squared(start_world);
                let db = b.1.distance_squared(start_world);
                da.partial_cmp(&db).unwrap()
            });

            // Split each crossed segment and collect intersection node IDs
            let mut chain: Vec<NodeId> = vec![start_node];
            for (seg_id, point) in intersections {
                let int_node = road_network.split_segment_at(seg_id, point);
                chain.push(int_node);
            }
            chain.push(end_node);

            // Create new road as chain of segments through intersection nodes
            for pair in chain.windows(2) {
                road_network.add_segment(pair[0], pair[1], Vec::new(), RoadType::Dirt, 8.0);
            }
        }
        return;
    }

    // Place point on left click
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(window) = window.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let filter = |entity: Entity| terrain_query.contains(entity);
    let settings = MeshRayCastSettings::default()
        .with_filter(&filter);

    let hits = ray_cast.cast_ray(ray, &settings);
    if let Some((_, hit)) = hits.first() {
        // Snap to nearby existing node if one exists
        let point = if let Some(node_id) = road_network.nearest_node(hit.point, SNAP_RADIUS) {
            road_network.node(node_id).unwrap().position
        } else {
            hit.point
        };
        // Reject if too close to the last placed point
        if let Some(&last) = placement.points.last() {
            if point.distance(last) < MIN_SEGMENT_LENGTH {
                return;
            }
        }
        placement.points.push(point);
    }
}

/// Draw gizmo preview of the road being placed (yellow)
/// and debug visualization of all committed roads in the network (white nodes, orange segments).
pub fn draw_road_debug(
    placement: Res<RoadPlacementState>,
    active_tool: Res<ActiveTool>,
    road_network: Res<RoadNetwork>,
    mut gizmos: Gizmos,
) {
    // --- Committed road network ---
    let node_color = Color::srgb(1.0, 1.0, 1.0);
    let segment_color = Color::srgb(1.0, 0.6, 0.2);

    for node in road_network.nodes().values() {
        gizmos.sphere(Isometry3d::from_translation(node.position), 0.8, node_color);
    }

    for segment in road_network.segments().values() {
        let Some(a) = road_network.node(segment.nodes[0]) else { continue };
        let Some(b) = road_network.node(segment.nodes[1]) else { continue };

        // Build full point sequence: start + control points + end
        let mut path = vec![a.position];
        path.extend_from_slice(&segment.control_points);
        path.push(b.position);

        let curve = sample_catmull_rom(&path, 16);
        for pair in curve.windows(2) {
            gizmos.line(pair[0], pair[1], segment_color);
        }
    }

    // --- In-progress placement preview (yellow curve) ---
    if *active_tool != ActiveTool::Road || placement.points.is_empty() {
        return;
    }

    let preview_color = Color::srgb(1.0, 1.0, 0.0);
    for &point in &placement.points {
        gizmos.sphere(Isometry3d::from_translation(point), 1.0, preview_color);
    }

    if placement.points.len() >= 2 {
        let curve = sample_catmull_rom(&placement.points, 16);
        for pair in curve.windows(2) {
            gizmos.line(pair[0], pair[1], preview_color);
        }
    }
}
