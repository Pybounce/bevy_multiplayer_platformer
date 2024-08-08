use bevy::prelude::*;
use bevy_rapier2d::prelude::CollidingEntities;


#[derive(Component)]
pub struct SpriteAnimator {
    ///Milliseconds between each frame
    atlas_rects: Vec<Rect>,
    current_atlas_index: usize,
    timer: Timer,
    repeat_forever: bool,
    currently_animate: bool
}

#[derive(Component)]
pub struct AnimateOnTouch
{
    pub animator_entity: Entity
}


impl SpriteAnimator {
    pub fn new(frame_delay: u128, atlas_rects: Vec<Rect>) -> SpriteAnimator {
        SpriteAnimator {
            atlas_rects: atlas_rects.clone(),
            current_atlas_index: 0,
            timer: Timer::from_seconds(frame_delay as f32 / 1000.0, TimerMode::Repeating),
            repeat_forever: true,
            currently_animate: true,
        }
    }
    pub fn new_non_repeating(frame_delay: u128, atlas_rects: Vec<Rect>) -> SpriteAnimator {
        SpriteAnimator {
            atlas_rects: atlas_rects.clone(),
            current_atlas_index: 0,
            timer: Timer::from_seconds(frame_delay as f32 / 1000.0, TimerMode::Repeating),
            repeat_forever: false,
            currently_animate: false,
        }
    }
    pub fn increment_atlas_index(&mut self) {
        self.current_atlas_index += 1;
        if self.current_atlas_index >= self.atlas_rects.len() {
            self.currently_animate = self.repeat_forever;
            self.current_atlas_index = 0;
        }
    }
    pub fn get_current_atlas_rect(&self) -> Rect {
        self.atlas_rects[self.current_atlas_index]
    }
}

pub fn animate_sprites(
    mut query: Query<(&mut SpriteAnimator, &mut Sprite)>,
    time: Res<Time>
) {
    for (mut anim_sprite, mut sprite) in &mut query {
        if anim_sprite.currently_animate == false { continue; }
        anim_sprite.timer.tick(time.delta());
        if anim_sprite.timer.just_finished() {
            anim_sprite.increment_atlas_index();
            sprite.rect = Some(anim_sprite.get_current_atlas_rect());
        }
    }
}


pub fn check_animate_on_touch(
    toucher_query: Query<&CollidingEntities>,
    mut sprite_animators: Query<&mut SpriteAnimator>,
    mut animate_on_touch_query: Query<&AnimateOnTouch>,
) {
    for colliding_entities in &toucher_query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(aot) = animate_on_touch_query.get_mut(colliding_entity) {
                if let Ok(mut sa) = sprite_animators.get_mut(aot.animator_entity) {
                    sa.currently_animate = true;
                } 
            }
        }
    }
}