
use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, MaterialMesh2dBundle}};

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test.wgsl".into()
    }
}


pub fn add_custom_shader_sprite(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(512.0, 512.0)).into(),
        transform: Transform::default(),
        material: materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
        }),
        ..default()
    });
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
            projection.scale -= dy / 300.0;
        }
    }
}
