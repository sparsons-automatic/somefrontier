# Some Frontier Lore Bible

This is the canonical internal setting reference for player-visible names,
summaries, factions, places, materials, ships, and NPCs. It guides content; it
does not require the UI to explain the setting or promise story systems that do
not exist yet.

## Player Fantasy

The player is an independent ship operator working the edge of settled space.
They make a living by reading incomplete survey data, moving practical cargo,
turning raw material into useful hardware, maintaining a small fleet, and
opening routes that established industry has not made safe or routine.

Some Frontier is not a chosen-one story. Competence, preparation, and useful
relationships create progress. The player's ship should feel like a working
place and every new system like a logistical problem before it becomes a home.

## Setting Premise

The frontier is a sparse network of stations, survey archives, industrial
claims, and unreliable jump corridors. Settlement exists, but no single state
has enough reach to make the region orderly. Cooperatives, unions, route crews,
independent haulers, archives, and raiders each control part of the machinery
that keeps people alive.

Knowledge is infrastructure. A scan can reveal a viable mine, a route record can
make travel survivable, and a fabrication template can matter as much as ore.
Credits are useful because stations coordinate labor and scarce stock through
markets, but physical cargo remains the foundation of the economy.

## Tone And Constraints

- Practical, hopeful, and a little worn. Life is difficult, but ordinary people
  have built systems that work.
- Industrial rather than militaristic. Combat protects routes and cargo; it is
  not the only or default measure of success.
- Specific rather than grandiose. Prefer a damaged relay, a short fuel reserve,
  or an unusual mineral seam over galaxy-ending stakes.
- Strange discoveries should begin as observable facts. Let players encounter
  exotic matter, vitrine crystal, or an eldridium anomaly before declaring what
  it ultimately means.
- Technology is capable but not magical. Ships still require fuel, ammunition,
  components, repair work, survey data, and travel time.
- Avoid Earth-national stereotypes, foretold heroes, universal empires, and
  lore that depends on an unimplemented dialogue, quest, diplomacy, or colony
  system.

Humor can appear in worker slang, improvised equipment, or exuberant prototype
names such as `SUPER NUKE`. It should feel like people naming the tools they use,
not the setting laughing at itself.

## The Frontier Economy

The gameplay loop is also the in-world economic chain:

1. Cartographers, beacon crews, and independent operators establish what is
   present and whether a route is usable.
2. Miners and survey ships recover raw ore, ice, gases, crystals, and anomaly
   material from imperfectly known sites.
3. Depots aggregate bulk cargo; refineries and fabrication shops turn it into
   plates, cells, electronics, fuel, ammunition, and ship systems.
4. Haulers move those goods between specialized stations whose local stock is
   finite and slow to replace.
5. Yard crews consume parts and credits to keep ships operating and to fit the
   upgrades needed for more distant, hazardous routes.

This is why surveying, mining, processing, cargo, stations, contracts, research,
and upgrades matter. A new material should connect to this chain through a
credible source and at least one credible use. A station should have a reason to
exist beyond being another shop.

## Shipped Groups

These descriptions are boundaries for current content, not exhaustive histories.
Packs may add local chapters, vendors, ships, or tensions without making them a
monolithic government.

### Cinder Cooperative

An industrial cooperative coordinating starter-system refining, patrols, and
station logistics. It favors shared infrastructure, predictable trade, and
keeping essential routes open. Cinder is friendly by default, but it is a
working organization rather than a benevolent state.

### Freebelt Compact

A loose society of independent haulers and dock crews. Freebelters value local
agreements, cargo access, and freedom from centralized schedules. Their ships
and stations should sound practical, mobile, and trade-oriented.

### Helioforge Yard Union

The repair, refit, and drydock workers who keep frontier ships flying. Its
identity comes from skilled labor, safe machinery, and hard-earned knowledge of
what survives beyond maintained lanes.

### Frontier Cartographers

Survey archivists and route specialists who preserve scans, mineral reports,
and navigational records. Their authority comes from useful, verifiable
knowledge rather than territorial control.

### Frontier Beacon Authority

Signal-watch and navigation crews responsible for beacons, route telemetry,
and emergency broadcasts. It is a narrow service authority, not a general
police force.

### Redwake Raiders

A hostile raider society currently represented by probes and attacks on
frontier traffic. Their methods suggest scavenged automation, route shadowing,
and pressure on remote haulers. Their origin, leadership, and full social
structure remain deliberately open for later content.

