#![allow(dead_code)]
#![allow(unused_variables)]

mod game;
use game::Game;

fn main() {
	let g = Game::load("data.txt");

	println!("{:?}", g);
}
