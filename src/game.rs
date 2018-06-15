use piston_window::*;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position(i32, i32);

type Snake = Vec<Position>;

#[derive(Debug)]
pub struct RunningState {
    direction: Direction,
    snake: Snake,
    marker: Option<Position>,
    collected: i32,
}

#[derive(Debug)]
pub enum Game {
    NotStarted,
    Running(RunningState),
    Finished(i32),
}

pub const GAME_LEVEL: i32 = 15;
pub const GRID_SIZE: i32 = 32;
pub const GRID_PIXEL_SIZE: f64 = 10.0;

pub fn new() -> Game {
    Game::NotStarted
}

impl Game {
    pub fn start(&self) -> Game {
        Game::Running(RunningState {
            direction: Direction::Right,
            snake: vec![
                Position(10, 10),
                Position(10, 11),
                Position(10, 12),
                Position(10, 13),
                Position(10, 14),
                Position(10, 15),
                Position(10, 16),
            ],
            marker: None,
            collected: 0,
        })
    }

    pub fn tick(self, counter: i32) -> (Game, i32) {
        let mut local_counter = counter + 1;
        match self {
            Game::NotStarted => {
                println!("Not Started");
                (self, local_counter)
            }
            Game::Running(mut state) => {
                let (snake, collected_marker, collision) = update_snake(&state);
                if collected_marker {
                    state.collected += 1;
                }
                if collected_marker || local_counter % (GAME_LEVEL * 5) == 0 {
                    local_counter = 0;
                    state.marker = Some(spawn_marker(&snake))
                };
                state.snake = snake;
                if collision {
                    (Game::Finished(state.collected), local_counter)
                } else {
                    (Game::Running(state), local_counter)
                }
            }
            Game::Finished(_) => {
                println!("Game Ended");
                (self, local_counter)
            }
        }
    }

    pub fn change_direction(&mut self, direction: Option<Direction>) {
        if let Game::Running(state) = self {
            match direction {
                Some(Direction::Up) => if state.direction != Direction::Down {
                    state.direction = Direction::Up
                },
                Some(Direction::Down) => if state.direction != Direction::Up {
                    state.direction = Direction::Down
                },
                Some(Direction::Left) => if state.direction != Direction::Right {
                    state.direction = Direction::Left
                },
                Some(Direction::Right) => if state.direction != Direction::Left {
                    state.direction = Direction::Right
                },
                None => (),
            }
        }
    }

    pub fn render(&self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        clear([1.0; 4], graphics);

        let grid_size = GRID_SIZE as f64;
        rectangle(
            [0.86, 0.86, 0.86, 0.8],
            [
                0.0,
                0.0,
                GRID_PIXEL_SIZE * grid_size,
                GRID_PIXEL_SIZE * grid_size,
            ],
            context.transform,
            graphics,
        );
        match self {
            Game::NotStarted => println!("Do nothing"),
            Game::Running(state) => {
                for item in &state.snake {
                    let x = (item.0 as f64) * GRID_PIXEL_SIZE;
                    let y = (item.1 as f64) * GRID_PIXEL_SIZE;
                    rectangle(
                        [0.0, 0.0, 0.0, 1.0],
                        [x, y, GRID_PIXEL_SIZE, GRID_PIXEL_SIZE],
                        context.transform,
                        graphics,
                    );
                }

                if let Some(marker) = &state.marker {
                    let x = (marker.0 as f64) * GRID_PIXEL_SIZE;
                    let y = (marker.1 as f64) * GRID_PIXEL_SIZE;
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red
                        [x, y, GRID_PIXEL_SIZE, GRID_PIXEL_SIZE],
                        context.transform,
                        graphics,
                    );
                }

                let transform = context.transform.trans(100.0, 400.0);
                text::Text::new_color([0.64, 0.11, 0.11, 1.0], 32).draw(
                    &format!("Score: {}", &state.collected),
                    glyphs,
                    &context.draw_state,
                    transform,
                    graphics,
                );
            }
            Game::Finished(_) => {
                let transform = context.transform.trans(30.0, 100.0);
                text::Text::new_color([0.64, 0.11, 0.11, 1.0], 32).draw(
                    "Game Finished!",
                    glyphs,
                    &context.draw_state,
                    transform,
                    graphics,
                );
            }
        }
    }
}

fn spawn_marker(snake: &Snake) -> Position {
    let marker = random_position();
    if snake.contains(&marker) {
        spawn_marker(snake)
    } else {
        marker
    }
}

fn update_snake(state: &RunningState) -> (Snake, bool, bool) {
    if let Some(element) = state.snake.first() {
        let mut updated = state.snake.clone();
        let head = increment_position(element, &state.direction);
        let collected_marker = if let Some(existing_marker) = &state.marker {
            existing_marker == &head
        } else {
            false
        };
        if !collected_marker {
            updated.pop();
        }
        let collision = updated.contains(&head) || outside_boundary(&head);
        updated.insert(0, head);
        (updated, collected_marker, collision)
    } else {
        (vec![Position(0, 0)], false, false)
    }
}

fn outside_boundary(position: &Position) -> bool {
    let x = position.0;
    let y = position.1;
    x < 0 || x >= GRID_SIZE || y < 0 || y >= GRID_SIZE
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
