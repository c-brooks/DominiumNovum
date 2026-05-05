// Render the Week Queue UI, which shows the player's current travel queue

use crate::action_queue::WeekQueue;
use crate::action_queue::*;
use crate::map::ProvinceMap;
use crate::player::{Location, PlayerCharacter};
use crate::ticker::clock::{DayEnded, ExecutionMode, GameClock, WeekEnded};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn week_queue_ui(
    mut contexts: EguiContexts,
    mut queue: ResMut<WeekQueue>,
    mut clock: ResMut<GameClock>,
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
                            QueuedAction::Travel {
                                to_province,
                                arriving: _,
                            } => province_map
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
                    if ui.button("▶ Play").clicked() {
                        clock.execution_mode = ExecutionMode::Playing;
                    }
                    if ui.button("-> Step").clicked() {
                        clock.execution_mode = ExecutionMode::Stepping;
                    }

                    if ui.button("✕ Clear").clicked() {
                        *queue = WeekQueue::default();
                    }
                });
            });
        });
}

pub fn execute_queued_action(
    mut day_reader: MessageReader<DayEnded>,
    mut week_reader: MessageReader<WeekEnded>,
    mut queue: ResMut<WeekQueue>,
    mut player: Query<&mut Location, With<PlayerCharacter>>,
) {
    for event in day_reader.read() {
        let day_idx = event.day_of_week as usize;
        if day_idx >= queue.days.len() {
            continue;
        }

        queue.current_day = day_idx;

        let action = queue.days[day_idx].clone();
        match action {
            QueuedAction::Travel {
                to_province,
                arriving: true,
            } => {
                if let Ok(mut location) = player.single_mut() {
                    location.province_id = to_province;
                }
            }
            _ => {}
        }
    }

    for _ in week_reader.read() {
        *queue = WeekQueue::default();
    }
}
