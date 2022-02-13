use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GrowEvent>().add_event::<GameOverEvent>();
    }
}

pub struct GrowEvent;

pub struct GameOverEvent;
