use self::tile::*;
pub mod tile;
pub mod utils;


// use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader};



pub struct Game {
	board: Vec<Tile>,
	hand: Vec<Tile>,
}

impl Game {
	// add code here
	pub fn new() -> Game {
		Game {
			board: Vec::new(),
			hand: Vec::new(),
		}
	}

	pub fn load(&self, filename: &str) {
		let f = File::open("data.txt")
			.expect("Failed to open file");

		// check line length == 2
		let mut reader = BufReader::new(f);

		let board = utils::read_line(&mut reader);
		let hand = utils::read_line(&mut reader);

		println!("{:?}", board);
		println!("{:?}", hand);
	}

	fn parse(&self, board: &str, hand: &str) {
		self.board =
			board.split_whitespace()
			.map(|tile| Tile::new(&tile))
			.collect();
	}
}
