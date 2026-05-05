# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start

### Building and Running

```bash
# Development build with optimized dependencies
cargo build

# Run the game
cargo run

# Run with dev features (dynamic linking for faster iteration)
cargo run --features dev

# Build release
cargo build --release
```

### Workspace Structure

```
src/
  main.rs              # App entry, plugin registration, WeekQueue resource init
  inputevents/         # Keyboard, mouse, scroll input → InputEvent messages
    mod.rs             # InputPlugin, InputEvent, InputAction enums
    systems.rs         # Input handler systems (handle_input, handle_mouse_drag)
  map/                 # Geographic data, camera, province/town systems
    mod.rs             # MapPlugin, GeoJSON loading, camera system, coordinate conversions
    province.rs        # ProvinceDef, ProvinceMap, spatial queries (point-in-polygon)
    town.rs            # TownDef, TownMap resource
    picking.rs         # province_picking_system (raycasting from cursor to provinces)
    render.rs          # setup_map_layers, update_province_colours, towns rendering
  player/              # Player character, location, markers
    mod.rs             # PlayerCharacter, Location, player_hover_system, spawn systems
  action_queue/        # Turn-based action planning
    mod.rs             # WeekQueue resource, QueuedAction enum (Travel | Idle)
  dom_ui/              # Egui-based UI with parchment theme
    mod.rs             # DomUIPlugin, province info panel, player tooltip
    assets.rs          # Font loading, texture registration, theme styling
    week_queue.rs      # week_queue_ui system — 7-day plan panel, Step/Clear buttons
```

## Architecture Overview

Dominium is a map-based strategy game with Bevy 0.18.1 as its core engine. The architecture follows a plugin-based, system-driven design:

### Plugin Structure

**Core Plugins:**
- `InputPlugin` — captures input (keyboard, mouse, scroll) and sends `InputEvent` messages
- `MapPlugin` — loads provinces/towns, manages camera, handles spatial queries for picking
- `DomUIPlugin` — renders UI panels (province info, player tooltips, week queue) using egui
- Plus Bevy's defaults: `EguiPlugin`, `ShapePlugin` (lyon), `FrameTimeDiagnosticsPlugin`
- `WeekQueue` resource initialized directly in `main.rs` via `.init_resource::<WeekQueue>()`

### Data Flow

```
Input Events
    ↓
InputEvent Messages (MessageWriter/Reader)
    ↓
camera_system processes MoveCamera/ZoomCamera/PanCamera
    ↓
Camera position/scale updated
    ↓
province_picking_system: cursor → camera → world → geographic → spatial query
    ↓
SelectedProvince / HoveredProvince updated
    ↓
update_province_colours / UI systems react to selection/hover state
```

### Key Systems and Their Order

**Startup (chained order matters):**
1. `MapPlugin`: `load_province_map` → `load_towns` → `setup_map_layers` (provinces render)
2. `DomUIPlugin`: `load_ui_assets` → `setup_egui_theme` → `register_ui_textures` (UI ready)
3. Camera setup and player spawning

**Update (per frame):**
- `InputPlugin`: `handle_input` → `handle_mouse_drag` (independent, can run in parallel)
- `EguiPrimaryContextPass`: all egui UI systems (`selected_province_ui`, `player_hover_ui`, `week_queue_ui`) — **must** use this schedule, not `Update`, or the context won't be initialized
- `MapPlugin`:
  - `province_picking_system` — hit-test provinces from cursor
  - `update_province_colours` — update visuals based on hover/select state
  - `camera_system` — process camera input messages
  - `player_hover_system` — detect cursor proximity to player marker
- `DomUIPlugin`: `selected_province_ui`, `player_hover_ui`, `week_queue_ui` (run independently)

## Coordinate Systems

**Critical conversion chain for understanding the codebase:**

```
Geographic (GeoJSON)          Screen/World            NDC              Bevy World
─────────────────────────────────────────────────────────────────────────────────
(lon, lat)                    Vec2                    (-1..1, -1..1)   Vec3
   ↓ geo_to_screen()          ↓ camera                ↓ (ndc calculation
-128°, 50.8°    →    (0, 0)   ↓ viewport_to_world_2d) ↓ in zoom logic)
                         (1400, 900)

Constants (see src/map/mod.rs):
  MAP_ORIGIN_LON: -128.0
  MAP_ORIGIN_LAT: 50.8
  MAP_SCALE: 2000.0 pixels/degree
```

**In code:**
- `geo_to_screen(lon, lat)` — geographic to world coordinates (used for rendering)
- `screen_to_geo(vec2)` — world to geographic (used in picking system)
- `camera.viewport_to_world_2d()` — screen pixels to world coordinates (input handling)

**Camera zoom math** (see `camera_system` in map/mod.rs):
- `transform.scale` controls zoom level (49.0 = fully zoomed out, 5.0 = min zoom)
- Zoom-to-cursor requires NDC conversion: `ndc = (screen_pos / window_size) * 2 - 1` with Y flipped
- World position under cursor: `camera_pos + ndc * (window_size / 2) * scale`

## Important Implementation Details

### Message System (Bevy 0.18.1)

Dominium uses Bevy's `MessageWriter`/`MessageReader` (not the deprecated `EventWriter`):

```rust
// Sender (in systems)
mut events: MessageWriter<InputEvent>
events.write(InputEvent { action: ... })

// Receiver (in systems)
mut reader: MessageReader<InputEvent>
for event in reader.read() { ... }
```

