use bevy::prelude::*;

use crate::events::*;
use crate::models::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_system(score_text)
            .add_system(high_score_text);
    }
}

fn load_assets(mut cmd: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 32.0,
        color: Color::WHITE,
    };
    let score_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Left,
    };
    let high_score_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Right,
    };

    let window = windows.get_primary().unwrap();

    cmd.spawn_bundle(Text2dBundle {
        text: Text::with_section("Score: 0", text_style.clone(), score_alignment),
        transform: Transform {
            translation: Vec3::new(-window.width() / 2. + 16., window.height() / 2. - 32., 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ScoreText);

    cmd.spawn_bundle(Text2dBundle {
        text: Text::with_section("High scores: 0", text_style.clone(), high_score_alignment),
        transform: Transform {
            translation: Vec3::new(window.width() / 2. - 16., window.height() / 2. - 32., 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(HighScoreText);
}

fn score_text(
    mut score: ResMut<Score>,
    mut text_q: Query<&mut Text, With<ScoreText>>,
    mut game_over_reader: EventReader<GameOverEvent>,
) {
    if game_over_reader.iter().next().is_some() {
        score.0 = 0;
    }

    for mut text in text_q.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.0.to_string());
    }
}

fn high_score_text(
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
    mut text_q: Query<&mut Text, With<HighScoreText>>,
    mut game_over_reader: EventReader<GameOverEvent>,
) {
    if game_over_reader.iter().next().is_some() {
        high_score.0 = high_score.0.max(score.0);
    }

    for mut text in text_q.iter_mut() {
        text.sections[0].value = format!("High score: {}", high_score.0.to_string());
    }
}
