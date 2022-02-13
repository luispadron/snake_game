use bevy::{prelude::*, window::WindowMode};

mod camera;
mod constants;
mod events;
mod models;
mod snake;

use camera::*;
use events::*;
use models::*;
use snake::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 800.,
            height: 800.,
            vsync: false,
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(EventsPlugin)
        .add_plugin(ModelsPlugin)
        .add_plugin(SnakeCameraPlugin)
        .add_plugin(SnakePlugin)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}
