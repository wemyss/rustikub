use self::tile::*;
pub mod tile;
pub mod utils;


// use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader};


#[derive(Debug)]
pub struct Game {
	board: Vec<Tile>,
	hand: Vec<Tile>,
}

fn parse(tiles: &str) -> Vec<Tile> {

	tiles.split_whitespace()
		.flat_map(|tile| {
			let (colors, nums) = tile.split_at(tile.find(char::is_numeric).unwrap());
			let nums = utils::parse_number_range(nums);

			println!("{:?}", colors);
			println!("{:?}", nums);
			nums.flat_map(move |num| colors.chars().map(move |c| Tile::new(c, num)))
		})
		.collect()
}



impl Game {
	// add code here
	pub fn new() -> Game {
		Game {
			board: Vec::new(),
			hand: Vec::new(),
		}
	}

	pub fn load(filename: &str) -> Game {
		let f = File::open("data.txt")
			.expect("Failed to open file");

		// check line length == 2
		let mut reader = BufReader::new(f);

		let board = utils::read_line(&mut reader);
		let hand = utils::read_line(&mut reader);

		println!("BOARD: {:?}", board);
		println!("HAND: {:?}", hand);

		Game {
			board: parse(&board),
			hand: parse(&hand),
		}
	}
}
