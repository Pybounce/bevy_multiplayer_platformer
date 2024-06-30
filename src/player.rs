use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 2.0, 0.0);
const PLAYER_MOVE_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Player {
    move_up_key: KeyCode,
    move_down_key: KeyCode,
    move_right_key: KeyCode,
    move_left_key: KeyCode,
    move_speed: f32,
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(Player {
            move_up_key: KeyCode::KeyW,
            move_down_key: KeyCode::KeyS,
            move_right_key: KeyCode::KeyD,
            move_left_key: KeyCode::KeyA,
            move_speed: PLAYER_MOVE_SPEED,
        })
        .insert(SpriteBundle {
            transform: Transform {
                scale: PLAYER_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        });
}

pub fn move_player(mut query: Query<(&mut Transform, &Player)>, 
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut t, p) in &mut query.iter_mut() {
        let mut new_direction = Vec3::new(0.0, 0.0, 0.0);

        if input.pressed(p.move_up_key) {
            new_direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if input.pressed(p.move_down_key) {
            new_direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if input.pressed(p.move_right_key) {
            new_direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if input.pressed(p.move_left_key) {
            new_direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if new_direction.length() > 0.00001 {
            t.translation += new_direction.normalize() * time.delta_seconds() * p.move_speed;
        }
    }
}