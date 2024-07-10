use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::local_player::LocalPlayer;





pub fn check_out_of_bounds(
    mut query: Query<&mut Transform, With<LocalPlayer>>
) {
    for mut t in &mut query {
        if t.translation.y < -1000.0 {
            t.translation = Vec3::new(100.0, 100.0, 0.0);
        }
    }
}


