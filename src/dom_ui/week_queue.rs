// Render the Week Queue UI, which shows the player's current travel queue

use crate::action_queue::WeekQueue;
use crate::action_queue::*;
use crate::map::ProvinceMap;
use crate::player::{Location, PlayerCharacter};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn week_queue_ui(
    mut contexts: EguiContexts,
    mut queue: ResMut<WeekQueue>,
    mut player: Query<&mut Location, With<PlayerCharacter>>,
    province_map: Res<ProvinceMap>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::Area::new("week_plan".into())
        .fixed_pos(egui::pos2(100.0, 800.0))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Day slots row
                ui.horizontal(|ui| {
                    for (i, action) in queue.days.iter().enumerate() {
                        let is_current = i == queue.current_day;

                        let travel_text = match action {
                            QueuedAction::Idle => "—".to_string(),
                            QueuedAction::Travel { to_province } => province_map
                                .get(*to_province)
                                .map(|p| format!("-> {}", p.name))
                                .unwrap_or("-> ?".to_string()),
                        };

                        let text_color = if is_current {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::WHITE
                        };

                        let response = egui::Frame::new()
                            .fill(if is_current {
                                egui::Color32::from_rgb(80, 60, 30)
                            } else {
                                egui::Color32::from_rgb(50, 40, 25)
                            })
                            .inner_margin(egui::Margin::same(8))
                            .corner_radius(egui::CornerRadius::same(4))
                            .show(ui, |ui| {
                                ui.set_width(80.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("Day {}", i + 1))
                                            .color(text_color),
                                    );
                                    ui.label(egui::RichText::new(&travel_text).color(text_color));
                                });
                            });

                        // response.response.on_hover_text(&travel_text);

                        ui.add_space(4.0);
                    }
                });

                ui.add_space(8.0);

                // Buttons row
                ui.horizontal(|ui| {
                    if ui.button("▶ Step").clicked() {
                        step_queue(&mut queue, &mut player);
                    }

                    if ui.button("✕ Clear").clicked() {
                        *queue = WeekQueue::default();
                    }
                });
            });
        });
}

fn step_queue(queue: &mut WeekQueue, player: &mut Query<&mut Location, With<PlayerCharacter>>) {
    if queue.current_day >= 7 {
        return;
    }

    let action = queue.days[queue.current_day].clone();

    match action {
        QueuedAction::Travel { to_province } => {
            if let Ok(mut location) = player.single_mut() {
                location.province_id = to_province;
            }
        }
        QueuedAction::Idle => {}
    }

    queue.current_day += 1;

    // Reset when week is done
    if queue.current_day >= 7 {
        *queue = WeekQueue::default();
    }
}
