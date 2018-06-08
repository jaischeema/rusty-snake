extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::prelude::*;

const GRID_SIZE: i32 = 32;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Position(i32, i32);

fn random_position() -> Position {
    let rng = thread_rng();
    Position(
        rng.to_owned().gen_range(0, GRID_SIZE),
        rng.to_owned().gen_range(0, GRID_SIZE).clone(),
    )
}

#[derive(Debug, Clone)]
struct RunningState {
    direction: Direction,
    snake: Vec<Position>,
    marker: Option<Position>,
    collected: i32,
}

fn main() {
    let mut state = RunningState {
        direction: Direction::Right,
        snake: vec![Position(0, 0)],
        marker: None,
        collected: 0,
    };

    let mut window: PistonWindow = WindowSettings::new("Snakes", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let level = 10;
    let mut marker_timer = 0;
    window.set_ups(level);

    while let Some(event) = window.next() {
        let direction = match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => Direction::Up,
            Some(Button::Keyboard(Key::Down)) => Direction::Down,
            Some(Button::Keyboard(Key::Left)) => Direction::Left,
            Some(Button::Keyboard(Key::Right)) => Direction::Right,
            _ => state.direction.to_owned(),
        };

        state = RunningState {
            direction: direction,
            ..state
        };

        if let Some(_) = event.update_args() {
            marker_timer += 1;
            let (snake, collected_marker) = update_snake(&state);
            let marker = if collected_marker || (marker_timer % (level * 5)) == 0 {
                state.collected += 1;
                marker_timer = 0;
                Some(spawn_marker(&snake))
            } else {
                state.marker
            };

            state = RunningState {
                snake: snake,
                marker: marker,
                ..state
            };
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            for item in &state.snake {
                let x = (item.0 as f64) * 10.0;
                let y = (item.1 as f64) * 10.0;
                rectangle(
                    [0.0, 0.0, 0.0, 1.0], // red
                    [x, y, 10.0, 10.0],
                    context.transform,
                    graphics,
                );
            }

            if let Some(marker) = &state.marker {
                let x = (marker.0 as f64) * 10.0;
                let y = (marker.1 as f64) * 10.0;
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // red
                    [x, y, 10.0, 10.0],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}

fn spawn_marker(snake: &Vec<Position>) -> Position {
    let marker = random_position();
    if snake.contains(&marker) {
        spawn_marker(snake)
    } else {
        marker
    }
}

fn update_snake(state: &RunningState) -> (Vec<Position>, bool) {
    if let Some(element) = state.snake.first() {
        let mut updated = state.snake.clone();
        let head = increment_position(element, &state.direction);
        let collected_marker = if let Some(existing_marker) = &state.marker {
            existing_marker == &head
        } else {
            false
        };
        updated.insert(0, head);
        if !collected_marker {
            updated.pop();
        }
        (updated, collected_marker)
    } else {
        (vec![Position(0, 0)], false)
    }
}

fn increment_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position(position.0, position.1 - 1),
        Direction::Down => Position(position.0, position.1 + 1),
        Direction::Right => Position(position.0 + 1, position.1),
        Direction::Left => Position(position.0 - 1, position.1),
    }
}
