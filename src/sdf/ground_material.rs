use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::{render_resource::{AsBindGroup, Buffer, BufferInitDescriptor, BufferUsages, ShaderRef}, renderer::{RenderDevice, RenderQueue}}, sprite::{Material2d, MaterialMesh2dBundle}};

use crate::stage::stage_builder::StageBuilderData;

use super::enums::SDFShapeID;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GroundMaterial {
    #[uniform(0)]
    pub size: f32,
    #[uniform(1)]
    pub colour: LinearRgba,
    #[uniform(2)]
    pub stroke_colour: LinearRgba,
    #[uniform(3)]
    pub stroke_width: f32,
    #[storage(4, read_only, buffer)]
    pub buffer: Buffer,
}

impl Material2d for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ground.wgsl".into()
    }
}
