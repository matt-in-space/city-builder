# Traffic Simulation

**Priority:** P3
**Status:** Not Started
**Depends On:** Road network, population system, building spawning, basic economy

---

## Overview

Vehicles move along roads between buildings, creating visible traffic that reflects the city's economic activity. Traffic congestion emerges from bottlenecks in the road network and feeds back into the simulation — congested routes are slower, affecting commute times, goods delivery, and quality of life.

Traffic is both visual polish (the city feels alive when vehicles move) and a gameplay system (the player must design road networks that handle growing demand). It bridges the gap between static roads and a living city.

---

## Technical Details

**Vehicle types (1920s era):**
- Horse-drawn carts (early game, slow, freight and passenger)
- Automobiles (emerging, faster, mostly passenger)
- Trucks (freight, replacing horse carts gradually)
- Construction vehicles (appear at active construction sites)
- Streetcars (if rail system is implemented)

**Traffic model:**
- Simplified agent-based: vehicles spawn at origin, path-find to destination, despawn on arrival
- Pathfinding on the road network graph (A* or Dijkstra with edge weights based on distance, speed limit, congestion)
- Congestion: track vehicle density per road segment; high density reduces travel speed
- Vehicles don't need full physics — follow the road spline at appropriate speed, queue behind slower vehicles

**Trip generation:**
- Residents commute: home → work → home (daily cycle scaled to game time)
- Commercial trips: residents → shops for goods
- Freight trips: imports arrive from map edge, deliveries between businesses
- Construction trips: material deliveries to active construction sites

**Visual representation:**
- Simple box meshes colored by vehicle type moving along road splines
- Vehicles maintain spacing, slow at intersections, queue in congestion
- Parked vehicles near buildings (static decoration)

**Gameplay feedback:**
- Congestion overlay showing road segments by traffic level (green → yellow → red)
- Congested commutes reduce worker happiness
- Congested freight routes increase goods delivery time and cost
- Player responds by building alternative routes, wider roads, or transit

---

## Implementation Checklist

- [ ] Define vehicle types with properties: speed, size, capacity, visual appearance
- [ ] Implement trip generation: residents create commute trips, businesses create freight trips
- [ ] Implement pathfinding on road network graph (weighted by distance and congestion)
- [ ] Spawn vehicle entities that follow road splines from origin to destination
- [ ] Track vehicle density per road segment for congestion calculation
- [ ] Congestion reduces travel speed on affected segments
- [ ] Implement basic intersection behavior (vehicles slow/stop at junctions)
- [ ] Create placeholder vehicle meshes (colored boxes) for each vehicle type
- [ ] Add traffic/congestion overlay (toggleable) showing road segment load
- [ ] Feed congestion data back into simulation: happiness, delivery times, costs
- [ ] Add parked vehicle decoration near occupied buildings

---

## Acceptance Criteria

- Vehicles visibly move along roads between buildings
- Traffic volume corresponds to population and economic activity
- Congested road segments are visibly slower with queuing vehicles
- Congestion overlay clearly shows problem areas
- Building alternative routes reduces congestion on overloaded segments
- The city feels alive with movement during normal gameplay
