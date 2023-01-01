use bevy::ecs::entity::Entity;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::buildings::{Building, BuildingRegistry};
use crate::characters::Character;
use crate::dom_ui::assets::BuildingTextureIds;
use crate::dom_ui::assets::{CharacterPortraitIds, UiTextureIds};
use crate::dom_ui::buildings as building_icons;

#[derive(Resource, Default)]
pub struct SelectedBuilding(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct CharacterPickerOpen(pub bool);

pub fn building_detail_ui(
    mut contexts: EguiContexts,
    texture_ids: Res<UiTextureIds>,
    portrait_ids: Res<CharacterPortraitIds>,
    building_texture_ids: Res<BuildingTextureIds>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut picker_open: ResMut<CharacterPickerOpen>,
    building_query: Query<&Building>,
    building_registry: Res<BuildingRegistry>,
    character_query: Query<&Character>,
) {
    let Some(entity) = selected_building.0 else {
        return;
    };
    let Ok(building) = building_query.get(entity) else {
        return;
    };
    let Some(def) = building_registry.0.get(&building.kind) else {
        return;
    };

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Area::new("building_detail".into())
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -220.0))
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(300.0, 160.0), egui::Sense::hover());

            ui.painter().add(
                egui::Shadow {
                    offset: [4, 6],
                    blur: 8,
                    spread: 4,
                    color: egui::Color32::from_black_alpha(120),
                }
                .as_shape(rect, egui::CornerRadius::same(20)),
            );

            egui::Image::new(egui::load::SizedTexture::new(
                texture_ids.parchment,
                rect.size(),
            ))
            .corner_radius(egui::CornerRadius::same(20))
            .paint_at(ui, rect);

            ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(12.0)), |ui| {
                ui.add_space(8.0);

                // Header: icon + name/level + close button
                ui.horizontal(|ui| {
                    building_icons::building_icon(ui, building.kind, 32.0, &building_texture_ids);
                    ui.label(
                        egui::RichText::new(format!("{} Lv. {}", def.name, building.level))
                            .color(egui::Color32::BLACK)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("✕").clicked() {
                            selected_building.0 = None;
                            picker_open.0 = false;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.label(egui::RichText::new("Manager:").color(egui::Color32::BLACK));
                ui.add_space(4.0);

                // Manager portrait slot
                let slot_response = ui.horizontal(|ui| {
                    let slot_size = egui::vec2(64.0, 64.0);
                    match building.manager {
                        Some(manager_entity) => {
                            if let Ok(character) = character_query.get(manager_entity) {
                                let (slot_rect, response) =
                                    ui.allocate_exact_size(slot_size, egui::Sense::click());
                                if ui.is_rect_visible(slot_rect) {
                                    if let Some(&tex_id) = portrait_ids.0.get(&character.portrait) {
                                        egui::Image::new(egui::load::SizedTexture::new(
                                            tex_id,
                                            slot_rect.size(),
                                        ))
                                        .corner_radius(egui::CornerRadius::same(4))
                                        .paint_at(ui, slot_rect);
                                    } else {
                                        ui.painter().rect_filled(
                                            slot_rect,
                                            egui::CornerRadius::same(4),
                                            egui::Color32::from_rgb(100, 90, 70),
                                        );
                                    }
                                }
                                ui.label(
                                    egui::RichText::new(&character.name)
                                        .color(egui::Color32::BLACK),
                                );
                                response
                            } else {
                                empty_slot(ui, slot_size)
                            }
                        }
                        None => empty_slot(ui, slot_size),
                    }
                });

                if slot_response.inner.clicked() {
                    picker_open.0 = true;
                }
            });
        });
}

fn empty_slot(ui: &mut egui::Ui, size: egui::Vec2) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(4),
            egui::Stroke::new(1.5, egui::Color32::from_rgb(120, 100, 70)),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "+",
            egui::FontId::proportional(24.0),
            egui::Color32::from_rgb(120, 100, 70),
        );
    }
    response
}

pub fn character_picker_ui(
    mut contexts: EguiContexts,
    portrait_ids: Res<CharacterPortraitIds>,
    mut picker_open: ResMut<CharacterPickerOpen>,
    selected_building: Res<SelectedBuilding>,
    building_registry: Res<BuildingRegistry>,
    character_query: Query<(Entity, &Character)>,
    mut building_query: Query<&mut Building>,
) {
    if !picker_open.0 {
        return;
    }
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Build map: character entity → name of the building they already manage
    let assigned: std::collections::HashMap<Entity, &str> = building_query
        .iter()
        .filter_map(|b| {
            let manager = b.manager?;
            let name = building_registry.0.get(&b.kind).map(|d| d.name)?;
            Some((manager, name))
        })
        .collect();

    let mut open = picker_open.0;
    egui::Window::new("Assign Manager")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .collapsible(false)
        .open(&mut open)
        .show(ctx, |ui| {
            for (char_entity, character) in &character_query {
                let already_managing = assigned.get(&char_entity).copied();
                let dimmed = egui::Color32::LIGHT_GRAY;
                // let dimmed = egui::Color32::from_rgba_unmultiplied(80, 65, 45, 160);
                let text_color = if already_managing.is_some() {
                    dimmed
                } else {
                    egui::Color32::WHITE
                };

                let row = ui.horizontal(|ui| {
                    let portrait_size = egui::vec2(48.0, 48.0);
                    let (rect, _) = ui.allocate_exact_size(portrait_size, egui::Sense::hover());
                    if ui.is_rect_visible(rect) {
                        if let Some(&tex_id) = portrait_ids.0.get(&character.portrait) {
                            let mut img = egui::Image::new(egui::load::SizedTexture::new(
                                tex_id,
                                rect.size(),
                            ))
                            .corner_radius(egui::CornerRadius::same(4));
                            if already_managing.is_some() {
                                img = img.tint(egui::Color32::from_rgba_unmultiplied(
                                    255, 255, 255, 120,
                                ));
                            }
                            img.paint_at(ui, rect);
                        } else {
                            ui.painter()
                                .rect_filled(rect, egui::CornerRadius::same(4), dimmed);
                        }
                    }
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&character.name)
                                .strong()
                                .color(text_color),
                        );
                        ui.label(
                            egui::RichText::new(format!("Age: {}", character.age))
                                .color(text_color),
                        );
                    });
                });

                let row_interact = row.response.interact(egui::Sense::click());

                if let Some(building_name) = already_managing {
                    row_interact.on_hover_text(format!("Already managing {}", building_name));
                } else if row_interact.clicked() {
                    if let Some(building_entity) = selected_building.0 {
                        if let Ok(mut building) = building_query.get_mut(building_entity) {
                            building.manager = Some(char_entity);
                        }
                    }
                    picker_open.0 = false;
                    return;
                }

                ui.separator();
            }

            ui.add_space(4.0);
            if ui.button("Cancel").clicked() {
                picker_open.0 = false;
            }
        });

    if !open {
        picker_open.0 = false;
    }
}
