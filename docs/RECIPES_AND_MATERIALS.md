# Recipes and Materials

This document tracks current and planned material chains for Some Frontier.

## Resource Tiers

| Tier | Role | Examples |
|---|---|---|
| Raw resource | Mined directly from planets | Iron ore, copper ore, nickel ore |
| Refined material | Smelted from raw resources | Iron plate, copper plate |
| Component | Fabricated from refined materials | Copper wire, gear, circuit |
| Utility item | Enables exploration or operations | Survey drone |
| Fuel and energy | Powers travel or advanced systems | Reactor cell, hydrogen fuel |

## Space Mining Resource Pool

These are candidate minerals and volatiles for future planets, recipes, and
upgrade chains.

| Resource | Resource Band | Primary Uses |
|---|---|---|
| Iron | Common starter | Frames, gears, plates, basic ship parts |
| Copper | Common starter | Wire, circuits, conductive parts |
| Nickel | Common starter | Alloys, engine parts, plating, batteries |
| Silicon | Common starter | Electronics, sensors, solar systems, scanners |
| Aluminum | Common starter | Lightweight hull parts, drone frames |
| Carbon | Common starter | Composites, filters, fuel chemistry |
| Water ice | Common starter | Hydrogen fuel, coolant, emergency fuel recovery |
| Sulfur | Mid-game industrial | Chemical processing, batteries, explosives |
| Magnesium | Mid-game industrial | Lightweight alloys, thermal systems, flares |
| Titanium | Mid-game industrial | Strong hull parts, engine upgrades |
| Cobalt | Mid-game industrial | High-performance alloys, batteries |
| Lithium | Mid-game industrial | Batteries, capacitors, drone power |
| Silver | Mid-game industrial | Advanced electronics, high-conductivity components |
| Gold | Mid-game industrial | Precision electronics, corrosion-resistant parts |
| Platinum group metals | Advanced industrial | Catalysts, advanced electronics, fuel processing |
| Rare earths | Advanced industrial | Sensors, motors, guidance, scanner upgrades |
| Uranium | Advanced energy | Reactor fuel, high-tier energy systems |
| Thorium | Advanced energy | Safer reactor fuel variant, stable power cells |
| Helium-3 | Advanced energy | Fusion fuel, long-range travel, gas giant systems |
| Exotic matter | Exotic late game | Jump drives, warp systems, extreme upgrades |

## Recommended Material Progression

| Phase | New Resources | New Production Branch |
|---|---|---|
| Starter expansion | Nickel, silicon, water ice | Alloys, electronics, fuel/coolant |
| Industrial expansion | Aluminum, titanium, lithium, sulfur | Lightweight frames, batteries, stronger upgrades |
| Advanced expansion | Cobalt, silver, gold, platinum group metals, rare earths | High-performance parts, advanced scanners, drone systems |
| Energy expansion | Uranium, thorium, helium-3 | Reactor fuels, fusion cells, long-range engines |
| Late-game expansion | Exotic matter, vitrine crystal, eldridium nodule | Jump systems, exotic containment, endgame upgrades |

## Current Materials

Core item definitions include `unit_mass` in kilograms per inventory unit.
Current values use gameplay-scale cargo chunks: common ore units are roughly
5-22 kg, refined stock is roughly 3-28 kg, compact electronics are 1-7 kg, and
larger drone/drive parts range from 35-48 kg.

