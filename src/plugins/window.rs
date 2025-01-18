use bevy::{prelude::*, window::WindowResized};
use bundles::*;
use components::*;
use states::*;

use crate::*;

pub struct WindowHandlerPlugin;

impl Plugin for WindowHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_resize_system);
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&mut Sprite, &AreaState)>,
) {
    for (mut sprite, area) in &mut query {
        for e in resize_reader.read() {
            let height = e.height;
            let width = e.width;
            let scaled = area.get_width_and_height_scaled(width, height);
            dbg!(height);
            dbg!(width);
            dbg!(scaled);
            dbg!(sprite.custom_size);
            sprite.custom_size = Some(scaled);
            dbg!(sprite.custom_size);
        }
    }
}
