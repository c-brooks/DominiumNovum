use crate::map::ProvinceMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component, Debug, Clone)]
pub struct CharacterIdentity {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub colour: Color,
}

#[derive(Component, Debug, Clone)]
pub struct Location {
    pub province_id: u32,
}

#[derive(Resource, Default)]
pub struct HoveredPlayer(pub Option<Entity>);

pub fn player_hover_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    markers: Query<(Entity, &Transform), With<PlayerMarker>>,
    mut hovered: ResMut<HoveredPlayer>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        hovered.0 = None;
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor) else {
        return;
    };

    // Use camera scale to keep hit radius consistent on screen
    let cam_scale = cam_transform.compute_transform().scale.x;
    let hit_radius = 500.0; // matches your circle radius

    hovered.0 = markers
        .iter()
        .find(|(_, t)| t.translation.xy().distance(world_pos) < hit_radius)
        .map(|(e, _)| e);
}

pub fn spawn_player(mut commands: Commands) {
    let starting_province = 18;
    let name = "Bob".to_string();
    let colour = Color::srgb(0.9, 0.8, 0.1); // gold

    commands.spawn((
        CharacterIdentity {
            id: 0,
            name,
            age: 35,
            colour,
        },
        Location {
            province_id: starting_province,
        },
        PlayerCharacter,
    ));
}

pub fn update_player_marker(
    player_location: Query<&Location, (With<PlayerCharacter>, Changed<Location>)>,
    mut markers: Query<&mut Transform, With<PlayerMarker>>,
    province_map: Res<ProvinceMap>,
) {
    let Ok(location) = player_location.single() else {
        return;
    };
    let Some(pos) = province_map.province_centroid_screen_pos(location.province_id) else {
        return;
    };
    let Ok(mut transform) = markers.single_mut() else {
        return;
    };
    transform.translation = Vec3::new(pos.x, pos.y, 10.0);
}

pub fn spawn_player_marker(
    mut commands: Commands,
    province_map: Res<ProvinceMap>,
    player_location: Query<&Location, With<PlayerCharacter>>,
    player_identity: Query<&CharacterIdentity, With<PlayerCharacter>>,
) {
    let Ok(location) = player_location.single() else {
        return;
    };
    let Ok(identity) = player_identity.single() else {
        return;
    };

    let province_centre = match province_map.province_centroid_screen_pos(location.province_id) {
        Some(pos) => pos,
        None => {
            println!(
                "Could not find screen position for province {}",
                location.province_id
            );
            return;
        }
    };

    let circle = shapes::Circle {
        radius: 500.0,
        center: Vec2::ZERO,
    };

    println!(
        "Spawning player marker at province {} (screen pos: {:?})",
        location.province_id, province_centre
    );

    let colour = identity.colour;
    commands.spawn((
        ShapeBuilder::with(&circle)
            .fill(Fill::color(colour))
            .stroke((Color::srgb(0.3, 0.2, 0.0), 2.0))
            .build(),
        Transform::from_xyz(province_centre.x, province_centre.y, 10.0),
        PlayerMarker,
    ));
}
