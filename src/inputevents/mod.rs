use bevy::prelude::*;
pub mod systems;

pub struct InputPlugin;

#[derive(Message, Clone)]
pub struct InputEvent {
    pub action: InputAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputAction {
    MoveCamera { direction: Vec2 },
    PanCamera { delta: Vec2 },
    ZoomCamera { delta: f32, centre: Vec2 },
    SelectProvince { id: u32 },
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InputEvent>()
            .add_systems(Update, (systems::handle_input, systems::handle_mouse_drag));
    }
}
