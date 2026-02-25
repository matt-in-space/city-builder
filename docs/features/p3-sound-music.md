# Sound & Music

**Priority:** P3
**Status:** Not Started
**Depends On:** Core gameplay systems (for contextual audio)

---

## Overview

Era-appropriate audio that brings the city to life. Jazz-age music, ambient city sounds, and contextual audio feedback that responds to what's happening in the simulation. Sound is one of the most impactful polish layers — a sawmill should sound like a sawmill, and a bustling commercial district should sound different from a quiet residential street.

---

## Technical Details

**Music:**
- 1920s jazz and ragtime inspired soundtrack
- Adaptive music that responds to game state: upbeat during boom times, somber during recession
- Smooth transitions between tracks based on context
- Volume scales with game speed (quieter when fast-forwarding)

**Ambient sounds:**
- Layered ambient system based on camera position and nearby buildings
- Industrial areas: machinery, hammering, steam
- Commercial areas: chatter, store bells, automobiles
- Residential areas: quieter, birds, distant sounds
- Construction sites: hammering, sawing, vehicle engines
- Water: gentle lapping, river flow
- Wind on open terrain

**UI sounds:**
- Button clicks, tool selection
- Road placement confirmation
- Building construction start/complete
- Notification chimes
- Event popup sounds

**Implementation approach:**
- Use Bevy's built-in audio or `bevy_kira_audio` for more control
- Spatial audio: sounds positioned in 3D, volume based on camera distance
- Ambient sound zones tied to building clusters and terrain features
- Music system separate from spatial audio (2D, always audible)

---

## Implementation Checklist

- [ ] Set up audio plugin and asset pipeline
- [ ] Implement background music system with track transitions
- [ ] Source or create 1920s-inspired music tracks
- [ ] Implement spatial ambient sound system tied to building types and areas
- [ ] Create ambient sound assets (industrial, commercial, residential, nature)
- [ ] Add UI sound effects for interactions (clicks, confirmations, notifications)
- [ ] Add construction sound effects (contextual to construction state)
- [ ] Implement adaptive music responding to economic state (boom vs bust)
- [ ] Add volume controls and audio settings
- [ ] Camera-distance-based volume scaling for spatial sounds

---

## Acceptance Criteria

- Background music plays with era-appropriate style
- Music adapts to game state (upbeat in growth, somber in decline)
- Zooming into different areas produces distinct ambient soundscapes
- Construction sites have audible activity
- UI interactions provide audio feedback
- Player can control volume levels independently (music, SFX, ambient)
