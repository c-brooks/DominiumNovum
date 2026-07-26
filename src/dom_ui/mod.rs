use bevy::app::App;
use bevy::prelude::*;

use crate::action_queue::WeekQueue;
use crate::buildings::Building;
use crate::buildings::BuildingRegistry;
use crate::buildings::ProvinceBuildingIndex;
use crate::dom_ui::assets::BuildingTextureIds;
use crate::dom_ui::assets::apply_parchment_theme;
use crate::dom_ui::assets::load_ui_assets;
use crate::dom_ui::assets::register_ui_textures;
use crate::dom_ui::clock_ui::clock_ui;
use crate::dom_ui::week_queue::{execute_queued_action, week_queue_ui};
use crate::map::MapLoadState;
use crate::map::ProvinceMap;
use crate::map::province::SelectedProvince;
use crate::player::CharacterIdentity;
use crate::player::HoveredPlayer;
use crate::player::Location;
use crate::player::PlayerCharacter;
use crate::ticker::DailyTickSet;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

pub mod assets;
pub mod buildings;
pub mod clock_ui;
pub mod week_queue;

pub struct DomUIPlugin;

impl Plugin for DomUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (load_ui_assets, setup_egui_theme, register_ui_textures).chain(),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (
                selected_province_ui.run_if(in_state(MapLoadState::Ready)),
                player_hover_ui,
                week_queue_ui.run_if(in_state(MapLoadState::Ready)),
                clock_ui,
            ),
        )
        .add_systems(
            Update,
            execute_queued_action.in_set(DailyTickSet::Production),
        );
    }
}

pub fn setup_egui_theme(mut contexts: EguiContexts) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let regular = include_bytes!("../../assets/fonts/IMFellEnglish-Regular.ttf");

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "IMFellEnglish".to_string(),
        std::sync::Arc::new(egui::FontData::from_static(regular)),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "IMFellEnglish".to_string());

    println!("Fonts registered: {:?}", fonts.families);
    apply_parchment_theme(ctx, fonts);
}

// * Selected province info panel and player hover tooltip are implemented here.
// * When a province is selected, an info panel appears in the bottom right showing the province name and ID.
// * From this panel, you can select "travel here" to queue a travel action for the player character.
pub fn selected_province_ui(
    mut contexts: EguiContexts,
    texture_ids: Res<assets::UiTextureIds>,
    selected: Res<SelectedProvince>,
    province_map: Res<ProvinceMap>,
    buildings: Res<ProvinceBuildingIndex>,
    building_query: Query<&Building>,
    building_registry: Res<BuildingRegistry>,
    building_texture_ids: Res<BuildingTextureIds>,
    mut week_queue: ResMut<WeekQueue>,
    player_location: Query<&Location, With<PlayerCharacter>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    if let Some(province_id) = selected.0 {
        if let Some(province) = province_map.get(province_id) {
            egui::Area::new("province_info".into())
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
                .show(ctx, |ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(300.0, 200.0), egui::Sense::hover());

                    // Shadow behind the panel
                    ui.painter().add(
                        egui::Shadow {
                            offset: [4, 6],
                            blur: 8,
                            spread: 4,
                            color: egui::Color32::from_black_alpha(120),
                        }
                        .as_shape(rect, egui::CornerRadius::same(20)),
                    );

                    // Paint parchment texture with rounded corners as background
                    egui::Image::new(egui::load::SizedTexture::new(
                        texture_ids.parchment,
                        rect.size(),
                    ))
                    .corner_radius(egui::CornerRadius::same(20))
                    .paint_at(ui, rect);

                    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(&province.name)
                                    .font(egui::FontId::proportional(22.0))
                                    .size(22.0)
                                    .strong(),
                            );
                            ui.label(format!("ID: {}", province_id));
                            if ui.button("Travel").clicked() {
                                if let Ok(location) = player_location.single() {
                                    week_queue.queue_travel(
                                        location.province_id,
                                        province_id,
                                        &province_map,
                                    );
                                }
                            }
                        });
                    });
                    if let Some(building_entities) = buildings.buildings.get(&province_id) {
                        ui.add_space(12.0);
                        ui.label(
                            egui::RichText::new("Building:")
                                .font(egui::FontId::proportional(18.0))
                                .size(18.0)
                                .strong(),
                        );
                        for entity in building_entities {
                            if let Ok(building) = building_query.get(*entity) {
                                if let Some(def) = building_registry.0.get(&building.kind) {
                                    ui.horizontal(|ui| {
                                        buildings::building_icon(
                                            ui,
                                            building.kind,
                                            32.0,
                                            &building_texture_ids,
                                        );
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{} Lv. {}",
                                                def.name, building.level
                                            ))
                                            .color(egui::Color32::BLACK),
                                        );
                                    });
                                }
                            }
                        }
                    }
                });
        }
    }
}

pub fn player_hover_ui(
    mut contexts: EguiContexts,
    hovered: Option<Res<HoveredPlayer>>,
    identities: Query<&CharacterIdentity, With<PlayerCharacter>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let Some(hovered) = hovered else {
        return;
    };
    let Some(_) = hovered.0 else {
        return;
    };
    let Ok(identity) = identities.single() else {
        return;
    };

    if let Some(pointer_pos) = ctx.pointer_hover_pos() {
        egui::Area::new(egui::Id::new("player_tooltip"))
            .fixed_pos(pointer_pos + egui::vec2(12.0, 12.0))
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.label(egui::RichText::new(&identity.name).strong());
                    ui.label(format!("Age: {}", identity.age));
                });
            });
    }
}
