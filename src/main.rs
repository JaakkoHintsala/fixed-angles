mod objects;
mod plugins;

use bevy::prelude::*;
use plugins::{setup::SetUpPlugin, user_input_movement::UserInputMovementPlugin};
use std::ops::Neg;

use components::*;
use objects::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetUpPlugin)
        .add_plugins(UserInputMovementPlugin)
        .run();
}
