
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{local_player::LocalPlayer, stage::stage_builder::stage_creator::{get_object_tilemap_rect_from_index, ObjectAtlasIndices}};


pub fn update_player_animation(
    mut query: Query<(&mut Sprite, &Velocity), With<LocalPlayer>>
) {
    for (mut sprite, vel) in &mut query {
        if vel.linvel.x.abs() < 5.0 {
            sprite.rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::PlayerIdle0).into();
        }
        else if vel.linvel.x.abs() < 70.0 {
            sprite.rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::PlayerRight0).into();
        }
        else {
            sprite.rect = get_object_tilemap_rect_from_index(ObjectAtlasIndices::PlayerRight1).into();
        }
    }
}