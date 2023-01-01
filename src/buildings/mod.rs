use bevy::{
    app::{App, Plugin, Startup},
    ecs::{component::Component, entity::Entity, resource::Resource, system::Commands},
    platform::collections::HashMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildingKind {
    Farm,
    Mine,
    Sawmill,
    Barracks, // ...
}

pub struct BuildingDef {
    pub kind: BuildingKind,
    pub name: &'static str,
    pub gold_per_day: i32,
    // pub consumes: ResourceBundle, // grain, wood, stone, etc.
    // pub produces: ResourceBundle,
    pub max_level: u8,
    pub max_staff: u8,
}

#[derive(Resource, Default)]
pub struct BuildingRegistry(pub HashMap<BuildingKind, BuildingDef>);

#[derive(Component)]
pub struct Building {
    pub kind: BuildingKind,
    pub level: u8,
    pub location: BuildingLocation,
    pub manager: Option<bevy::ecs::entity::Entity>,
}

#[derive(Component)]
pub struct BuildingLocation {
    pub province_id: u32,
    pub town_id: Option<u32>,
}

// Province → buildings index
// This is used for quickly looking up which buildings are in a province when we want to calculate resource production, etc.
// Update when buildings are added/removed
#[derive(Resource, Default)]
pub struct ProvinceBuildingIndex {
    pub buildings: HashMap<u32, Vec<Entity>>,
}

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_buildings);
    }
}

fn register_buildings(mut commands: Commands) {
    let mut registry = HashMap::new();

    println!("Registering buildings...");
    registry.insert(
        BuildingKind::Farm,
        BuildingDef {
            kind: BuildingKind::Farm,
            name: "Farm",
            gold_per_day: 10,
            // consumes: ResourceBundle::default(),
            // produces: ResourceBundle {
            //     grain: 5,
            //     ..default()
            // },
            max_level: 3,
            max_staff: 5,
        },
    );

    registry.insert(
        BuildingKind::Mine,
        BuildingDef {
            kind: BuildingKind::Mine,
            name: "Mine",
            gold_per_day: 15,
            // consumes: ResourceBundle::default(),
            // produces: ResourceBundle {
            //     stone: 3,
            //     ..default()
            // },
            max_level: 3,
            max_staff: 5,
        },
    );

    registry.insert(
        BuildingKind::Sawmill,
        BuildingDef {
            kind: BuildingKind::Sawmill,
            name: "Sawmill",
            gold_per_day: 12,
            // consumes: ResourceBundle::default(),
            // produces: ResourceBundle {
            //     wood: 4,
            //     ..default()
            // },
            max_level: 3,
            max_staff: 5,
        },
    );

    registry.insert(
        BuildingKind::Barracks,
        BuildingDef {
            kind: BuildingKind::Barracks,
            name: "Barracks",
            gold_per_day: 20,
            // consumes: ResourceBundle {
            //     grain: 2,
            //     wood: 2,
            //     stone: 1,
            //     ..default()
            // },
            // produces: ResourceBundle {
            //     soldiers: 2,
            //     ..default()
            // },
            max_level: 3,
            max_staff: 10,
        },
    );

    println!("Registered buildings: {:?}", registry.keys());
    commands.insert_resource(BuildingRegistry(registry));

    let farm_id = commands
        .spawn(Building {
            kind: BuildingKind::Farm,
            level: 1,
            location: BuildingLocation { province_id: 18, town_id: None },
            manager: None,
        })
        .id();

    let mine_id = commands
        .spawn(Building {
            kind: BuildingKind::Mine,
            level: 2,
            location: BuildingLocation { province_id: 18, town_id: None },
            manager: None,
        })
        .id();

    let mut pbi = ProvinceBuildingIndex::default();
    pbi.buildings
        .entry(18)
        .or_insert_with(Vec::new)
        .extend(vec![farm_id, mine_id]);

    let sawmill_id = commands
        .spawn(Building {
            kind: BuildingKind::Sawmill,
            level: 1,
            location: BuildingLocation { province_id: 142, town_id: None },
            manager: None,
        })
        .id();

    let barracks_id = commands
        .spawn(Building {
            kind: BuildingKind::Barracks,
            level: 1,
            location: BuildingLocation { province_id: 142, town_id: None },
            manager: None,
        })
        .id();

    pbi.buildings
        .entry(142)
        .or_insert_with(Vec::new)
        .push(sawmill_id);
    pbi.buildings
        .entry(142)
        .or_insert_with(Vec::new)
        .push(barracks_id);

    commands.insert_resource(pbi);
    println!("Spawned seed buildings");
}
