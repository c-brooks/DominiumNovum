use crate::{buildings::BuildingKind, dom_ui::assets::BuildingTextureIds};
use bevy_egui::egui;

pub fn building_icon(
    ui: &mut egui::Ui,
    kind: BuildingKind,
    size: f32,
    textures: &BuildingTextureIds,
) -> egui::Response {
    let color = match kind {
        BuildingKind::Farm => egui::Color32::from_rgb(100, 180, 80),
        BuildingKind::Mine => egui::Color32::from_rgb(60, 60, 60),
        BuildingKind::Sawmill => egui::Color32::from_rgb(150, 100, 50),
        BuildingKind::Barracks => egui::Color32::from_rgb(180, 60, 60),
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        if let Some(&id) = textures.0.get(&kind) {
            egui::Image::new(egui::load::SizedTexture::new(id, rect.size())).paint_at(ui, rect);
        } else {
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::same(4), color);
        }
    }
    response
}
