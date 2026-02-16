# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

1920s city builder / economic simulation game. The player acts as a city governor — setting policies, laying infrastructure, and responding to crises while the economy drives organic city growth. Built with Bevy 0.18 (Rust). See `docs/overview.md` for full game design and `docs/roadmap.md` for implementation milestones.

## Commands

- **Build/run:** `cargo run`
- **Check (no codegen):** `cargo check`
- **Run tests:** `cargo test`
- **Single test:** `cargo test test_name`
- **Clippy:** `cargo clippy`
- **Toolchain:** managed via mise (`mise.toml` pins Rust to latest)

Dev builds optimize dependencies at opt-level 2 (configured in `Cargo.toml`) so the game runs at playable framerates during development.

## Architecture

The project is early-stage — currently a single `src/main.rs` with a Bevy app, 3D camera, ground plane, and directional light.

The planned architecture (from `docs/roadmap.md`) follows Bevy's ECS pattern with these major systems to be built in order:

1. **Terrain** — heightmap generation, mesh, biome painting
2. **Roads** — spline-based freeform placement, mesh generation
3. **Zones & Lots** — brush-painted zones, lot subdivision from road network
4. **Building spawning** — placeholder geometry on lots, driven by economy
5. **Construction pipeline** — plan/under-construction/complete visual states
6. **Economy** — individual agents at small scale transitioning to statistical cohorts at large scale; goods, supply chains, market pricing, city budget

## Key Design Decisions

- **Economy-first:** buildings spawn from economic demand, not timers or population gates
- **Freeform roads:** spline-based, not grid-locked
- **Individual-to-statistical simulation:** discrete agents under ~500 pop, cohort-based above, with "notable households" preserved for narrative
- **Construction as process:** Plan → Approve → Procure → Construct → Complete pipeline
- **Prototype visuals:** colored boxes/planes first, Art Deco art assets later
