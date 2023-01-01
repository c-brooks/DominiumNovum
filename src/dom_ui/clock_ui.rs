// Render the Clock (date) UI,

use crate::ticker::clock::GameClock;

use crate::dom_ui::assets::UiTextureIds;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn clock_ui(mut contexts: EguiContexts, texture_ids: Res<UiTextureIds>, clock: Res<GameClock>) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::Area::new("clock".into())
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(200.0, 50.0), egui::Sense::hover());
            // Paint parchment texture with rounded corners as background
            egui::Image::new(egui::load::SizedTexture::new(
                texture_ids.parchment,
                rect.size(),
            ))
            .corner_radius(egui::CornerRadius::same(20))
            .paint_at(ui, rect);

            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Year {} · Month {} · Day {}",
                            clock.year(),
                            clock.month(),
                            clock.day
                        ))
                        .color(egui::Color32::BLACK),
                    );
                });
            });
        });
}
