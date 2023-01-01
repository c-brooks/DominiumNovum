// src/main.rs
mod action_queue;
mod buildings;
mod characters;
mod dom_ui;
mod inputevents;
mod map;
mod player;
mod ticker;

use action_queue::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use buildings::BuildingsPlugin;
use characters::CharactersPlugin;
use dom_ui::DomUIPlugin;
use inputevents::*;
use map::MapPlugin;
use ticker::TickerPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dominium Novum".into(),
                    resolution: (1400u32, 900u32).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    // Without this, winit creates its own <canvas> and appends
                    // it to <body> instead of reusing the one already in
                    // web/index.html — leaving the original empty canvas
                    // stacked above the real one (a full screen-height of
                    // blank space).
                    canvas: Some("#bevy".to_string()),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            ShapePlugin,           // bevy_prototype_lyon
            EguiPlugin::default(), // bevy_egui
            InputPlugin,
            MapPlugin,
            DomUIPlugin,
            BuildingsPlugin,
            TickerPlugin,
            CharactersPlugin,
        ))
        .init_resource::<WeekQueue>()
        .add_systems(Startup, (map::setup_camera, player::spawn_player).chain())
        // Runs once political.geojson/towns.geojson have loaded and ProvinceMap exists.
        .add_systems(OnEnter(map::MapLoadState::Ready), player::spawn_player_marker)
        .add_systems(Update, map::camera_system)
        .run()
}
