// src/main.rs
mod action_queue;
mod dom_ui;
mod inputevents;
mod map;
mod player;

use action_queue::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use dom_ui::DomUIPlugin;
use inputevents::*;
use map::MapPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dominium Novum".into(),
                    resolution: (1400u32, 900u32).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
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
        ))
        .init_resource::<WeekQueue>()
        .add_systems(
            Startup,
            (
                map::setup_camera,
                player::spawn_player,
                player::spawn_player_marker,
            )
                .chain(),
        )
        .add_systems(Update, map::camera_system)
        .run()
}
