use bevy::prelude::*;

use crate::*;
use components::*;

pub struct UserInputMovementPlugin;

impl Plugin for UserInputMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (move_with_user_input, flip_sprites ).chain());
    }
}

fn move_with_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<(&mut Transform, &mut Direction, &MovableByUserInput)>,
    time: Res<Time>,
) {
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    for (mut transform, mut direction, movable_by_user_input) in &mut transforms {
        if let MovableByUserInput::Active((speed_x, speed_y)) = movable_by_user_input {
            let new_x = transform.translation.x + direction_x * speed_x * time.delta_secs();
            let new_y = transform.translation.y + direction_y * speed_y * time.delta_secs();

            *direction = match (direction_x, direction_y) {
                (x, y) if x == 0.0 && y > 0.0 => Direction::Up,
                (x, y) if x > 0.0 && y > 0.0 => Direction::UpRight,
                (x, y) if x > 0.0 && y == 0.0 => Direction::Right,
                (x, y) if x > 0.0 && y < 0.0 => Direction::DownRight,
                (x, y) if x == 0.0 && y < 0.0 => Direction::Down,
                (x, y) if x < 0.0 && y < 0.0 => Direction::DownLeft,
                (x, y) if x < 0.0 && y == 0.0 => Direction::Left,
                (x, y) if x < 0.0 && y > 0.0 => Direction::UpLeft,
                (_, _) => *direction,
            };

            transform.translation.x = new_x;
            transform.translation.y = new_y;
        };
    }
}

fn flip_sprites(mut query: Query<(&mut Transform, &Direction)>) {
    for (mut transform, direction) in &mut query {
        match direction {
            Direction::Up => {}
            Direction::UpRight => {
                transform.scale.x = transform.scale.x.abs();
            }
            Direction::Right => {
                transform.scale.x = transform.scale.x.abs();
            }
            Direction::DownRight => {
                transform.scale.x = transform.scale.x.abs();
            }
            Direction::Down => {}
            Direction::DownLeft => {
                transform.scale.x = transform.scale.x.abs().neg();
            }
            Direction::Left => {
                transform.scale.x = transform.scale.x.abs().neg();
            }
            Direction::UpLeft => {
                transform.scale.x = transform.scale.x.abs().neg();
            }
        }
    }
}