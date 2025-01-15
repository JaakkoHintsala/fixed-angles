use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AreaState {
    #[default]
    StartingArea,
    Castle,
    Forest,
}

pub struct WidthAndHeight {
    width: f32,
    height: f32,
}

impl AreaState {
    fn get_back_ground_picture(&self, asset_server: Res<AssetServer>) -> Handle<Image> {
        match &self {
            AreaState::StartingArea => asset_server.load("background.png"),
            AreaState::Castle => asset_server.load("background.png"),
            AreaState::Forest => asset_server.load("background.png"),
        }
    }

    fn get_2d_camera(&self) -> (OrthographicProjection, Transform, Camera2d) {
        (
            OrthographicProjection {
                // We can set the scaling mode to FixedVertical to keep the viewport height constant as its aspect ratio changes.
                // The viewport height is the height of the camera's view in world units when the scale is 1.
                scaling_mode: self.get_scaling_mode(),
                // This is the default value for scale for orthographic projections.
                // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
                scale: 1.,
                ..OrthographicProjection::default_2d()
            },
            self.get_camera_transform(),
            Camera2d::default(),
        )
    }

    fn get_scaling_mode(&self) -> ScalingMode {
        match self {
            AreaState::StartingArea => ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            AreaState::Castle => ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            AreaState::Forest => ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
        }
    }

    fn get_camera_transform(&self) -> Transform {
        match self {
            AreaState::StartingArea => Transform::from_xyz(0.0, 2.0, 1.0),
            AreaState::Castle => Transform::from_xyz(0.0, 50.0, 1.0),
            AreaState::Forest => Transform::from_xyz(0.0, -50.0, 1.0),
        }
    }
}
