use bevy::prelude::*;
use bundles::*;
use components::*;
use states::*;

use crate::*;

pub struct SetUpPlugin;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        // Transform::from_xyz(0., 0., 0.),
        Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            image: asset_server.load("man-running-silhouette-vector-9.png"),
            ..default()
        },
        UserInputMovable {
            movable_by_user_input: MovableByUserInput::Active((50.0, 50.0)),
            direction: Direction::Up,
        },
    ));

    commands.spawn((
        MyCameraMarker,
        Camera2d,
        Transform::from_xyz(10.0, 12.0, 1.0),
    ));
}



impl Plugin for SetUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AreaState>();
        app.add_systems(Startup, setup);
    }
}

