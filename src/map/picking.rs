use crate::map::province::{HoveredProvince, ProvinceMap, SelectedProvince};
use crate::map::screen_to_geo;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub fn province_picking_system(
    mut egui: EguiContexts,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut hovered: ResMut<HoveredProvince>,
    mut selected: ResMut<SelectedProvince>,
    mouse: Res<ButtonInput<MouseButton>>,
    registry: Res<ProvinceMap>,
) {
    // Don't pick provinces when egui is handling the pointer
    if egui
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false)
    {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // Convert world position to geographic coordinates
    let (lon, lat) = screen_to_geo(world_pos);

    // Hit test against province polygons
    let province_id = registry.province_at_point(lon, lat);

    // Only write if changed, so Bevy's change detection can skip update_province_colours
    if hovered.0 != province_id {
        hovered.0 = province_id;
    }

    if mouse.just_pressed(MouseButton::Left) {
        selected.0 = hovered.0;
    }
}
