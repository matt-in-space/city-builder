# Bridge Construction

**Priority:** P3
**Status:** Not Started
**Depends On:** Road network, construction pipeline, basic economy

---

## Overview

Bridges allow roads to span water, valleys, and other terrain gaps. They are expensive, material-intensive infrastructure that unlocks new areas of the map for development. Bridge construction is a major investment that reflects the city's economic capacity.

In the 1920s setting, bridge types are constrained by era-appropriate technology: timber for short spans, steel truss for medium spans, concrete arch for longer spans. Each has different cost, material requirements, and visual character.

---

## Technical Details

**Bridge detection:**
- When a road segment would cross below-terrain or over water, it's flagged as a bridge segment
- Bridge requires anchor points on solid ground at each end
- Maximum span limited by bridge type and materials

**Bridge types (1920s era):**

| Type | Max Span | Materials | Cost | Visual |
|------|----------|-----------|------|--------|
| Timber | Short (~50 ft) | Lumber | Low | Wooden beam bridge |
| Steel Truss | Medium (~200 ft) | Steel, concrete | High | Truss framework |
| Concrete Arch | Long (~300 ft) | Concrete, steel | Very high | Arched span |

**Construction:**
- Bridges enter the standard construction pipeline but take significantly longer
- Material requirements are substantial — a steel bridge needs imported steel
- Construction stalls if materials are unavailable (economy integration)
- Bridge under construction is visually distinct (scaffolding, partial structure)

**Rendering:**
- Road surface hovers over the gap at appropriate height
- Support pillars (box geometry) extend down to ground/water
- Bridge type determines visual style of supports and deck
- Must clear what's underneath (water surface, terrain)

**Gameplay:**
- Bridges unlock access to isolated areas (resources across rivers, expansion across valleys)
- High cost creates meaningful economic decisions — is the bridge worth the investment?
- Bridge maintenance adds ongoing cost to city budget

---

## Implementation Checklist

- [ ] Detect when road segments span gaps (below terrain or over water)
- [ ] Flag bridge segments and determine required bridge type based on span length
- [ ] Validate bridge placement: anchor points on solid ground, span within limits
- [ ] Calculate bridge cost based on type, span length, and current material prices
- [ ] Integrate bridge construction with construction pipeline (longer duration, more materials)
- [ ] Generate bridge mesh: road surface, support pillars, type-appropriate structure
- [ ] Ensure bridge height clears terrain/water below
- [ ] Add bridge maintenance cost to city budget
- [ ] Visual states for bridge construction (scaffolding, partial structure)

---

## Acceptance Criteria

- Roads can span water and terrain gaps via bridges
- Bridge type is automatically selected based on span length
- Bridge construction takes significant time and materials
- Completed bridges are visually distinct from regular road segments (visible supports)
- Bridges enable access to previously isolated map areas
- Bridge cost reflects current material prices from the economy
