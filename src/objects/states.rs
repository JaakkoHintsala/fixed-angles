use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AreaState {
    #[default]
    StartingArea,
    Castle,
    Forest,
}