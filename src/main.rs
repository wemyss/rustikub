mod game;
use crate::game::Game;

fn main() {
	let g = Game::load("data.txt");

	println!("{:?}", g);
}
