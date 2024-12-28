
mod ioscreen;
mod game;
use game::Game;

fn main()
{
    let mut game_instance = Game::new();
    game_instance.game_loop();
}