| Item | Tier | Source | Used For |
|---|---|---|---|
| Iron ore | Raw resource | Ferric Copper Planetoid, Cobalt Scar Planetoid | Iron plates |
| Copper ore | Raw resource | Ferric Copper Planetoid, Argent Relay Rock | Copper plates |
| Nickel ore | Raw resource | Nickel-Iron Shard, Cobalt Scar Planetoid, Platinum Wake Debris | Future structural alloys |
| Silicon ore | Raw resource | Silicate Glass World | Future electronics and scanners |
| Aluminum ore | Raw resource | Pale Silica Reach, Basalt Magnesium World | Aluminum plates and lightweight frames |
| Magnesium ore | Raw resource | Kestrel Titanium Moon, Basalt Magnesium World | Lightweight alloys and industrial upgrades |
| Lithium ore | Raw resource | Brine Spark Body | Lithium cells and drone power |
| Titanium ore | Raw resource | Kestrel Titanium Moon, Thorium Quiet Moon | Titanium plates and stronger upgrades |
| Cobalt ore | Raw resource | Ember Nickel Drift, Kestrel Titanium Moon, Cobalt Scar Planetoid, Uranium Blackstone, Platinum Wake Debris | Cobalt alloys and high-performance parts |
| Silver ore | Raw resource | Auric Needle Cluster, Argent Relay Rock | Silver contacts and advanced electronics |
| Gold ore | Raw resource | Auric Needle Cluster | Gold conductors and precision electronics |
| Platinum group metals | Raw resource | Auric Needle Cluster, Platinum Wake Debris | High-performance catalysts and advanced engine parts |
| Rare earths | Raw resource | Pale Silica Reach, Argent Relay Rock, Thorium Quiet Moon, Vitrine Prism World | Rare earth magnets and scanner parts |
| Uranium ore | Raw resource | Uranium Blackstone | Reactor pellets |
| Thorium ore | Raw resource | Thorium Quiet Moon, Uranium Blackstone | Stable reactor pellets |
| Helium-3 | Raw resource | Helium Skimmer Belt | Fusion fuel cells |
| Exotic matter | Exotic | Eldridium Anomaly | Exotic matter capsules |
| Water ice | Raw resource | Fractured Ice Body | Future hydrogen fuel and coolant |
| Carbon | Raw resource | Fractured Ice Body | Future composites and fuel chemistry |
| Sulfur | Raw resource | Marrow Ice Comet, Brine Spark Body, Basalt Magnesium World | Battery chemistry and industrial processing |
| Azurite ore | Raw resource | Bleached Azure World, Vitrine Prism World | Future advanced recipes |
| Vitrine crystal | Raw resource | Bleached Azure World, Vitrine Prism World, Eldridium Anomaly | Future advanced recipes |
| Eldridium nodule | Raw resource | Bleached Azure World, Eldridium Anomaly | Future advanced recipes |
| Iron plate | Refined material | Smelt iron ore | Gears, circuits, upgrades |
| Copper plate | Refined material | Smelt copper ore | Copper wire, upgrades |
| Nickel plate | Refined material | Smelt nickel ore | Structural alloys, engine parts |
| Silicon wafer | Refined material | Process silicon ore | Electronics and scanner parts |
| Hydrogen fuel | Fuel and energy | Process water ice | Common travel fuel |
| Coolant | Fuel and energy | Process water ice | Reactor and engine support |
| Fuel canister | Fuel and energy | Process hydrogen fuel and aluminum plate | Stored travel fuel |
| Structural alloy | Refined material | Smelt iron plate and nickel plate | Ship hull and engine upgrades |
| Electronics | Component | Craft copper wire and silicon wafer | Scanner, drone, and upgrade recipes |
| Aluminum plate | Refined material | Smelt aluminum ore | Lightweight frames and drone bodies |
| Lithium cell | Component | Process lithium ore and copper wire | Batteries, capacitors, drone power |
| Titanium plate | Refined material | Smelt titanium ore | Strong hull and engine upgrades |
| Cobalt alloy | Refined material | Smelt cobalt ore and nickel plate | High-performance ship parts |
| Silver contact | Component | Process silver ore and copper wire | Advanced electronics |
| Gold conductor | Component | Process gold ore and copper wire | Precision electronics |
| Rare earth magnet | Component | Craft rare earths and cobalt alloy | Scanner motors and guidance systems |
| Reactor pellet | Fuel and energy | Process uranium or thorium ore with coolant | Reactor fuel |
| Helium-3 cell | Fuel and energy | Process helium-3 with electronics | Fusion fuel |
| Exotic matter capsule | Exotic | Craft exotic matter, vitrine crystal, and eldridium nodule | Late-game jump fuel |
| Advanced scanner core | Component | Craft electronics, rare earth magnet, and silver contact | High-tier survey upgrades |
| Fusion drive core | Component | Craft helium-3 cell, reactor pellet, and gold conductor | Long-range engine upgrades |
| Jump core | Exotic | Craft exotic matter capsule, advanced scanner core, and eldridium nodule | Late-game jump systems |
| Copper wire | Component | Fabricate copper plate | Circuits, survey drones |
| Gear | Component | Fabricate iron plate | Survey drones, upgrades |
| Circuit | Component | Fabricate iron plate and copper wire | Survey drones, upgrades |
| Survey drone | Utility item | Fabricate components | Planet scanning |
| Improved survey drone | Utility item | Upgrade survey drone with electronics and alloy | Future deeper scan capability |

