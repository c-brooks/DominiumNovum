use std::char;

use bevy::{
    app::{App, Plugin, Startup},
    ecs::{component::Component, entity::Entity, resource::Resource, system::Commands},
    platform::collections::HashMap,
};

use crate::player::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterPortrait {
    Baker,
    Commander,
    Doctor,
    Guildmaster,
    Lumberjack,
}

#[derive(Component)]
pub struct Character {
    pub name: String,
    pub age: u32,
    pub portrait: CharacterPortrait,
}

#[derive(Resource, Default)]
pub struct CharacterRegistry {
    pub entities: HashMap<u32, Entity>,
    next_id: u32,
}

impl CharacterRegistry {
    pub fn next_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id
    }

    pub fn get(&self, id: u32) -> Option<Entity> {
        self.entities.get(&id).copied()
    }
}

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterRegistry>()
            .add_systems(Startup, spawn_starting_characters);
    }
}

fn spawn_starting_characters(mut commands: Commands) {
    let mut registry = CharacterRegistry::default();
    println!("Spawning starting characters...");

    let char1 = (
        Character {
            name: "Alice".into(),
            age: 30,
            portrait: CharacterPortrait::Baker,
        },
        Location { province_id: 18 },
    );
    let char2 = (
        Character {
            name: "Bob".into(),
            age: 25,
            portrait: CharacterPortrait::Commander,
        },
        Location { province_id: 19 },
    );
    let char3 = (
        Character {
            name: "Charlie".into(),
            age: 40,
            portrait: CharacterPortrait::Lumberjack,
        },
        Location { province_id: 20 },
    );
    let char4 = (
        Character {
            name: "David".into(),
            age: 35,
            portrait: CharacterPortrait::Guildmaster,
        },
        Location { province_id: 21 },
    );
    let char5 = (
        Character {
            name: "Eve".into(),
            age: 28,
            portrait: CharacterPortrait::Doctor,
        },
        Location { province_id: 22 },
    );

    let entity1 = commands.spawn(char1).id();
    let entity2 = commands.spawn(char2).id();
    let entity3 = commands.spawn(char3).id();
    let entity4 = commands.spawn(char4).id();
    let entity5 = commands.spawn(char5).id();

    let id1 = registry.next_id();
    let id2 = registry.next_id();
    let id3 = registry.next_id();
    let id4 = registry.next_id();
    let id5 = registry.next_id();

    registry.entities.insert(id1, entity1);
    registry.entities.insert(id2, entity2);
    registry.entities.insert(id3, entity3);
    registry.entities.insert(id4, entity4);
    registry.entities.insert(id5, entity5);

    commands.insert_resource(registry);
}
