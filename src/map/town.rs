use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TownProperties {
    pub town_id: u32,
    pub name: String,
    pub province_id: u32,
}

// Processed province ready for game use
#[derive(Debug, Clone)]
pub struct TownDef {
    pub id: u32,
    pub name: String,
    pub centroid: (f64, f64),
    pub province_id: u32,
}

#[derive(Resource)]
pub struct TownMap {
    pub towns: Vec<TownDef>,
}
