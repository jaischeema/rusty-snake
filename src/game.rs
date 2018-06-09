use piston_window::*;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position(i32, i32);

#[derive(Debug, Clone)]
pub struct RunningState {
    direction: Direction,
    snake: Vec<Position>,
    marker: Option<Position>,
    collected: i32,
}

#[derive(Debug)]
pub enum Game {
    NotStarted,
    Running(RunningState),
    Finished(i32),
}

pub const GAME_LEVEL: i32 = 5;
pub const GRID_SIZE: i32 = 32;

pub fn new() -> Game {
    Game::NotStarted
}

impl Game {
    pub fn start(&self) -> Game {
        Game::Running(RunningState {
            direction: Direction::Right,
            snake: vec![Position(0, 0)],
            marker: None,
            collected: 0,
        })
    }

    pub fn tick(&mut self, counter: i32) -> i32 {
        let mut local_counter = counter + 1;
        match self {
            Game::NotStarted => println!("Not Started"),
            Game::Running(state) => {
                let (snake, collected_marker) = update_snake(&state);
                if collected_marker || local_counter % (GAME_LEVEL * 5) == 0 {
                    state.collected += 1;
                    local_counter = 0;
                    state.marker = Some(spawn_marker(&snake))
                };
                state.snake = snake;
            }
            Game::Finished(_) => println!("Game Ended"),
        }
        local_counter
    }

    pub fn change_direction(&mut self, direction: Option<Direction>) {
        if let Game::Running(state) = self {
            if let Some(direction) = direction {
                state.direction = direction
            }
        }
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
        clear([1.0; 4], graphics);

        match self {
            Game::NotStarted => println!("Do nothing"),
            Game::Running(state) => {
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
            }
            Game::Finished(_) => println!("Finished"),
        }
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

fn random_position() -> Position {
    let rng = thread_rng();
    Position(
        rng.to_owned().gen_range(0, GRID_SIZE),
        rng.to_owned().gen_range(0, GRID_SIZE).clone(),
    )
}
