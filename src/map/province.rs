use bevy::prelude::*;
use geo::{BoundingRect, Contains, Point, Polygon, Rect};
use serde::Deserialize;

// Raw data as it comes from GeoJSON
#[derive(Debug, Deserialize)]
pub struct ProvinceProperties {
    pub province_id: u32,
    pub name: String,
    pub neighbors: Option<String>, // "1,2,3" — we parse this
}

// Processed province ready for game use
#[derive(Debug, Clone)]
pub struct ProvinceDef {
    pub id: u32,
    pub name: String,
    pub neighbors: Vec<u32>,
    pub polygon: Polygon<f64>,
    pub bbox: Rect<f64>,
    pub centroid: (f64, f64),
    pub base_colour: Color,
}

// Bevy resource holding all provinces
#[derive(Resource)]
pub struct ProvinceMap {
    pub provinces: Vec<ProvinceDef>,
}

impl ProvinceMap {
    pub fn province_at_point(&self, lon: f64, lat: f64) -> Option<u32> {
        let point = Point::new(lon, lat);
        self.provinces
            .iter()
            .filter(|p| p.bbox.contains(&point))
            .find(|p| p.polygon.contains(&point))
            .map(|p| p.id)
    }

    pub fn get(&self, id: u32) -> Option<&ProvinceDef> {
        self.provinces.iter().find(|p| p.id == id)
    }
}

// Bevy component — attached to each province shape entity
#[derive(Component)]
pub struct ProvinceShape {
    pub id: u32,
    pub base_colour: Color,
}

// Bevy resource tracking selection state
#[derive(Resource, Default)]
pub struct SelectedProvince(pub Option<u32>);

#[derive(Resource, Default)]
pub struct HoveredProvince(pub Option<u32>);
