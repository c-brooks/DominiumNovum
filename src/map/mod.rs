pub mod picking;
pub mod province;
pub mod render;
pub mod town;

pub use province::ProvinceMap;

use crate::dom_ui::*;
use crate::inputevents::{InputAction, InputEvent};
use crate::map::render::setup_map_layers;
use crate::map::render::setup_towns;
use crate::player;
use bevy::prelude::Camera2d;
use bevy::prelude::*;
use geo::{BoundingRect, Centroid, LineString, Polygon};
use geojson::{Feature, GeoJson, Geometry, GeometryValue};
use province::*;
use town::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedProvince>()
            .init_resource::<HoveredProvince>()
            .init_resource::<player::HoveredPlayer>()
            .add_systems(
                Startup,
                (
                    load_province_map,
                    load_towns,
                    setup_map_layers,
                    // setup_towns
                )
                    .chain(),
            ) // chain ensures load runs before setup
            .add_systems(
                Update,
                (
                    picking::province_picking_system,
                    render::update_province_colours,
                    player::player_hover_system,
                    player::update_player_marker,
                ),
            );
    }
}

pub fn load_towns(mut commands: Commands) {
    let geojson_str = include_str!("../../assets/towns.geojson");
    let geojson: GeoJson = geojson_str.parse().expect("Could not parse towns.geojson");

    let feature_collection = match geojson {
        GeoJson::FeatureCollection(fc) => fc,
        _ => panic!("Expected a FeatureCollection"),
    };

    let towns: Vec<TownDef> = feature_collection
        .features
        .into_iter()
        .enumerate()
        .filter_map(|(i, f)| parse_town_feature(f, i as u32))
        .collect();

    println!("Loaded {} towns", towns.len());
    commands.insert_resource(TownMap { towns });
}

#[derive(Component)]
pub struct TownMarker {
    pub id: u32,
    pub name: String,
}

fn parse_town_feature(feature: Feature, id: u32) -> Option<TownDef> {
    let props = feature.properties.as_ref()?;
    let name = props.get("ADMIN_AREA_NAME")?.as_str()?.to_string();

    // println!("Parsing town: {}", name);
    // println!("Properties: {:?}", props);

    let geometry = feature.geometry?;
    let (lon, lat) = match geometry.value {
        GeometryValue::Point { coordinates } => (coordinates[0], coordinates[1]),
        _ => {
            println!("Unexpected geometry type for town — skipping");
            return None;
        }
    };

    Some(TownDef {
        id,
        name,
        centroid: (lon, lat),
        province_id: 0,
    })
}

pub fn load_province_map(mut commands: Commands) {
    let geojson_str = include_str!("../../assets/political.geojson");

    let geojson: GeoJson = geojson_str
        .parse()
        .expect("Could not parse provinces.geojson");

    let feature_collection = match geojson {
        GeoJson::FeatureCollection(fc) => fc,
        _ => panic!("Expected a FeatureCollection"),
    };

    let mut provinces = Vec::new();

    for feature in feature_collection.features {
        if let Some(province) = parse_province_feature(feature) {
            provinces.push(province);
        }
    }

    compute_neighbors(&mut provinces);
    println!("Loaded {} provinces", provinces.len());
    commands.insert_resource(ProvinceMap { provinces });
}

// After all provinces are loaded, compute neighbors by finding provinces that share
// boundary vertices. QGIS exports use exact shared coordinates along common borders,
// so this is reliable and runs in O(N×V) — one pass to build the vertex map, one to read it.
fn compute_neighbors(provinces: &mut Vec<ProvinceDef>) {
    use std::collections::HashMap;

    // Round to 5 decimal places (~1m precision) to key on shared vertices.
    let quantize = |v: f64| (v * 100_000.0).round() as i64;

    // Map each vertex → list of province IDs whose boundary passes through it.
    let mut vertex_to_provinces: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
    for province in provinces.iter() {
        for coord in province.polygon.exterior().coords() {
            let key = (quantize(coord.x), quantize(coord.y));
            vertex_to_provinces
                .entry(key)
                .or_default()
                .push(province.id);
        }
    }

    // Build adjacency: provinces sharing ≥1 vertex are neighbors.
    // Use a HashSet per province to avoid duplicate edges.
    let mut adjacency: HashMap<u32, std::collections::HashSet<u32>> = HashMap::new();
    for province_ids in vertex_to_provinces.values() {
        if province_ids.len() < 2 {
            continue;
        }
        for i in 0..province_ids.len() {
            for j in (i + 1)..province_ids.len() {
                let a = province_ids[i];
                let b = province_ids[j];
                adjacency.entry(a).or_default().insert(b);
                adjacency.entry(b).or_default().insert(a);
            }
        }
    }

    for province in provinces.iter_mut() {
        if let Some(neighbors) = adjacency.remove(&province.id) {
            province.neighbors = neighbors.into_iter().collect();
        }
    }
}

fn parse_province_feature(feature: Feature) -> Option<ProvinceDef> {
    // Extract properties
    let props = feature.properties.as_ref()?;

    let id = props.get("AA_ID")?.as_u64()? as u32;

    let name = props.get("AA_NAME")?.as_str()?.to_string();

    // Extract polygon geometry
    let geometry = feature.geometry?;
    let polygon = parse_polygon(geometry)?;

    // Calculate centroid for label placement
    let centroid = polygon
        .centroid()
        .map(|c| (c.x(), c.y()))
        .unwrap_or((0.0, 0.0));

    let bbox = polygon.bounding_rect()?;

    Some(ProvinceDef {
        id,
        name,
        neighbors: vec![],
        bbox,
        polygon,
        centroid,
        base_colour: Color::WHITE, // default color, will be randomized in render setup
        travel_days: 1,
    })
}

