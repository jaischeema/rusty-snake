extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod game;

use piston_window::*;

fn main() {
    let mut game = game::new().start();

    let mut window: PistonWindow = WindowSettings::new("Snakes", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut marker_timer = 0;
    window.set_ups(game::GAME_LEVEL as u64);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font = &assets.join("SpaceMono-Regular.ttf");
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();

    while let Some(event) = window.next() {
        let direction = match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => Some(game::Direction::Up),
            Some(Button::Keyboard(Key::Down)) => Some(game::Direction::Down),
            Some(Button::Keyboard(Key::Left)) => Some(game::Direction::Left),
            Some(Button::Keyboard(Key::Right)) => Some(game::Direction::Right),
            _ => None,
        };

        game.change_direction(direction);

        if let Some(_) = event.update_args() {
            let (new_state, updated_marker) = game.tick(marker_timer);
            game = new_state;
            marker_timer = updated_marker;
        }

        window.draw_2d(&event, |context, graphics| {
            game.render(context, graphics, &mut glyphs);
        });
    }
}
