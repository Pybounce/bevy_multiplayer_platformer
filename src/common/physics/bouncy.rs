
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollidingEntities, Velocity};

#[derive(Component)]
pub struct Bouncy {
    pub force: Vec2
}

pub fn check_bouncy_collisions(
    mut bounceable_query: Query<(&mut Velocity, &CollidingEntities)>,
    bouncy_query: Query<&Bouncy>
) {
    for (mut v, colliding_entities) in &mut bounceable_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(b) = bouncy_query.get(colliding_entity) {
                v.linvel += b.force;
            }
        }
    }
}
