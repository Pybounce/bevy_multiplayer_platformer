
use bevy::prelude::*;

#[derive(Component)]
pub struct Rotator {
    pub speed: f32
}

pub fn rotate_rotators(
    mut query: Query<(&mut Transform, &Rotator)>,
    time: Res<Time>
) {
    for (mut t, r) in &mut query {
        t.rotate(Quat::from_rotation_z(r.speed * time.delta_seconds()));
    }
}