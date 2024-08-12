use bevy::{math::Rect, prelude::{default, Commands, Component, Entity, Query, Res, With}, sprite::Sprite, time::{Time, Timer, TimerMode}, transform::commands};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sleeping};

use crate::{common::animated_sprite::SpriteAnimator, ground::Ground, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE_HALF}}};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct IntervalBlock {
    timer: Timer,
    currently_active: bool
}

pub struct IntervalBlockFactory;

impl IntervalBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, interval_block_asset: &stage_asset::IntervalBlock) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, interval_block_asset.grid_pos, atlas_rects[0], 0.0, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            IntervalBlock {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                currently_active: false
            },
            SpriteAnimator::new_non_repeating(100, atlas_rects),
            Ground,
        ));
    }
}

pub fn tick_interval_blocks(
    mut query: Query<(Entity, &mut IntervalBlock, &mut SpriteAnimator)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, mut interval_block, mut sprite_anim) in &mut query {
        interval_block.timer.tick(time.delta());
        if interval_block.timer.just_finished() {
            interval_block.currently_active = !interval_block.currently_active;
            if interval_block.currently_active {
                sprite_anim.play_reverse();
                commands.entity(e).try_insert(Collider::cuboid(TILE_SIZE_HALF, TILE_SIZE_HALF));
                commands.entity(e).try_insert(InstantKiller);
            }
            if !interval_block.currently_active {
                sprite_anim.play();
                commands.entity(e).remove::<Collider>();
            }
        }
    }
}

pub fn stop_interval_block_crush(query: Query<Entity, (With<InstantKiller>, With<IntervalBlock>)>, mut commands: Commands) {
    for e in &query {
        commands.entity(e).remove::<InstantKiller>();
    }
}