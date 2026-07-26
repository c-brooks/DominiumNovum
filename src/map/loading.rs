// Runtime loading of the (large) geojson data files.
//
// Startup/Update systems that need ProvinceMap or TownMap must gate on
// `MapLoadState::Ready` (via `run_if(in_state(...))` or `OnEnter`), since
// those resources don't exist until the fetch + parse completes.

use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bevy::reflect::TypePath;

use crate::map::town::TownMap;
use crate::map::{ProvinceMap, parse_provinces_geojson, parse_towns_geojson};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum MapLoadState {
    #[default]
    Loading,
    Ready,
}

#[derive(Asset, TypePath)]
pub struct GeoJsonSource(pub String);

#[derive(Default, TypePath)]
pub struct GeoJsonAssetLoader;

impl AssetLoader for GeoJsonAssetLoader {
    type Asset = GeoJsonSource;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(GeoJsonSource(String::from_utf8_lossy(&bytes).into_owned()))
    }

    fn extensions(&self) -> &[&str] {
        &["geojson"]
    }
}

#[derive(Resource)]
pub struct MapAssetHandles {
    pub provinces: Handle<GeoJsonSource>,
    pub towns: Handle<GeoJsonSource>,
}

pub fn begin_loading_map_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MapAssetHandles {
        provinces: asset_server.load("political.geojson"),
        towns: asset_server.load("towns.geojson"),
    });
}

pub fn check_map_data_loaded(
    handles: Res<MapAssetHandles>,
    sources: Res<Assets<GeoJsonSource>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<MapLoadState>>,
) {
    let Some(provinces_src) = sources.get(&handles.provinces) else {
        return;
    };
    let Some(towns_src) = sources.get(&handles.towns) else {
        return;
    };

    let provinces = parse_provinces_geojson(&provinces_src.0);
    println!("Loaded {} provinces", provinces.len());
    commands.insert_resource(ProvinceMap { provinces });

    let towns = parse_towns_geojson(&towns_src.0);
    println!("Loaded {} towns", towns.len());
    commands.insert_resource(TownMap { towns });

    // Drop the strong handles so the (large) decoded source strings can be
    // freed now that we've parsed them into structured data.
    commands.remove_resource::<MapAssetHandles>();

    next_state.set(MapLoadState::Ready);
}
