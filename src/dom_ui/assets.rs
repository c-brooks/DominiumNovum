use bevy::platform::collections::HashMap;
use bevy::prelude::*;
// Removed incorrect import: use bevy::text::FontSource;
use bevy::text::*;
use bevy_egui::EguiTextureHandle;
use bevy_egui::{EguiContexts, egui};

use crate::buildings::BuildingKind;

#[derive(Resource)]
pub struct UiAssets {
    pub parchment: Handle<Image>,
}

#[derive(Resource)]
pub struct UiTextureIds {
    pub parchment: egui::TextureId,
}

#[derive(Resource)]
pub struct BuildingTextureIds(pub HashMap<BuildingKind, egui::TextureId>);

#[derive(Resource)]
pub struct BuildingIconAssets(pub HashMap<BuildingKind, Handle<Image>>);

pub fn load_ui_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut icons = HashMap::new();
    icons.insert(BuildingKind::Farm, asset_server.load("buildings/farm.png"));
    icons.insert(BuildingKind::Mine, asset_server.load("buildings/mine.png"));
    icons.insert(
        BuildingKind::Sawmill,
        asset_server.load("buildings/sawmill.png"),
    );
    icons.insert(
        BuildingKind::Barracks,
        asset_server.load("buildings/barracks.png"),
    );
    commands.insert_resource(BuildingIconAssets(icons));

    commands.insert_resource(UiAssets {
        parchment: asset_server.load("parchment.png"),
    });
}

pub fn register_ui_textures(
    mut contexts: EguiContexts,
    ui_assets: Res<UiAssets>,
    building_icons: Res<BuildingIconAssets>,
    mut commands: Commands,
) {
    let parchment = contexts.add_image(EguiTextureHandle::Strong(ui_assets.parchment.clone()));
    commands.insert_resource(UiTextureIds { parchment });

    let ids = building_icons
        .0
        .iter()
        .map(|(&kind, handle)| {
            let id = contexts.add_image(EguiTextureHandle::Strong(handle.clone()));
            (kind, id)
        })
        .collect();
    commands.insert_resource(BuildingTextureIds(ids));
}

pub fn apply_parchment_theme(ctx: &egui::Context, font_definitions: egui::FontDefinitions) {
    let mut visuals = egui::Visuals::light();

    visuals.panel_fill = egui::Color32::TRANSPARENT;
    visuals.window_fill = egui::Color32::TRANSPARENT;

    visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(80, 55, 30, 40);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(80, 55, 30, 80);

    visuals.override_text_color = Some(egui::Color32::from_rgb(45, 30, 15));

    visuals.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(1.5, egui::Color32::from_rgb(80, 55, 30));
    visuals.window_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 55, 30));
    visuals.window_corner_radius = egui::CornerRadius::same(20);

    ctx.set_fonts(font_definitions);
    ctx.set_visuals(visuals);
}
