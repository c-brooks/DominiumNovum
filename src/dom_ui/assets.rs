use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct UiAssets {
    pub parchment: Handle<Image>,
}

#[derive(Resource)]
pub struct UiTextureIds {
    pub parchment: egui::TextureId,
}

pub fn load_ui_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        parchment: asset_server.load("parchment.png"),
    });
}

pub fn register_ui_fonts(mut contexts: EguiContexts) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let regular = std::fs::read("assets/fonts/IMFellEnglish-Regular.ttf")
        .expect("Failed to read IMFellEnglish-Regular.ttf");
    let italic = std::fs::read("assets/fonts/IMFellEnglish-Italic.ttf")
        .expect("Failed to read IMFellEnglish-Italic.ttf");

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "IMFellEnglish-Regular".to_string(),
        egui::FontData::from_owned(regular).into(),
    );
    fonts.font_data.insert(
        "IMFellEnglish-Italic".to_string(),
        egui::FontData::from_owned(italic).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "IMFellEnglish-Regular".to_string());

    ctx.set_fonts(fonts);
}

pub fn register_ui_textures(
    mut contexts: EguiContexts,
    ui_assets: Res<UiAssets>,
    mut commands: Commands,
) {
    let parchment = contexts.add_image(bevy_egui::EguiTextureHandle::Strong(
        ui_assets.parchment.clone(),
    ));
    commands.insert_resource(UiTextureIds { parchment });
}

pub fn apply_parchment_theme(ctx: &egui::Context) {
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

    ctx.set_visuals(visuals);
}
