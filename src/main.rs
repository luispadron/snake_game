use bevy::prelude::*;

mod camera;
mod constants;
mod events;
mod models;
mod score;
mod snake;

use camera::*;
use events::*;
use models::*;
use score::*;
use snake::*;

fn main() {
    App::new()
        .add_plugin(EventsPlugin)
        .add_plugin(ModelsPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(SnakeCameraPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
