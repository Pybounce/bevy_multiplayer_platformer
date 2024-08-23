use bevy::{math::VectorSpace, prelude::*, render::{render_resource::{BufferInitDescriptor, BufferUsages}, renderer::{RenderDevice, RenderQueue}}};

use crate::{sdf::{ground_material::GroundMaterial, test::CustomMaterial}, stage::stage_objects::StageObject};

use super::{events::{StageBuildCompleteEvent, StageBuildFailedEvent}, stage_asset::Stage, stage_creator::{StageCreator, TILE_SIZE}, CurrentStageData, StageBuilderData};


pub fn unload_old_stage(
    stage_piece_query: Query<(Entity, &StageObject)>,
    mut commands: Commands,
    stage_builder_data: Res<StageBuilderData>,
) {
    for (e, sp) in &stage_piece_query {
        if sp.stage_id != stage_builder_data.stage_id {
            commands.entity(e).despawn();
            commands.remove_resource::<CurrentStageData>();
        }
    }
}

pub fn try_build_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage_builder_data: Res<StageBuilderData>,
    stage_assets: Res<Assets<Stage>>,
    mut complete_event_writer: EventWriter<StageBuildCompleteEvent>,
    mut failed_event_writer: EventWriter<StageBuildFailedEvent>,
    mut sdf_mats: ResMut<Assets<CustomMaterial>>,
    mut ground_mats: ResMut<Assets<GroundMaterial>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>

) {
    match asset_server.load_state(&stage_builder_data.stage_handle) {
        bevy::asset::LoadState::NotLoaded => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed(_) => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
    }

    let stage_asset = stage_assets.get(&stage_builder_data.stage_handle);
    let tilemap_handle: Handle<Image> = asset_server.load("tilemap.png");
    let object_tilemap_handle: Handle<Image> = asset_server.load("object_tilemap.png");




    match stage_asset {
        Some(stage) => {

            
            let ground_buffer_values: Vec<Vec2> = vec![Vec2::ZERO; stage.ground_tiles.len()];

            let ground_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor 
            { 
                    label: Some("Ground Buffer Data"), 
                    contents: bytemuck::cast_slice(&ground_buffer_values), 
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            });
        
            let ground_mat_handle = ground_mats.add(GroundMaterial {
                size: stage.grid_width.max(stage.grid_height) as f32,
                colour: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
                stroke_colour: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                stroke_width: 0.05 / stage.grid_width.max(stage.grid_height) as f32,
                buffer: ground_buffer.clone(),
            });
        
            let spike_mat_handle = sdf_mats.add(CustomMaterial::for_spike());
            let saw_mat_handle = sdf_mats.add(CustomMaterial::for_saw());


            let stage_creator = StageCreator::new(&stage, &tilemap_handle, &object_tilemap_handle, &stage_builder_data, &ground_mat_handle, &spike_mat_handle, &saw_mat_handle, &ground_buffer);
            if stage_creator.build(&mut commands, &render_queue, &ground_mats) {
                commands.insert_resource(CurrentStageData {
                    stage_id: stage.id,
                    spawn_translation: (stage.spawn_grid_pos * TILE_SIZE).extend(0.0),
                    bounds: Rect::new(-TILE_SIZE, -TILE_SIZE, stage.grid_width as f32 * TILE_SIZE, stage.grid_height as f32 * TILE_SIZE),
                });
                complete_event_writer.send(StageBuildCompleteEvent { stage_id: stage_builder_data.stage_id });
            }
            else {
                failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            }
        },
        None => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
    }
}
