
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PhysicsController {
    pub max_velocity: Vec2,
    pub min_velocity: Vec2
}

pub fn apply_physics_controller_limits(
    mut query: Query<(&mut Velocity, &PhysicsController)>
) {
    for (mut v, pc) in &mut query {
        v.linvel.x = v.linvel.x.max(pc.min_velocity.x).min(pc.max_velocity.x);
        v.linvel.y = v.linvel.y.max(pc.min_velocity.y).min(pc.max_velocity.y);
    }
}