# Completed Features

Features that have been fully implemented. Each entry summarizes what was built and notes any implementation details worth knowing.

---

## Wave 1: Foundation

### Terrain Generation & Rendering
Heightmap-based terrain using fBm Perlin noise (seed 42, 6 octaves). 256x256 grid over a 500x500 world (≈2 units/cell). Biome classification (Sand/Grass/Dirt/Rock) based on elevation, slope, and water level. Vertex-colored mesh with per-vertex normals. Water plane at configurable elevation (default 10.0). Height scale defaults to 30.0.

**Key code:** `src/terrain.rs` — `TerrainConfig`, `Heightmap` (with `sample_world()` for bilinear interpolation), `BiomeMap`, generation + mesh systems.

### Camera Controls
Orbit camera with WASD ground-plane movement, scroll zoom along look direction, right-click drag rotation. Speed scales with camera height. Uses `CityCamera` marker component.

**Key code:** `src/camera.rs`

### Road Network & Mesh Generation
Freeform spline-based roads. Click to place control points on terrain (R to toggle tool, Enter to confirm, Escape to cancel). Catmull-Rom spline interpolation. Snap-to-existing nodes (3.0 radius). Automatic segment splitting at intersections. Road mesh generated from spline cross-sections projected onto terrain with 0.15 Y offset. Intersection disc polygons at junctions (12-sided). Vertex-colored by road type (Dirt/Gravel/Paved). Default width 2.0 units. Minimum segment length 3.0 units.

**Key code:** `src/road.rs` — `RoadNetwork` graph (nodes + segments), `RoadPlacementState`, `ActiveTool`, mesh generation, debug gizmos (white nodes, orange segments, yellow preview).

### UI & Game State
egui-based HUD with date (starting Jan 1920), speed controls, city funds, population count. Left toolbar (Select, Road, Zone stub, Building stub). Game speed: Pause/Normal/Fast/VeryFast (Space toggle, 1/2/3 keys). 10 real seconds = 1 game month at 1x. Cursor world position via per-frame terrain raycast. Info panel showing position, elevation, resource info, nearby road nodes. Timed notification system.

**Key code:** `src/ui.rs` — `GameTime`, `GameSpeed`, `CityBudget`, `CursorWorldPosition`, `Notifications`.

**Key detail:** UI systems run on `EguiPrimaryContextPass` schedule. Input consumption checked via `bevy_egui::input::EguiWantsInput`.

---

## Wave 2: Core Gameplay Loop (In Progress)

### Map Resources
Procedural resource layer overlaid on terrain at heightmap resolution (256x256). Five resource types generated from terrain features:

| Resource | Terrain Conditions | Noise Seed | Noise Threshold |
|---|---|---|---|
| Coal | Dirt/Rock biome, elevation > 40% | 200 | 0.5 (rare, clustered) |
| Clay | Near water, low elevation (water+0 to water+4) | 300 | 0.3 |
| Stone | Rock biome | 400 | 0.3 |
| Fertile Land | Very flat (>0.96), near water, low elevation | 500 | -0.2 |
| Timber | Grass/Dirt biome, moderate slope, above water+3 | 100 | 0.2 (most common) |

Priority order prevents overlap: Coal > Clay > Stone > Fertile Land > Timber. Each cell stores resource type + richness (0.0-1.0). Resources are visually represented as color tints blended into terrain vertex colors at 70% richness weight. Info panel shows resource type and richness on hover.

**Key code:** `src/resources.rs` — `ResourceType`, `ResourceMap` (with `sample_world()`), `generate_resource_map()`.

**Deferred items:** Resource accessibility check (whether a resource is connected to road network) will be handled by building spawning as a cross-system query. Resource overlay toggle is future polish.
