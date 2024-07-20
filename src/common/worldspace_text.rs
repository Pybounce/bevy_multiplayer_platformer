
use bevy::{prelude::*, text::TextLayoutInfo};

use crate::local_player::LocalPlayer;

#[derive(Component)]
pub struct WorldSpaceText {

}


pub fn setup_ws_text(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_released(KeyCode::KeyK) {
        commands.spawn(TextBundle {
            text: Text::from_section(
                   "pibs",
                   TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -100.0),
                ..default()
            },
            ..default()
        })
        .insert(WorldSpaceText {});
    }

}

pub fn update_ws_text(
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    camera_query: Query<&Transform, With<Camera>>,
    mut text_query: Query<(&mut Style, &TextLayoutInfo), (With<WorldSpaceText>, Without<LocalPlayer>, Without<Camera>)>,
    window_query: Query<&Window, (Without<LocalPlayer>, Without<Camera>, Without<WorldSpaceText>)>
) {
    let w_opt = window_query.get_single();
    if let Err(_) = w_opt { return; }
    let window = w_opt.unwrap();
    
    let c_opt = camera_query.get_single();
    if let Err(_) = c_opt { return; }
    let camera_transform = c_opt.unwrap();

    let p_opt = player_query.get_single();
    if let Err(_) = p_opt { return; }
    let player_transform = p_opt.unwrap();

    for (mut s, tli) in &mut text_query {
        let player_camera_offset = player_transform.translation - camera_transform.translation;
        s.bottom = Val::Px((window.height() / 2.0) + player_camera_offset.y + tli.logical_size.y);
        s.left = Val::Px((window.width() / 2.0) + player_camera_offset.x - (tli.logical_size.x / 2.0));
    }
    
    
}