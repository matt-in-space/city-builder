use bevy::prelude::*;
use std::collections::HashMap;

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
