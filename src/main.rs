//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
enum MovableByUserInput {
    Active((f32, f32)),
    UnActive,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("man-running-silhouette-vector-9.png")),
        Transform::from_xyz(100., 0., 0.),
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn move_with_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<(&mut Transform, &MovableByUserInput)>,
    time: Res<Time>,
) {
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    for (mut transform, movable_by_user_input) in &mut transforms {
        if let MovableByUserInput::Active((speed_x, speed_y)) = movable_by_user_input {
            let new_x = transform.translation.x + direction_x * speed_x * time.delta_secs();
            let new_y = transform.translation.y + direction_y * speed_y * time.delta_secs();

            transform.translation.x = new_x;
            transform.translation.y = new_y;
        };
    }

}

pub struct PlayerSetUpPlugin;

impl Plugin for PlayerSetUpPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