## Current Recipes

| Output | Inputs | Station | Purpose |
|---|---|---|---|
| Iron plate x1 | Iron ore x2 | Smelting | Basic structural material |
| Copper plate x1 | Copper ore x2 | Smelting | Basic conductive material |
| Copper wire x2 | Copper plate x1 | Crafting | Conductive component |
| Gear x1 | Iron plate x2 | Crafting | Mechanical component |
| Circuit x1 | Iron plate x1, copper wire x3 | Crafting | Electronic component |
| Survey drone x1 | Circuit x1, gear x1, copper wire x4 | Crafting | One-way planet survey |
| Nickel plate x1 | Nickel ore x2 | Smelting | Stronger industrial material |
| Structural alloy x1 | Iron plate x1, nickel plate x1 | Smelting | Stronger upgrade material |
| Silicon wafer x1 | Silicon ore x2 | Processing | Electronics substrate |
| Hydrogen fuel x1 | Water ice x2 | Processing | Common ship fuel |
| Coolant x1 | Water ice x1, copper plate x1 | Processing | Reactor and engine support |
| Electronics x1 | Copper wire x2, silicon wafer x1 | Crafting | Scanner and drone component |
| Improved survey drone x1 | Survey drone x1, electronics x1, structural alloy x1 | Crafting | Deeper scan capability |
| Aluminum plate x1 | Aluminum ore x2 | Smelting | Lightweight industrial material |
| Lithium cell x1 | Lithium ore x1, copper wire x2 | Processing | Battery and capacitor component |
| Titanium plate x1 | Titanium ore x2 | Smelting | Strong hull and engine material |
| Cobalt alloy x1 | Cobalt ore x1, nickel plate x1 | Smelting | High-performance upgrade material |
| Silver contact x1 | Silver ore x1, copper wire x1 | Processing | Advanced electronics component |
| Gold conductor x1 | Gold ore x1, copper wire x1 | Processing | Precision electronics component |
| Rare earth magnet x1 | Rare earths x1, cobalt alloy x1 | Crafting | Scanner and guidance component |
| Reactor pellet x1 | Uranium ore x1, coolant x1 | Processing | High-output reactor fuel component |
| Reactor pellet x1 | Thorium ore x1, coolant x1 | Processing | Stable reactor fuel component |
| Helium-3 cell x1 | Helium-3 x1, electronics x1 | Processing | Fusion fuel component |
| Fuel canister x1 | Hydrogen fuel x3, aluminum plate x1 | Processing | Stored travel fuel |
| Exotic matter capsule x1 | Exotic matter x1, vitrine crystal x1, eldridium nodule x1 | Crafting | Late-game jump fuel |
| Advanced scanner core x1 | Electronics x1, rare earth magnet x1, silver contact x1 | Crafting | High-tier survey upgrade component |
| Fusion drive core x1 | Helium-3 cell x1, reactor pellet x1, gold conductor x1 | Crafting | Long-range engine upgrade component |
| Jump core x1 | Exotic matter capsule x1, advanced scanner core x1, eldridium nodule x1 | Crafting | Late-game jump upgrade component |

## Planet Resource Map

