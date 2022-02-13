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
        .insert_resource(SnakeSegments::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_event::<GrowEvent>()
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(rand::random::<f64>() * 3. + 0.5))
                .with_system(spawn_food),
        )
        .add_system(
            snake_movement_input
                .label(SnakeState::Input)
                .before(SnakeState::Move),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SNAKE_SPEED))
                .with_system(snake_movement.label(SnakeState::Move))
                .with_system(snake_eating.label(SnakeState::Eat).after(SnakeState::Move))
                .with_system(snake_grow.label(SnakeState::Grow).after(SnakeState::Eat)),
        )
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

const ARENA_WIDTH: u32 = 20;
const ARENA_HEIGHT: u32 = 20;
const SNAKE_SPEED: f64 = 0.12;

#[derive(Component)]
struct Snake {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

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

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum SnakeState {
    Input,
    Move,
    Eat,
    Grow,
}

struct GrowEvent;

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

fn spawn_snake(mut cmd: Commands, mut segments: ResMut<SnakeSegments>) {
    let snake_color = Color::rgb(random(), random(), random());
    let snake_head = cmd
        .spawn_bundle(SpriteBundle {
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
            direction: Direction::default(),
        })
        .insert(SnakeSegment)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8))
        .id();

    segments.0 = vec![snake_head, spawn_segment(&mut cmd, Position { x: 3, y: 2 })];
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

fn spawn_segment(cmd: &mut Commands, pos: Position) -> Entity {
    let segment_color = Color::rgb(random(), random(), random());

    return cmd
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: segment_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(pos)
        .insert(Size::square(0.65))
        .id();
}

fn snake_movement_input(input: Res<Input<KeyCode>>, mut snake_q: Query<&mut Snake>) {
    let mut snake = snake_q.iter_mut().next().unwrap();

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

fn snake_movement(snake_q: Query<&Snake>, mut q: Query<(&mut Position, With<SnakeSegment>)>) {
    let snake = snake_q.iter().next().unwrap();

    let delta = match snake.direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
    };

    // Make segments follow the leading segment
    let mut prev_pos: Option<Position> = None;
    for (mut pos, _) in q.iter_mut() {
        if let Some(old_pos) = prev_pos {
            prev_pos = Some(*pos);
            pos.x = old_pos.x;
            pos.y = old_pos.y;
        } else {
            prev_pos = Some(*pos);
            pos.x += delta.0;
            pos.y += delta.1;
        }
    }
}

fn snake_eating(
    mut cmd: Commands,
    mut grow_writer: EventWriter<GrowEvent>,
    snake_q: Query<&Position, With<Snake>>,
    food_q: Query<(Entity, &Position), With<Food>>,
) {
    let snake_pos = snake_q.iter().next().unwrap();
    for (food, food_pos) in food_q.iter() {
        if food_pos == snake_pos {
            cmd.entity(food).despawn();
            grow_writer.send(GrowEvent);
        }
    }
}

fn snake_grow(
    mut cmd: Commands,
    mut segments: ResMut<SnakeSegments>,
    segments_q: Query<(Entity, &mut Position, With<SnakeSegment>)>,
    mut grow_reader: EventReader<GrowEvent>,
) {
    if !grow_reader.iter().next().is_some() {
        return;
    }

    let last_segment = segments.0.last().unwrap();
    let last_segment_pos = segments_q
        .iter()
        .filter(|(entity, _, _)| *entity == *last_segment)
        .last()
        .unwrap()
        .1;

    segments.0.push(spawn_segment(&mut cmd, *last_segment_pos));
}
