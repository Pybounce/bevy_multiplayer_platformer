use bevy::prelude::*;


#[derive(Component)]
pub struct AnimatedSprite {
    ///Milliseconds between each frame
    frame_delay: u128,
    atlas_rects: Vec<Rect>,
    current_atlas_index: usize,
    current_time: u128
}

impl AnimatedSprite {
    pub fn new(frame_delay: u128, atlas_rects: Vec<Rect>) -> AnimatedSprite {
        AnimatedSprite {
            frame_delay: frame_delay,
            atlas_rects: atlas_rects.clone(),
            current_atlas_index: 0,
            current_time: 0,
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
    mut query: Query<(&mut AnimatedSprite, &mut Sprite)>,
    time: Res<Time>
) {
    for (mut anim_sprite, mut sprite) in &mut query {

        anim_sprite.current_time += time.delta().as_millis();

        if anim_sprite.current_time >= anim_sprite.frame_delay {

            anim_sprite.current_time -= anim_sprite.frame_delay;
            anim_sprite.increment_atlas_index();

            sprite.rect = Some(anim_sprite.get_current_atlas_rect());

        }
    }
}