use bevy::prelude::*;

pub struct ModelsPlugin;

impl Plugin for ModelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .insert_resource(SnakeSegments::default());
    }
}

#[derive(Default)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct Snake {
    pub direction: SnakeDirection,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct EntitySize {
    pub w: f32,
    pub h: f32,
}

impl EntitySize {
    pub fn square(w: f32) -> Self {
        EntitySize { w, h: w }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum SnakeDirection {
    Left,
    Up,
    Right,
    Down,
}

impl SnakeDirection {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection::Up
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SnakeState {
    Input,
    Move,
    Eat,
}
