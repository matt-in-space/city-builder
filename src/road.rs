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

/// The currently active player tool.
#[derive(Resource, Default, PartialEq, Eq)]
pub enum ActiveTool {
    #[default]
    None,
    Road,
}

/// Distance (world units) within which a click snaps to an existing node.
const SNAP_RADIUS: f32 = 5.0;

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

        // Interior clicks become spline control points
        let control_points: Vec<Vec3> = if points.len() > 2 {
            points[1..points.len() - 1].to_vec()
        } else {
            Vec::new()
        };

        let start_node = road_network.nearest_node(start_pos, SNAP_RADIUS)
            .unwrap_or_else(|| road_network.add_node(start_pos));
        let end_node = road_network.nearest_node(end_pos, SNAP_RADIUS)
            .unwrap_or_else(|| road_network.add_node(end_pos));
        road_network.add_segment(start_node, end_node, control_points, RoadType::Dirt, 8.0);
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
        placement.points.push(point);
    }
}

/// Draw gizmo preview of the road being placed.
pub fn draw_road_placement_preview(
    placement: Res<RoadPlacementState>,
    active_tool: Res<ActiveTool>,
    mut gizmos: Gizmos,
) {
    if *active_tool != ActiveTool::Road || placement.points.is_empty() {
        return;
    }

    // Draw spheres at each placed point
    for &point in &placement.points {
        gizmos.sphere(Isometry3d::from_translation(point), 1.0, Color::srgb(1.0, 1.0, 0.0));
    }

    // Draw lines between consecutive points
    for window in placement.points.windows(2) {
        gizmos.line(window[0], window[1], Color::srgb(1.0, 1.0, 0.0));
    }
}