### Traders, Patrols, And Civilians

Independent traders do not need to belong to one culture, and faction ownership
does not make every contact behave alike. Patrols protect infrastructure and
respond to threats; they should not imply a fully simulated legal system.
Vendors, miners, mechanics, archivists, and cargo crews make the frontier feel
inhabited even when their interaction is limited to current gameplay hooks.

## Naming Conventions

Names should be distinct at a glance, pronounceable, and short enough for the
game's tables and stock-egui windows.

- **Systems and regions:** use evocative geographic or working-route language:
  `Frontier`, `Duskfall Reach`, `Cinder Reaches`, `Ember Spiral`. Avoid strings
  of catalog numbers unless bureaucracy is the point.
- **Planets and sites:** pair a visual, material, or atmospheric image with a
  body type: `Cobalt Scar Planetoid`, `Marrow Ice Comet`, `Auric Needle Cluster`.
  Classification should describe what a surveyor sees, not disclose a secret.
- **Stations:** name the place for its location, history, or job: `Frontier
  Exchange`, `Ore Lattice Depot`, `Cinder Repair Yard`, `Pale Orbit Archive`.
  Service names can be more local: `Parts Cage`, `Scan Desk`, `Torchcut Bar`.
- **Factions:** use an identity plus an organizational form: Cooperative,
  Compact, Union, Cartographers, Authority, Raiders. Do not use different forms
  as synonyms; they hint at how the group coordinates itself.
- **Ships and NPC archetypes:** combine affiliation or origin with a readable
  job or hull: `Frontier Freehauler`, `Patrol Cutter`, `Redwake Probe`,
  `Twinspire Gunship`. A player hull name should suggest its role and silhouette.
- **People:** prefer compact names that read clearly in service lists, such as
  `Juno Vale`, `Toma Rusk`, `Mara Quill`, and `Sela Dorn`. Do not assign a real
  culture's naming rules to a faction unless that culture is intentionally and
  respectfully established.
- **Materials:** use familiar industrial names for familiar substances (`Iron
  ore`, `Copper plate`, `Fuel canister`) and restrained invented names for
  genuinely unusual discoveries (`Vitrine crystal`, `Eldridium nodule`).
- **Equipment and research:** state purpose before flourish. `Balanced Shield
  Matrix` and `Mining Calibration I` explain themselves; a more colorful name
  should still have a plain summary.
- **Internal IDs:** keep stable `snake_case` IDs. Let names and summaries evolve
  without changing save-facing identifiers.

Avoid reusing `Frontier`, `Cinder`, `Ember`, or `Duskfall` merely to sound like
the base game. A pack should develop a small, consistent local vocabulary of its
own.

## Player-Facing Writing

UI copy should answer the player's next practical question. Put the noun and
action first, then add one sentence of setting texture.

- Object name: usually 2–4 words.
- Summary: usually one sentence and under about 25 words when possible.
- Service or action description: say what it does, what it needs, or why it is
  unavailable before adding atmosphere.
- Planet summary: identify visible geology or conditions and connect them to
  survey, hazard, or material value without listing every mechanic.
- NPC summary: identify role, affiliation, and relevant behavior without writing
  a biography.
- Research summary: describe the knowledge gained and the practical result.

Prefer `Bulk ore yard where miners stage contract cargo` to a paragraph of
history. Longer lore belongs in future inspectable records, conversations, or
codex content once those systems exist. Do not hide rules exclusively in flavor
text, and do not repeat field data already visible beside the summary.

## Content-Pack Guidance

A content pack may introduce new societies, route cultures, naming traditions,
materials, and local conflicts. It does not need to imitate the core factions,
but it should preserve the setting's practical cause and effect.

For each themed pack, define privately or in its documentation:

- what work sustains the location;
- what goods, knowledge, or services move through it;
- who maintains its infrastructure and who pressures it;
- a small naming vocabulary shared by related places and objects;
- what the player can actually observe or do with the lore today.

References to another pack's faction, place, or material require the same
declared dependency and namespaced ID discipline as any other content reference.
Never assume `core` owns a universal culture. Packs can remain mechanically
compatible while disagreeing in-world.

Before merging player-visible content, review names and summaries together for
consistency, verify that new resources have sources and uses, and confirm that
the text describes shipped behavior rather than future intent.
