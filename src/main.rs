// src/main.rs
mod dom_ui;
mod inputevents;
mod map;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use inputevents::*;
use map::MapPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dominium Novum".into(),
                    resolution: (1400u32, 900u32).into(),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            ShapePlugin,           // bevy_prototype_lyon
            EguiPlugin::default(), // bevy_egui
            InputPlugin,
            MapPlugin, // our map plugin
        ))
        .add_systems(
            Startup,
            (
                map::setup_camera,
                dom_ui::assets::load_ui_assets,
                dom_ui::setup_egui_theme,
                dom_ui::assets::register_ui_textures,
                dom_ui::assets::register_ui_fonts,
            )
                .chain(),
        )
        .add_systems(Update, map::camera_system)
        .run()
}
