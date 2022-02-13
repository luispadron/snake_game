use bevy::{prelude::*, window::WindowMode};

use crate::constants::*;
use crate::models::*;

pub struct SnakeCameraPlugin;

impl Plugin for SnakeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 800.,
            height: 800.,
            vsync: false,
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        );
    }
}

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&EntitySize, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            size.w / ARENA_WIDTH as f32 * window.width() as f32,
            size.h / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.,
        );
    }
}