Resources must be initialized: `.add_message::<InputEvent>()` is called in both `InputPlugin` and `MapPlugin`.

### Spatial Queries (Province Picking)

Province picking uses the `geo` crate's geometric algorithms:
1. Cursor position → world space → geographic coordinates
2. Find provinces whose bounding box contains the point (fast early-exit)
3. Test point-in-polygon against the full polygon (exact test)
4. Used in `province_picking_system` and `ProvinceMap::province_at_point()`

### Camera Bounds Clamping

Camera viewport is clamped to map bounds at the end of `camera_system`:
- Map bounds: `(-28000, -13600)` to `(38000, 30400)` in world pixels
- Half-viewport: `(window_size / 2) * scale` (changes with zoom)
- If viewport larger than map: center the camera instead of clamping

### UI Rendering (egui)

**Font loading** (in `setup_egui_theme`):
- Fonts must be loaded directly from filesystem: `std::fs::read("assets/fonts/...")`
- Registered into `egui::FontDefinitions` and set via `ctx.set_fonts()`
- Cannot use Bevy's async asset loading with egui fonts in Startup

**Texture registration** (in `register_ui_textures`):
- Bevy image → `contexts.add_image()` → `egui::TextureId`
- Must happen after image asset is loaded (chained after `load_ui_assets`)

**UI panels** (province info, player tooltip, week queue):
- Province panel: `egui::Area` anchored at RIGHT_BOTTOM, fixed 300×150
- Paints shadow → parchment texture → text overlay using `scope_builder` with `max_rect`
- Player tooltip: follows cursor with `pointer_hover_pos()`, uses `Frame::popup()`
- Week queue panel: fixed position `(100.0, 800.0)`, shows 7-day slots; "Step" advances one day and executes queued action, "Clear" resets the queue

### Action Queue (Week Planning)

`WeekQueue` (in `action_queue/mod.rs`) is a resource holding 7 slots, one per day of the week. Each slot holds a `QueuedAction`:

```rust
enum QueuedAction {
    Travel(province_id),
    Idle,
}
```

The `week_queue_ui` system in `dom_ui/week_queue.rs` renders the queue panel and provides:
- **Step**: advances the current day and executes the queued action (e.g. moves the player to the target province)
- **Clear**: resets all 7 slots to `Idle`

Province selection in `selected_province_ui` can enqueue a `Travel` action for the selected province.

### Input Handling

**Keyboard/scroll** (in `handle_input`):
- Arrow keys or WASD: Move camera (normalized direction × speed)
- Scroll wheel: Zoom (captured via `AccumulatedMouseScroll`)

**Mouse drag** (in `handle_mouse_drag`):
- Left-click + motion: Pan camera
- Uses `Local<bool>` to track drag state across frames
- Motion delta from `AccumulatedMouseMotion`

## Debugging Tips

### Common Issues and Fixes

| Issue | Debug Steps |
|-------|-------------|
| Province doesn't pick | Check geographic coords in `screen_to_geo()`, verify polygon contains point |
| Text not rendering in UI | Ensure fonts chained before textures; verify font file path is correct |
| Camera jerky at boundaries | Check clamp math in `camera_system`; ensure viewport calc accounts for scale |
| Player marker invisible | Verify `spawn_player_marker` runs after `spawn_player` and location matches a province |
| Parchment texture stretched | Check texture size in `allocate_exact_size()` matches custom_size in `Image` |

### Useful Logging

```rust
// In picking_system
eprintln!("Cursor: {:?}, World: {:?}, Geo: ({}, {})", cursor_pos, world_pos, lon, lat);

// In camera_system
eprintln!("Camera scale: {}, bounds check: x∈({}, {})", scale, x_min, x_max);

// In province_picking_system
eprintln!("Testing point ({}, {}) against {} provinces", lon, lat, registry.provinces.len());
```

## Known Constraints and Quirks

- **Window resolution**: Hard-coded to 1400×900 in three places: `main.rs` and two sites in `map/mod.rs` (`camera_system`). All three must be updated together.
- **GeoJSON loading**: Expects specific property names (`AA_ID`, `AA_NAME`, `neighbors` for provinces; `ADMIN_AREA_NAME` for towns)
- **Terrain background**: Rendered as a sprite at world origin; adjust bounds in `setup_map_layers` if map extents change
- **Town rendering**: `setup_towns` exists in `render.rs` but is currently commented out in the startup chain — town data loads but no town entities are spawned
- **egui in Startup**: Cannot use async asset loading; all font files must exist at startup
- **Camera scale vs. projection scale**: Always use `transform.scale`, never `projection.scale` (which is always 1.0)

## Dependencies

- `bevy 0.18.1` — Game engine (with `dynamic_linking` feature for faster iteration)
- `bevy_egui 0.39.1` — Immediate-mode UI
- `bevy_prototype_lyon 0.16.0` — Vector shape rendering (filled/stroked polygons)
- `geo 0.32.0` — Geometric algorithms (point-in-polygon, bounding rects)
- `geojson 1.0.0` — GeoJSON parsing
- `rand 0.10.0` — RNG for province colors

## Dev Profile Optimization

The `[profile.dev]` section applies partial optimization (`opt-level = 1`) and full optimization for dependencies (`opt-level = 3`). This balances fast recompilation (Startup code is not optimized) with reasonable runtime performance.
