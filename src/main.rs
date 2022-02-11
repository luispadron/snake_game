use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(spawn_food),
        )
        .add_system(
            snake_movement_input
                .label(SnakeState::Input)
                .before(SnakeState::Move),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement.label(SnakeState::Move)),
        )
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

const ARENA_WIDTH: u32 = 20;
const ARENA_HEIGHT: u32 = 20;

#[derive(Component)]
struct Snake {
    direction: Direction,
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, Copy, PartialEq)]
struct Size {
    w: f32,
    h: f32,
}

impl Size {
    pub fn square(w: f32) -> Self {
        Self { w, h: w }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SnakeState {
    Input,
    Move,
    Eat,
    Grow,
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            size.w / ARENA_WIDTH as f32 * window.width() as f32,
            size.h / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
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

fn spawn_snake(mut cmd: Commands) {
    let snake_color = Color::rgb(random(), random(), random());

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: snake_color,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(10., 10., 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Snake {
        direction: Direction::Up,
    })
    .insert(Position { x: 0, y: 0 })
    .insert(Size::square(0.8));
}

fn spawn_food(mut cmd: Commands) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Food)
    .insert(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    })
    .insert(Size::square(0.8));
}

fn snake_movement_input(input: Res<Input<KeyCode>>, mut snakes: Query<&mut Snake>) {
    for mut snake in snakes.iter_mut() {
        let direction = if input.pressed(KeyCode::A) {
            Direction::Left
        } else if input.pressed(KeyCode::D) {
            Direction::Right
        } else if input.pressed(KeyCode::S) {
            Direction::Down
        } else if input.pressed(KeyCode::W) {
            Direction::Up
        } else {
            snake.direction
        };

        if direction != snake.direction.opposite() {
            snake.direction = direction;
        }
    }
}

fn snake_movement(mut q: Query<(&mut Position, &Snake)>) {
    for (mut position, snake) in q.iter_mut() {
        match snake.direction {
            Direction::Left => position.x -= 1,
            Direction::Right => position.x += 1,
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
        }
    }
}
