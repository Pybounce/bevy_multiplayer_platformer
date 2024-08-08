use bevy::prelude::*;


#[derive(Component)]
pub struct SpriteAnimator {
    ///Milliseconds between each frame
    atlas_rects: Vec<Rect>,
    current_atlas_index: usize,
    timer: Timer
}

impl SpriteAnimator {
    pub fn new(frame_delay: u128, atlas_rects: Vec<Rect>) -> SpriteAnimator {
        SpriteAnimator {
            atlas_rects: atlas_rects.clone(),
            current_atlas_index: 0,
            timer: Timer::from_seconds(frame_delay as f32 / 1000.0, TimerMode::Repeating)
        }
    }
    pub fn increment_atlas_index(&mut self) {
        self.current_atlas_index += 1;
        if self.current_atlas_index >= self.atlas_rects.len() {
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
        anim_sprite.timer.tick(time.delta());
        if anim_sprite.timer.just_finished() {
            anim_sprite.increment_atlas_index();
            sprite.rect = Some(anim_sprite.get_current_atlas_rect());
        }
    }
}