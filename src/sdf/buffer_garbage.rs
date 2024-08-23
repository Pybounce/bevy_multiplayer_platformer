
use bevy::render::renderer::{RenderContext, RenderQueue};
use bevy::{prelude::*, render::renderer::RenderDevice};
use bevy::render::render_resource::{BindGroup, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, RenderPipeline, ShaderStages};


#[derive(Resource)]
pub struct BufferBullshit {
    pub test_buffer: Buffer,
}

pub fn setup_buffer_bs(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let values: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor 
    { 
            label: Some("Buffer Data"), 
            contents: bytemuck::cast_slice(&values), 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
    });

    commands.insert_resource(BufferBullshit {
        test_buffer: buffer
    });

}


