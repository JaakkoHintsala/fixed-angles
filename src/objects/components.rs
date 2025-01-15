use bevy::prelude::*;


#[derive(Component)]
pub enum MovableByUserInput {
    Active((f32, f32)),
    UnActive,
}
#[derive(Component)]
pub struct MyCameraMarker;


#[derive(Component, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Component)]
pub struct Player;