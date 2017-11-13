use self::tile::*;
pub mod tile;
pub mod utils;

use std::fs::File;
use std::io::{BufReader};


#[derive(Debug)]
pub struct Game {
	board: Vec<Tile>,
	hand: Vec<Tile>,
}

impl Game {

	pub fn new() -> Game {
		Game {
			board: Vec::new(),
			hand: Vec::new(),
		}
	}

	/// Creates struct representing the game in the file
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
			board: parse_tiles(&board),
			hand: parse_tiles(&hand),
		}
	}

	pub fn validate_set() {

	}

	pub fn solve() {}
}

/// Converts a string representing tiles into a vector of Tile objects
///
/// `parse_tiles()` returns a vector of tiles. Each tile includes the color and
/// numerical value of that tile. e.g Red 7
///
/// # Examples
/// ```
/// parse_tiles("b1 blry5 r9-12 lyj7")
/// ```
fn parse_tiles(tiles: &str) -> Vec<Tile> {
	let mut parsed_tiles = Vec::new();

	for tile in tiles.split_whitespace() {
		match tile.find(char::is_numeric) {
			Some(x) => {
				let (colors, nums) = tile.split_at(x);
				let nums = utils::parse_number_range(nums);

				for num in nums {
					for c in colors.chars() {
						parsed_tiles.push(Tile::new(c, Some(num)));
					}
				}
			},
			None => {
				let colors = tile;

				for c in colors.chars() {
					parsed_tiles.push(Tile::new(c, None));
				}
			}
		}
	}

	parsed_tiles
}




#[cfg(test)]
mod tests {
	use super::*;

	// TODO: Empty tiles test

	#[test]
	fn parse_tiles_basic() {
		let expected = vec![
			Tile::new('b', Some(1)),
			Tile::new('l', Some(2)),
			Tile::new('r', Some(3)),
			Tile::new('y', Some(4)),
		];
		assert_eq!(expected, parse_tiles("b1 l2 r3 y4"));
	}

	#[test]
	fn parse_tiles_jokers() {
		let expected = vec![
			Tile::new('j', None),
			Tile::new('j', None),
			Tile::new('j', None),
			Tile::new('j', None),
		];
		assert_eq!(expected, parse_tiles("j j13 j1-2"));
	}
}