fn parse_polygon(geometry: Geometry) -> Option<Polygon<f64>> {
    match geometry.value {
        GeometryValue::Polygon {
            coordinates: coords,
        } => {
            let exterior: Vec<(f64, f64)> = coords[0].iter().map(|pos| (pos[0], pos[1])).collect();

            let exterior_ring = LineString::from(exterior);

            // Interior rings (holes) — unlikely for provinces but handle it
            let interior_rings: Vec<LineString<f64>> = coords[1..]
                .iter()
                .map(|ring| {
                    LineString::from(ring.iter().map(|pos| (pos[0], pos[1])).collect::<Vec<_>>())
                })
                .collect();

            Some(Polygon::new(exterior_ring, interior_rings))
        }
        // Some QGIS exports wrap single polygons in MultiPolygon
        GeometryValue::MultiPolygon { coordinates: multi } => {
            let first = multi.into_iter().next()?;
            let exterior = LineString::from(
                first[0]
                    .iter()
                    .map(|pos| (pos[0], pos[1]))
                    .collect::<Vec<_>>(),
            );
            Some(Polygon::new(exterior, vec![]))
        }
        _ => {
            println!("Unexpected geometry type — skipping");
            None
        }
    }
}

// Tuned for Vancouver Island
// Adjust these values to frame your map correctly
pub const MAP_ORIGIN_LON: f64 = -128.0; // left edge
pub const MAP_ORIGIN_LAT: f64 = 50.8; // top edge
pub const MAP_SCALE: f64 = 2000.0; // pixels per degree — tune this

pub fn geo_to_screen(lon: f64, lat: f64) -> Vec2 {
    Vec2::new(
        ((lon - MAP_ORIGIN_LON) * MAP_SCALE) as f32,
        // latitude is flipped — north is up in geo, down in screen space
        ((lat - MAP_ORIGIN_LAT) * MAP_SCALE) as f32,
    )
}

pub fn screen_to_geo(pos: Vec2) -> (f64, f64) {
    let lon = pos.x as f64 / MAP_SCALE + MAP_ORIGIN_LON;
    let lat = (pos.y as f64 / MAP_SCALE) + MAP_ORIGIN_LAT;
    (lon, lat)
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(700.0, 450.0, 999.0),
            scale: Vec3::splat(49.0), // Max zoom out = background fills screen
            ..Default::default()
        },
    ));
}

pub fn camera_system(
    camera_q: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    mut input_reader: MessageReader<InputEvent>,
) {
    let (mut camera, mut transform, mut projection) = camera_q.into_inner();

    for event in input_reader.read() {
        match &event.action {
            InputAction::MoveCamera { direction } => {
                transform.translation.x += direction.x * transform.scale.x;
                transform.translation.y += direction.y * transform.scale.y;
            }
            InputAction::ZoomCamera { delta, centre } => {
                let window_size = Vec2::new(1400.0, 900.0);
                let current_scale = transform.scale.x;

                // Convert cursor screen pos to NDC (-1..1), flipping Y
                let ndc = Vec2::new(
                    (centre.x / window_size.x) * 2.0 - 1.0,
                    -((centre.y / window_size.y) * 2.0 - 1.0),
                );

                // NDC → world space using transform.scale (not proj.scale)
                let cursor_world =
                    transform.translation.xy() + ndc * (window_size / 2.0) * current_scale;

                // Apply zoom
                const MAX_ZOOM_OUT: f32 = 49.0; // background is 66000x44000px, window 1400x900
                let new_scale = (current_scale * (1.0 - delta * 0.1)).clamp(5.0, MAX_ZOOM_OUT);
                transform.scale = Vec3::splat(new_scale);

                // Reposition camera so cursor_world stays under cursor
                let new_cam = cursor_world - ndc * (window_size / 2.0) * new_scale;
                transform.translation.x = new_cam.x;
                transform.translation.y = new_cam.y;
            }
            InputAction::PanCamera { delta } => {
                transform.translation.x -= delta.x * transform.scale.x;
                transform.translation.y += delta.y * transform.scale.y;
            }
            InputAction::SelectProvince { .. } => {
                // Not handled here
            }
        }
    }

    // Clamp camera so viewport never leaves the background bounds.
    // Half-viewport in world units = (window_size / 2) * scale
    let window_size = Vec2::new(1400.0, 900.0);
    let half_vp = window_size / 2.0 * transform.scale.x;

    const MAP_LEFT: f32 = -28000.0;
    const MAP_RIGHT: f32 = 38000.0;
    const MAP_BOTTOM: f32 = -13600.0;
    const MAP_TOP: f32 = 30400.0;

    let map_center_x = (MAP_LEFT + MAP_RIGHT) / 2.0;
    let map_center_y = (MAP_BOTTOM + MAP_TOP) / 2.0;

    let x_min = MAP_LEFT + half_vp.x;
    let x_max = MAP_RIGHT - half_vp.x;
    transform.translation.x = if x_min > x_max {
        map_center_x
    } else {
        transform.translation.x.clamp(x_min, x_max)
    };

    let y_min = MAP_BOTTOM + half_vp.y;
    let y_max = MAP_TOP - half_vp.y;
    transform.translation.y = if y_min > y_max {
        map_center_y
    } else {
        transform.translation.y.clamp(y_min, y_max)
    };
}
