# Construction Animation & Polish

**Priority:** P3
**Status:** Not Started
**Depends On:** Construction Pipeline (p0-construction-pipeline)

---

## Overview

Replace the translucent placeholder construction states with animated construction sequences. Scaffolding goes up, workers appear on site, material trucks arrive, buildings rise floor by floor. The goal is to make the player want to zoom in and watch their city being built. This is a major "feel" feature that brings the simulation to life.

---

## Implementation Checklist

- [ ] Scaffolding meshes that appear around buildings under construction
- [ ] Animated construction progress (building rises incrementally rather than appearing all at once)
- [ ] Construction vehicles (trucks, wagons) arriving at sites with materials
- [ ] Worker figures visible at construction sites
- [ ] Dust/activity particles at construction sites
- [ ] Construction sounds (hammering, sawing) when zoomed in
- [ ] Road construction shows grading, material laying, and paving phases

---

## Acceptance Criteria

- Zooming into a construction site shows visible activity — workers, vehicles, scaffolding
- Buildings visibly rise over time rather than popping from translucent to solid
- Construction feels like a real process happening in the world
