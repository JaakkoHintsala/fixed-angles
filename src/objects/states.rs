use bevy::{
    pbr::ViewLightProbesUniformOffset,
    prelude::*,
    render::camera::{ScalingMode, Viewport},
};
use strum_macros::{Display, EnumDiscriminants, EnumIter};


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Display, EnumIter, Component)]
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
    pub fn get_back_ground_picture(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match &self {
            AreaState::StartingArea => asset_server.load("medieval-castle-yard-26793497.webp"),
            AreaState::Castle => asset_server.load("frank-radecki-highresscreenshot00000.jpg"),
            AreaState::Forest => {
                asset_server.load("Culdees-Castle-Perthshire-for-sale-1-thumb.jpg")
            }
        }
    }

    pub fn get_2d_camera(&self) -> (OrthographicProjection, Transform, Camera2d) {
        // Viewport {
        //     physical_position: todo!(),
        //     physical_size: todo!(),
        //     ..Viewport::default()
        // };
        (
            OrthographicProjection {
                // We can set the scaling mode to FixedVertical to keep the viewport height constant as its aspect ratio changes.
                // The viewport height is the height of the camera's view in world units when the scale is 1.
                scaling_mode: ScalingMode::WindowSize,
                // This is the default value for scale for orthographic projections.
                // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
                scale: 1.,

                ..OrthographicProjection::default_2d()
            },
            self.get_camera_transform(),
            Camera2d::default(),
        )
    }

    pub fn get_width_and_height(&self) -> Vec2 {
        match self {
            AreaState::StartingArea => Vec2 { x: 599.0, y: 900.0 },
            AreaState::Castle => Vec2 {
                x: 1870.0,
                y: 954.0,
            },
            AreaState::Forest => Vec2 { x: 450.0, y: 300.0 },
        }
    }

    pub fn get_width_and_height_scaled(&self, max_width: f32, max_height: f32) -> Vec2 {
        let ratio_of_max = max_width / max_height;
        let unscaled = self.get_width_and_height();
        let ratio_of_unscaled = unscaled.x / unscaled.y;

        if ratio_of_max > ratio_of_unscaled {
            Vec2 {
                x: unscaled.x * (max_height / unscaled.y),
                y: max_height,
            }
        } else {
            Vec2 {
                x: max_width,
                y: unscaled.y * (max_width / unscaled.x),
            }
        }


    }

    pub fn get_center_coords(&self) -> Vec2 {
        match self {
            AreaState::StartingArea => Vec2 { x: 0.0, y: 0.0 },
            AreaState::Castle => Vec2 { x: 10000.0, y: 0.0 },
            AreaState::Forest => Vec2 { x: 20000.0, y: 0.0 },
        }
    }

    pub fn get_camera_transform(&self) -> Transform {
        Transform::from_xyz(self.get_center_coords().x, self.get_center_coords().y, 1.0)
    }
}
