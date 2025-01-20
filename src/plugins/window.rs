use bevy::{prelude::*, window::WindowResized};
use bundles::*;
use components::*;
use states::*;

use crate::*;

pub struct WindowHandlerPlugin;

impl Plugin for WindowHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_resize_system);
        app.add_systems(StateTransition, (resize_on_state_change,));
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut projection: Single<&mut OrthographicProjection, With<MyCameraMarker>>,
    area: Res<State<AreaState>>,
) {
    for e in resize_reader.read() {
        projection.scale = area.calculate_scale_by_window(e.width, e.height);
    }
}

fn resize_on_state_change(
    mut projection: Single<&mut OrthographicProjection, With<MyCameraMarker>>,
    window: Single<&Window>,
    area: Res<State<AreaState>>,
) {
    projection.scale = area.calculate_scale_by_window(window.width(), window.height());
}