| Planet | Scan State | Resources | Notes |
|---|---|---|---|
| Bleached Azure World | Hidden until surveyed | Azurite ore, vitrine crystal, eldridium nodule | Early rare-resource testbed |
| Ferric Copper Planetoid | Hidden until surveyed | Iron ore, copper ore | Core mining and crafting testbed |
| Nickel-Iron Shard | Hidden until surveyed | Nickel ore, iron ore | Starter alloy resource testbed |
| Silicate Glass World | Hidden until surveyed | Silicon ore, copper ore | Electronics and scanner resource testbed |
| Fractured Ice Body | Hidden until surveyed | Water ice, carbon | Fuel, coolant, and chemistry testbed |
| Ember Nickel Drift | Hidden until surveyed | Nickel ore, iron ore, cobalt ore | Industrial metallic expansion body |
| Pale Silica Reach | Hidden until surveyed | Silicon ore, aluminum ore, rare earths | Electronics, scanners, and drone material body |
| Marrow Ice Comet | Hidden until surveyed | Water ice, carbon, sulfur | Fuel, coolant, and chemistry expansion body |
| Kestrel Titanium Moon | Hidden until surveyed | Titanium ore, magnesium ore, cobalt ore | Heavy-metal upgrade and alloy body |
| Brine Spark Body | Hidden until surveyed | Lithium ore, sulfur, carbon | Battery chemistry and drone power body |
| Auric Needle Cluster | Hidden until surveyed | Gold ore, silver ore, platinum group metals | Precious-metal electronics and engine resource body |
| Argent Relay Rock | Hidden until surveyed | Silver ore, copper ore, rare earths | Conductive electronics and scanner resource body |
| Cobalt Scar Planetoid | Hidden until surveyed | Cobalt ore, nickel ore, iron ore | Magnetic industrial alloy resource body |
| Thorium Quiet Moon | Hidden until surveyed | Thorium ore, rare earths, titanium ore | Stable reactor and heavy-metal resource body |
| Uranium Blackstone | Hidden until surveyed | Uranium ore, thorium ore, cobalt ore | Hazardous reactor and heavy-metal resource body |
| Helium Skimmer Belt | Hidden until surveyed | Helium-3 | Fusion fuel and long-range travel body |
| Basalt Magnesium World | Hidden until surveyed | Magnesium ore, aluminum ore, sulfur | Volcanic silicate and lightweight alloy resource body |
| Platinum Wake Debris | Hidden until surveyed | Platinum group metals, nickel ore, cobalt ore | Rare impact debris and advanced engine resource body |
| Vitrine Prism World | Hidden until surveyed | Vitrine crystal, azurite ore, rare earths | Crystal-rich advanced scanner and rare-resource body |
| Eldridium Anomaly | Hidden until surveyed | Exotic matter, eldridium nodule, vitrine crystal | Unstable exotic late-game resource POI |

## Planned Planet Archetypes

| Archetype | Likely Resources | Gameplay Role |
|---|---|---|
| Metallic asteroid cluster | Iron, nickel, cobalt, platinum group metals | Industrial metals and alloy progression |
| Silicate world | Silicon, aluminum, magnesium, rare earths | Electronics, scanners, and drone parts |
| Icy body | Water ice, carbon, sulfur | Fuel, coolant, and chemistry |
| Heavy-metal moon | Titanium, uranium, thorium | Advanced hull and reactor systems |
| Gas giant skimmer zone | Helium-3 | Fusion fuel and long-range travel |

## Planet Coverage Plan

The material roadmap should not require one planet per resource. Most planets
should carry two or three related resources, with common resources repeated
often enough that the player is not locked to a single supply point.

Baseline target: **14-16 total mineable bodies** for the current roadmap.

Current implemented bodies: **20**.

Recommended next additions: **0** more bodies for baseline coverage.

### Coverage Targets by Rarity

| Resource Band | Resources | Target Sources | Notes |
|---|---|---|---|
| Common starter | Iron, copper, nickel, silicon, aluminum, carbon, water ice | 3-5 sources each | Avoid bottlenecks for foundational crafting and fuel recovery |
| Mid-game industrial | Sulfur, magnesium, titanium, cobalt, lithium, silver, gold | 2-3 sources each | Enough redundancy for routing choices and planet richness tuning |
| Advanced industrial | Platinum group metals, rare earths | 1-2 sources each | Valuable but not singular unless tied to a special planet |
| Advanced energy | Uranium, thorium, helium-3 | 1-2 sources each | Sparse, but should have at least one alternate source before hard fuel gates |
| Exotic late game | Exotic matter, vitrine crystal, eldridium nodule | 1 source each to start | Can be rare destination rewards or special POI resources |

### Proposed Planet Set

