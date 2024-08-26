use bevy::prelude::*;

#[derive(Component)]
pub struct FPSUI;

pub fn setup_fps_stuff(mut commands: Commands) {
    commands.insert_resource(FrameRate::new(60));
    commands
        .spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new("FPS   ", TextStyle {
                    font_size: 32.0,
                    color: Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
                    ..Default::default()
                }),
                TextSection::from_style(TextStyle {
                    font_size: 32.0,
                    color: Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
                    ..Default::default()
                }),
            ]),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        })
        .insert(FPSUI);
}

pub fn update_fps_ui(
    mut query: Query<&mut Text, With<FPSUI>>,
    time: Res<Time>,
    mut frame_rate: ResMut<FrameRate>
) {
    let mut text = query.single_mut();

    frame_rate.update(time.delta_seconds_f64());

    text.sections[1].value = format!("{:.2}", frame_rate.average_fps()).to_string();
}

#[derive(Resource, Default)]
pub struct FrameRate {
    frame_times: std::collections::VecDeque<f64>,
    max_frames: usize,
}

impl FrameRate {
    pub fn new(max_frames: usize) -> Self {
        FrameRate {
            frame_times: std::collections::VecDeque::with_capacity(max_frames),
            max_frames,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.frame_times.len() == self.max_frames {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(delta_time);
    }

    pub fn average_fps(&self) -> f64 {
        let sum: f64 = self.frame_times.iter().sum();
        (self.frame_times.len() as f64) / sum
    }
}