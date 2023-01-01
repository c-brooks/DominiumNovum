use crate::map::TownDef;
use crate::map::TownMarker;
use crate::map::town::TownMap;

// src/map/render.rs
use super::province::ProvinceShape;
use super::{ProvinceMap, geo_to_screen};
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy_prototype_lyon::geometry::ShapeBuilderBase;
use bevy_prototype_lyon::prelude::Shape;
use bevy_prototype_lyon::prelude::ShapeBuilder;

use bevy_prototype_lyon::prelude::Fill;
use bevy_prototype_lyon::shapes;

use bevy::color::palettes::css::*;
use rand::RngExt;

#[derive(Component, Clone)]
pub struct ProvinceColor(pub Color);

pub fn setup_map_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    province_map: Res<ProvinceMap>,
) {
    let mut rng = rand::rng();

    let west = -142_f64;
    let east = -109_f64;
    let north = 66_f64;
    let south = 44_f64;

    let top_left = geo_to_screen(west, north);
    let bottom_right = geo_to_screen(east, south);

    let width = bottom_right.x - top_left.x;
    let height = top_left.y - bottom_right.y;
    let center = Vec2::new(
        (top_left.x + bottom_right.x) / 2.0,
        (top_left.y + bottom_right.y) / 2.0,
    );

    // Ocean background — lowest layer
    commands.spawn((
        Sprite {
            image: asset_server.load_with_settings(
                "terrain.png",
                |settings: &mut ImageLoaderSettings| {
                    settings
                        .sampler
                        .get_or_init_descriptor()
                        .set_filter(bevy::image::ImageFilterMode::Linear);
                },
            ),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, 0.0),
        TerrainLayer,
    ));

    // Terrain texture — fades in on zoom
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("terrain.png"),
    //         transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(4096.0, 4096.0)),
    //             color: Color::rgba(1.0, 1.0, 1.0, 0.0), // start invisible
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     TerrainLayer,
    // ));

    // Spawn a province entity for each province
    for province in &province_map.provinces {
        let points: Vec<Vec2> = province
            .polygon
            .exterior()
            .points()
            .map(|p| geo_to_screen(p.x(), p.y()))
            .collect();

        let shape = shapes::Polygon {
            points: points.clone(),
            closed: true,
        };

        let initial_color = Color::srgba(
            rng.random_range(0.2..0.9),
            rng.random_range(0.2..0.9),
            rng.random_range(0.2..0.9),
            0.8,
        );

        commands.spawn((
            ShapeBuilder::with(&shape)
                .fill(Fill::color(initial_color))
                .stroke((BLACK, 10.0))
                .build(),
            ProvinceShape {
                id: province.id,
                base_colour: initial_color,
            },
            ProvinceColor(initial_color),
        ));
    }
}

pub fn setup_towns(mut commands: Commands, town_map: Res<TownMap>) {
    for town in &town_map.towns {
        let screen_pos = geo_to_screen(town.centroid.0, town.centroid.1);

        let circle = shapes::Circle {
            radius: 50.0,
            center: Vec2::ZERO,
        };

        commands.spawn((
            ShapeBuilder::with(&circle)
                .fill(Fill::color(Color::srgb(0.9, 0.9, 0.7)))
                .stroke((BLACK, 1.5))
                .build(),
            Transform::from_xyz(screen_pos.x, screen_pos.y, 3.0), // above provinces
            TownMarker {
                id: town.id,
                name: town.name.clone(),
            },
        ));
    }
}

#[derive(Component)]
pub struct TerrainLayer;

fn lighten(color: Color, amount: f32) -> Color {
    let LinearRgba {
        red,
        green,
        blue,
        alpha,
    } = color.to_linear();
    Color::linear_rgba(
        (red + amount).min(1.0),
        (green + amount).min(1.0),
        (blue + amount).min(1.0),
        1.0,
    )
}

// Update province fill colors based on game state
pub fn update_province_colours(
    selected: Res<SelectedProvince>,
    hovered: Res<HoveredProvince>,
    mut query: Query<(&ProvinceShape, &mut Shape)>,
    mut prev: Local<(Option<u32>, Option<u32>)>,
) {
    let current = (hovered.0, selected.0);
    if *prev == current {
        return;
    }

    // The IDs whose visual state may have changed
    let affected = [prev.0, prev.1, current.0, current.1];
    *prev = current;

    for (province, mut shape) in query.iter_mut() {
        if !affected.contains(&Some(province.id)) {
            continue;
        }

        let new_colour = if Some(province.id) == selected.0 {
            Color::srgba(0.9, 0.7, 0.1, 0.8)
        } else if Some(province.id) == hovered.0 {
            lighten(province.base_colour, 0.3)
        } else {
            province.base_colour
        };

        shape.fill = Some(Fill::color(new_colour));
    }
}

use super::province::{HoveredProvince, SelectedProvince};
