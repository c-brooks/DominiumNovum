// Render the Clock (date) UI,

use crate::ticker::clock::GameClock;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn clock_ui(mut contexts: EguiContexts, clock: Res<GameClock>) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::Area::new("clock".into())
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Day slots row
                ui.horizontal(|ui| {
                    ui.label(format!("Year {}", clock.year()));
                    ui.label(format!("Month {}", clock.month()));
                    ui.label(format!("Day {}", clock.day));
                });
            });
        });
}
