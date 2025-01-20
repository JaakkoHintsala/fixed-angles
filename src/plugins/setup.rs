use bevy::prelude::*;
use bundles::*;
use components::*;
use states::*;
use strum::IntoEnumIterator;

use crate::*;

pub struct SetUpPlugin;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
    default_area: Res<State<AreaState>>,
) {
    let start_location: Vec2 = AreaState::default().get_center_coords();
    
    let window = windows.single();

    commands.spawn((
        Player,
        Transform::from_xyz(start_location.x, start_location.y, 1.),
        Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            image: asset_server.load("man-running-silhouette-vector-9.png"),
            anchor: bevy::sprite::Anchor::Center,
            ..default()
        },
        UserInputMovable {
            movable_by_user_input: MovableByUserInput::Active((50.0, 50.0)),
            direction: Direction::Up,
        },
    ));

    for area in AreaState::iter() {
        commands.spawn((
            area,
            Transform::from_xyz(area.get_center_coords().x, area.get_center_coords().y, 0.0),
            Sprite {
                custom_size: Some(area.get_width_and_height()),
                image: area.get_back_ground_picture(&asset_server),
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
    }

    commands.spawn((
        MyCameraMarker,
        default_area.get_2d_camera(),
        // Transform::from_xyz(start_location.x, start_location.y, 1.),
    ));
}

impl Plugin for SetUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AreaState>();
        app.add_systems(Startup, setup);
    }
}
