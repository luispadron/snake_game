use bevy::{core::FixedTimestep, prelude::*};
use rand::random;

use crate::constants::*;
use crate::events::*;
use crate::models::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(rand::random::<f64>() * 3. + 0.5))
                    .with_system(spawn_food),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(SNAKE_SPEED))
                    .with_system(snake_movement_input.label(SnakeState::Input))
                    .with_system(
                        snake_movement
                            .label(SnakeState::Move)
                            .after(SnakeState::Input),
                    )
                    .with_system(snake_eating.label(SnakeState::Eat).after(SnakeState::Move)),
            )
            .add_system(snake_collision.after(SnakeState::Move))
            .add_system(GameOverEvent::system.after(SnakeState::Move))
            .add_system(GrowEvent::system.after(SnakeState::Eat));
    }
}

pub fn spawn_snake(mut cmd: Commands, mut segments: ResMut<SnakeSegments>) {
    let snake_head = cmd
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Snake {
            direction: SnakeDirection::default(),
        })
        .insert(SnakeSegment)
        .insert(Position { x: 3, y: 3 })
        .insert(EntitySize::square(0.8))
        .id();

    segments.0 = vec![snake_head, spawn_segment(&mut cmd, Position { x: 3, y: 2 })];
}

pub fn spawn_food(mut cmd: Commands) {
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
    .insert(EntitySize::square(0.8));
}

pub fn spawn_segment(cmd: &mut Commands, pos: Position) -> Entity {
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
        .insert(EntitySize::square(0.65))
        .id();
}

pub fn snake_movement_input(input: Res<Input<KeyCode>>, mut snake_q: Query<&mut Snake>) {
    let mut snake = snake_q.iter_mut().next().unwrap();

    let direction = if input.pressed(KeyCode::A) {
        SnakeDirection::Left
    } else if input.pressed(KeyCode::D) {
        SnakeDirection::Right
    } else if input.pressed(KeyCode::S) {
        SnakeDirection::Down
    } else if input.pressed(KeyCode::W) {
        SnakeDirection::Up
    } else {
        snake.direction
    };

    if direction != snake.direction.opposite() {
        snake.direction = direction;
    }
}

pub fn snake_movement(snake_q: Query<&Snake>, mut q: Query<(&mut Position, With<SnakeSegment>)>) {
    let snake = snake_q.iter().next().unwrap();

    let delta = match snake.direction {
        SnakeDirection::Left => (-1, 0),
        SnakeDirection::Right => (1, 0),
        SnakeDirection::Up => (0, 1),
        SnakeDirection::Down => (0, -1),
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

pub fn snake_collision(
    mut game_over_writer: EventWriter<GameOverEvent>,
    snake_q: Query<&Position, With<Snake>>,
    collidable_q: Query<&Position, (Without<Snake>, Without<Food>)>,
) {
    let snake_pos = *snake_q.iter().next().unwrap();

    // If snake is outside arena bounds => game over
    if snake_pos.x >= ARENA_WIDTH as i32
        || snake_pos.x < 0
        || snake_pos.y >= ARENA_HEIGHT as i32
        || snake_pos.y < 0
    {
        game_over_writer.send(GameOverEvent);
    }

    // If snake has collided with it's own segments
    if collidable_q.iter().any(|&c| c == snake_pos) {
        game_over_writer.send(GameOverEvent);
    }
}

pub fn snake_eating(
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

impl GrowEvent {
    fn system(
        mut cmd: Commands,
        mut segments: ResMut<SnakeSegments>,
        mut score: ResMut<Score>,
        mut grow_reader: EventReader<GrowEvent>,
        segments_q: Query<(Entity, &mut Position, With<SnakeSegment>)>,
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
        score.0 += 1;
    }
}

impl GameOverEvent {
    fn system(
        mut cmd: Commands,
        mut game_over_reader: EventReader<GameOverEvent>,
        segments: ResMut<SnakeSegments>,
        snake_q: Query<Entity, With<Snake>>,
        segments_q: Query<Entity, With<SnakeSegment>>,
        food_q: Query<Entity, With<Food>>,
    ) {
        if !game_over_reader.iter().next().is_some() {
            return;
        }

        snake_q
            .iter()
            .chain(segments_q.iter())
            .chain(food_q.iter())
            .for_each(|e| {
                cmd.entity(e).despawn();
            });

        // Restart the game
        spawn_snake(cmd, segments);
    }
}
