
mod ioscreen;
mod game;
use game::Game;

fn main()
{
    ioscreen::init();
    let mut game_instance = Game::new();
    game_instance.game_loop();
    ioscreen::deinit();
}