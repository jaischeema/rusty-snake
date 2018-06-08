extern crate piston_window;
extern crate rand;

mod game;

use piston_window::*;

fn main() {
    let mut state = game::start_game();

    let mut window: PistonWindow = WindowSettings::new("Snakes", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut marker_timer = 0;
    window.set_ups(game::GAME_LEVEL as u64);

    while let Some(event) = window.next() {
        let direction = match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => Some(game::Direction::Up),
            Some(Button::Keyboard(Key::Down)) => Some(game::Direction::Down),
            Some(Button::Keyboard(Key::Left)) => Some(game::Direction::Left),
            Some(Button::Keyboard(Key::Right)) => Some(game::Direction::Right),
            _ => None,
        };

        game::change_direction(&mut state, &direction);

        if let Some(_) = event.update_args() {
            game::tick(&mut state, &mut marker_timer);
        }

        window.draw_2d(&event, |context, graphics| {
            game::render(&state, context, graphics);
        });
    }
}
