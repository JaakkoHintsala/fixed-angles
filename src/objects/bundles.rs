use bevy::prelude::*;

use crate::*;

#[derive(Bundle)]
pub struct UserInputMovable {
    pub movable_by_user_input: MovableByUserInput,
    pub direction: Direction,
}