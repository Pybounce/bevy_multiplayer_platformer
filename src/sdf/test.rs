
use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::{render_resource::{AsBindGroup, Buffer, BufferInitDescriptor, BufferUsages, ShaderRef}, renderer::{RenderDevice, RenderQueue}}, sprite::{Material2d, MaterialMesh2dBundle}};

use crate::stage::stage_builder::StageBuilderData;

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
    pub stroke_width: f32,
    #[storage(4, read_only, buffer)]
    buffer: Buffer,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test.wgsl".into()
    }
}

impl CustomMaterial {
    pub fn for_spike(buffer: &Buffer) -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Spike as i32,
            colour: LinearRgba::new(0.7, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_width: 0.05,
            buffer: buffer.clone()
        }
    }
    pub fn for_saw(buffer: &Buffer) -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Saw as i32,
            colour: LinearRgba::new(0.7, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_width: 0.05,
            buffer: buffer.clone()
        }
    }
    pub fn for_ground(buffer: &Buffer) -> Self {
        CustomMaterial {
            shape_id: SDFShapeID::Ground as i32,
            colour: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            stroke_colour: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
            stroke_width: 0.05,
            buffer: buffer.clone()
        }
    }
}


pub fn zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut mats: ResMut<Assets<CustomMaterial>>,
    mut stage_builder_data_opt: Option<ResMut<StageBuilderData>>,
    queue: Res<RenderQueue>
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
    if let None = stage_builder_data_opt { return; }

    let stage_builder_data = stage_builder_data_opt.unwrap();
    
    if let Some(m) = mats.get_mut(&stage_builder_data.spike_mat_handle) {
        let values: Vec<f32> = vec![1.0, 0.0, 0.0, 1.0];
        queue.write_buffer(&m.buffer, 0, bytemuck::cast_slice(&values));
    }
}
