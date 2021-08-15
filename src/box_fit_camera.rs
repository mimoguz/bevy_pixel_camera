use bevy::prelude::{Bundle, GlobalTransform, Mat4, Reflect, ReflectComponent, Transform};
use bevy::render::camera::{Camera, CameraProjection, DepthCalculation, VisibleEntities};

/// Provides the components for the camera entity.
#[derive(Bundle)]
pub struct BoxFitCameraBundle {
    pub camera: Camera,
    pub projection: BoxFitProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl BoxFitCameraBundle {
    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified resolution inside the window.
    pub fn from_resolution(width: i32, height: i32) -> Self {
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            projection: BoxFitProjection {
                virtual_width: width,
                virtual_height: height,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }
}

/// Component for an orthographic projection.
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct BoxFitProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub zoom: f32,

    /// `zoom` will be automatically updated to always fit
    /// `virtual_width` in the window as best as possible.
    pub virtual_width: i32,

    /// `zoom` will be automatically updated to always fit
    /// `virtual_height` in the window as best as possible.
    pub virtual_height: i32,

    // If true, (0, 0) is the pixel closest to the center of the window,
    // otherwise it's at bottom left.
    pub centered: bool,
}

impl CameraProjection for BoxFitProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let zoom_x = width / self.virtual_width as f32;
        let zoom_y = height / self.virtual_height as f32;
        self.zoom = zoom_x.min(zoom_y);

        let actual_width = width / (self.zoom as f32);
        let actual_height = height / (self.zoom as f32);

        if self.centered {
            self.left = -((actual_width as i32) / 2) as f32;
            self.right = self.left + actual_width;
            self.bottom = -((actual_height as i32) / 2) as f32;
            self.top = self.bottom + actual_height;
        } else {
            self.left = 0.0;
            self.right = actual_width;
            self.bottom = 0.0;
            self.top = actual_height;
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for BoxFitProjection {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            virtual_width: 640,
            virtual_height: 480,
            zoom: 1.0,
            centered: true,
        }
    }
}