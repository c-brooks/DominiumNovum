use bevy::app::App;
use bevy::prelude::*;
use bevy::*;

use crate::dom_ui::assets::apply_parchment_theme;
use crate::map::ProvinceMap;
use crate::map::province::SelectedProvince;
use bevy_egui::{EguiContexts, egui};

pub mod assets;

pub fn setup_egui_theme(mut contexts: EguiContexts) {
    println!("Applying parchment theme to egui...");
    if let Ok(ctx) = contexts.ctx_mut() {
        apply_parchment_theme(ctx);
        println!("Parchment theme applied")
    }
}

pub fn selected_province_ui(
    mut contexts: EguiContexts,
    texture_ids: Res<assets::UiTextureIds>,
    selected: Res<SelectedProvince>,
    province_map: Res<ProvinceMap>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    if let Some(province_id) = selected.0 {
        if let Some(province) = province_map.get(province_id) {
            egui::Area::new("province_info".into())
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
                .show(ctx, |ui| {
                    egui::Frame {
                        corner_radius: egui::CornerRadius::same(20),
                        inner_margin: egui::Margin::symmetric(12, 12),
                        ..Default::default()
                    }
                    .show(ui, |ui| {
                        let rect = ui.max_rect();
                        ui.painter().image(
                            texture_ids.parchment,
                            rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new(&province.name).size(22.0).strong());
                            ui.label(format!("ID: {}", province.id));
                            ui.set_width(500.0);
                            ui.set_height(150.0);
                        })
                    });
                });
        }
    }
}
