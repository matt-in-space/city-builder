# Event System

**Priority:** P3
**Status:** Not Started
**Depends On:** Basic Economy (p1-basic-economy), Population & Immigration (p1-population-immigration)

---

## Overview

A Paradox-style (Crusader Kings, Stellaris) event system where narrative popups present the player with choices that have real consequences in the simulation. Events are triggered by game state conditions and form branching chains where past choices affect future events.

Architecture is three layers: an **engine** (evaluates triggers, presents events, executes effects — built once), a **schema** (defines available condition and effect types — expanded as systems are added), and a **content** layer (actual event definitions in data files — RON/TOML, moddable, expanded indefinitely).

Events cover construction conflicts, economic crises, social dynamics, Prohibition, labor disputes, and historical anchors (Crash of '29). The initial release would ship with ~50-100 events; expansions add more.

---

## Implementation Checklist

- [ ] Define event data schema: ID, trigger conditions, narrative text, options with effects, chain references
- [ ] Build event engine: condition evaluation, presentation UI, effect execution, follow-up scheduling
- [ ] Implement event pacing/budget system (1-2 events per game-month max, crisis events override)
- [ ] Create event data file format (RON or TOML) and loader
- [ ] Author initial event set covering core gameplay scenarios
- [ ] Implement event chains (choices lead to follow-up events)
- [ ] Events reference specific buildings/districts when contextually relevant

---

## Acceptance Criteria

- Events popup based on game state conditions and present meaningful choices
- Choices have visible consequences in the simulation
- Event chains create narrative continuity across months/years of gameplay
- New events can be added via data files without code changes
