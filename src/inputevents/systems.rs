use super::{InputAction, InputEvent};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn handle_mouse_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut events: MessageWriter<InputEvent>,
    mut dragging: Local<bool>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        *dragging = true;
    } else if mouse.just_released(MouseButton::Left) {
        *dragging = false;
    }

    if *dragging && motion.delta != Vec2::ZERO {
        events.write(InputEvent {
            action: InputAction::PanCamera {
                delta: motion.delta,
            },
        });
    }
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut events: MessageWriter<InputEvent>,
    time: Res<Time>,
) {
    let Ok(window) = window_q.single() else {
        return;
    };

    // Camera movement
    let mut direction = Vec2::ZERO;
    let speed = 300.0 * time.delta_secs();

    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        events.write(InputEvent {
            action: InputAction::MoveCamera {
                direction: direction * speed,
            },
        });
    }

    // Camera zoom
    if scroll.is_changed() && scroll.delta != Vec2::ZERO {
        let a = InputAction::ZoomCamera {
            delta: scroll.delta.y,
            centre: window.cursor_position().unwrap_or_default(),
        };
        events.write(InputEvent { action: a });
    }
}
