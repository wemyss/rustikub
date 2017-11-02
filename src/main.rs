#![allow(dead_code)]
#![allow(unused_variables)]

use game::Game;
mod game;

fn main() {
	let g = Game::load("data.txt");
}