| Count | Archetype | Primary Resources | Secondary Resources | Purpose |
|---:|---|---|---|---|
| 2 | Core metallic worlds | Iron, copper | Nickel | Starter plates, wire, gears, circuits |
| 2 | Nickel/industrial metallic bodies | Nickel, iron | Cobalt, platinum group metals | Alloys and engine parts |
| 2 | Silicate worlds | Silicon, aluminum | Magnesium, rare earths | Electronics, scanners, drones |
| 2 | Icy bodies | Water ice, carbon | Sulfur | Fuel, coolant, chemistry |
| 1 | Titanium moon | Titanium | Magnesium, cobalt | Strong hull and engine upgrades |
| 1 | Battery mineral body | Lithium, sulfur | Carbon | Batteries, capacitors, drone power |
| 1 | Precious-metal body | Silver, gold | Platinum group metals | Advanced electronics and catalysts |
| 1 | Heavy reactor moon | Uranium, thorium | Rare earths | Reactor fuel and power systems |
| 1 | Gas giant skimmer zone | Helium-3 | None or trace exotic matter | Fusion fuel and long-range travel |
| 1 | Exotic anomaly | Exotic matter | Eldridium nodule, vitrine crystal | Late-game jump or warp systems |
| 1 | Rare crystal world | Vitrine crystal, azurite ore | Eldridium nodule | Advanced scanning, shields, exotic containment |

This plan creates **15 total bodies** and covers every resource in the current
material roadmap with sensible overlap.

### Current Coverage Gaps

| Resource | Current Sources | Desired Sources | Gap |
|---|---:|---:|---:|
| Iron | 2 | 3-5 | 1-3 |
| Copper | 2 | 3-5 | 1-3 |
| Nickel | 1 | 3-5 | 2-4 |
| Silicon | 1 | 3-5 | 2-4 |
| Aluminum | 0 | 3-5 | 3-5 |
| Carbon | 1 | 3-5 | 2-4 |
| Water ice | 1 | 3-5 | 2-4 |
| Sulfur | 0 | 2-3 | 2-3 |
| Magnesium | 0 | 2-3 | 2-3 |
| Titanium | 0 | 2-3 | 2-3 |
| Cobalt | 0 | 2-3 | 2-3 |
| Lithium | 0 | 2-3 | 2-3 |
| Silver | 0 | 2-3 | 2-3 |
| Gold | 0 | 2-3 | 2-3 |
| Platinum group metals | 0 | 1-2 | 1-2 |
| Rare earths | 0 | 1-2 | 1-2 |
| Uranium | 0 | 1-2 | 1-2 |
| Thorium | 0 | 1-2 | 1-2 |
| Helium-3 | 0 | 1-2 | 1-2 |
| Exotic matter | 0 | 1 | 1 |
| Azurite ore | 1 | 1 | 0 |
| Vitrine crystal | 1 | 1 | 0 |
| Eldridium nodule | 1 | 1 | 0 |

### Placement Guidance

- Keep at least two starter-resource planets near the initial area.
- Place fuel/coolant resources close enough that fuel never becomes an early
  soft-lock.
- Use richer or rarer planets as reasons to travel farther, not as the only
  way to continue basic progression.
- Put advanced energy and exotic resources behind distance, scan tier, hazard,
  or ship-upgrade pressure.
- Prefer overlapping resource sets so multiple planets can support a recipe
  chain in different ways.

## Planned Fuel Chain

| Output | Inputs | Role |
|---|---|---|
| Basic reactor cell | Iron plate, copper wire, azurite ore | Recharge or boost ship energy |
| Water ice | Mined from icy planets, comets, or asteroids | Raw fuel resource |
| Hydrogen fuel | Water ice | Common fuel for longer-range travel |
| Fuel canister | Hydrogen fuel, copper plate | Stored travel fuel |
| Helium-3 cell | Helium-3, circuit, copper plate | Advanced fusion fuel |
| Exotic matter capsule | Exotic matter, vitrine crystal, eldridium nodule | Late-game jump fuel |

## Planned Material Uses

| Material | Future Uses |
|---|---|
| Azurite ore | Reactor cells, scanner upgrades, shield tuning |
| Vitrine crystal | Advanced sensors, drone return systems, map filters |
| Eldridium nodule | High-tier engines, exotic fuel containment, jump systems |
| Nickel | Structural alloys, stronger engine parts, industrial plating |
| Silicon | Electronics, scanners, solar systems, drone guidance |
| Water ice | Hydrogen fuel, coolant, emergency travel recovery |
| Survey drone | Tiered survey levels, hazard scouting, eventual reusable drones |

## Balance Notes

- Early recipes should use iron and copper so the player can learn the loop on
  the core-resource planet.
- Rare resources should unlock new capabilities rather than replace basic
  materials immediately.
- Fuel systems need an emergency recovery path before fuel becomes a hard travel
  cap.
- Drone costs should stay low while drones are consumed on launch.
