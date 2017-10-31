#![allow(dead_code)]
#![allow(unused_variables)]

// fn main() {
// 	let args: Vec<String> = std::env::args().skip(1).collect();

// 	let tiles: Vec<u8> = args.iter().map(|x|
// 		x.parse().expect("Please only input numbers!")
// 	).collect();

//     println!("Hello, world!. {:?}", tiles);
// }
use game::Game;
mod game;

fn main() {
	let mut g = Game::new();
	g.load("data.txt");
}
