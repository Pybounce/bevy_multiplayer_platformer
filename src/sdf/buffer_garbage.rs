
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

    
    //let bind_group_layout = render_device.create_bind_group_layout(
    //    Some("data_bind_group_layout"),
    //    &[BindGroupLayoutEntry {
    //        binding: 0,
    //        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
    //        ty: BindingType::Buffer {
    //            ty: BufferBindingType::Storage { read_only: true },
    //            has_dynamic_offset: false,
    //            min_binding_size: None
    //        },
    //        count: None
    //    }],
    //);

//    //let bind_group = render_device.create_bind_group(
    //    Some("data_bind_group"), 
    //    &bind_group_layout, 
    //    &[BindGroupEntry {
    //        binding: 0,
    //        resource: BindingResource::Buffer(
    //            BufferBinding {
    //                buffer: &buffer,
    //                offset: 0,
    //                size: None
    //            }
    //        ),
    //    }]
    //);
    //commands.insert_resource::<BufferBullshit>(BufferBullshit { test_buffer: buffer, test_bind_group: bind_group });
}


