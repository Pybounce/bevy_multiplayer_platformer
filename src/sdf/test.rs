
use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, MaterialMesh2dBundle}};

use super::enums::SDFShapeID;

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub shape_id: i32,
    #[uniform(1)]
    pub colour: LinearRgba,
    #[uniform(2)]
    pub stroke_colour: LinearRgba,
    #[uniform(3)]
    pub stroke_width: f32
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test.wgsl".into()
    }
}

impl CustomMaterial {
    pub fn for_spike() -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Spike as i32,
            colour: LinearRgba::new(0.7, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_width: 0.05
        }
    }
    pub fn for_saw() -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Saw as i32,
            colour: LinearRgba::new(0.7, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_width: 0.05
        }
    }
    pub fn for_ground() -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Ground as i32,
            colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
            stroke_width: 0.05
        }
    }
}


pub fn zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for mut projection in &mut camera_query {

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            projection.scale -= dy / 500.0;
        }
    }
}
