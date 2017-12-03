use std::io::{BufRead};
use game::tile::Tile;
use std::ops::Range;

const COLORS: &'static str = "blry";
const NUM_JOKERS: [u8; 3] = [0, 1, 2];
const RUN_LENGTHS: [u8; 3] = [3, 4, 5];
const MIN_VALUE: u8 = 1;
const MAX_VALUE: u8 = 13;

pub fn read_line<R: BufRead>(reader: &mut R) -> String {
	let mut line = String::new();

	reader.read_line(&mut line)
		.expect("Couldn't read line");

	String::from(line.trim())
}


pub fn parse_number_range(range: &str) -> Range<u8> {

	let mut range = range
		.split('-')
		.map(|x| x.parse::<u8>().unwrap());

	let a = range.next().unwrap();
	let b = range.next().unwrap_or(a);	// If there is no end bound, make b equal a

	(a..(b+1))
}

fn generate_sequential_runs(color: char, length: u8, joker_count: u8) -> Vec<Vec<Tile>> {
	(MIN_VALUE..(MAX_VALUE - length + joker_count + 2)).map(|x| {
		let mut tiles = Vec::new();
		let mut val = x;

		// FIXME: I don't handle jokers correctly I think
		for l in 0..length {
			if l < joker_count {
				tiles.push(Tile::new('j', None));
			} else {
				tiles.push(Tile::new(color, Some(val)));
				val += 1;
			}
		}

		tiles
	}).collect()
}

/// Generate all the sequential runs
///
/// Generates all sequential runs for all colors. Also includes runs with 0 or more jokers.
pub fn generate_all_sequential_runs() -> Vec<Vec<Tile>> {
	// for each color
	let mut tiles = Vec::new();
	for color in COLORS.chars() {
		for &length in RUN_LENGTHS.iter() {
			for &joker_count in NUM_JOKERS.iter() {
				tiles.append(&mut generate_sequential_runs(color, length, joker_count));
			}
		}
	}

	tiles
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_sequential_runs_test() {
		assert_eq!(
			vec![
				vec![
					Tile::new('y', Some(1)),
					Tile::new('y', Some(2)),
					Tile::new('y', Some(3)),
					Tile::new('y', Some(4)),
					Tile::new('y', Some(5)),
				],
				vec![
					Tile::new('y', Some(2)),
					Tile::new('y', Some(3)),
					Tile::new('y', Some(4)),
					Tile::new('y', Some(5)),
					Tile::new('y', Some(6)),
				],
				vec![
					Tile::new('y', Some(3)),
					Tile::new('y', Some(4)),
					Tile::new('y', Some(5)),
					Tile::new('y', Some(6)),
					Tile::new('y', Some(7)),
				],
				vec![
					Tile::new('y', Some(4)),
					Tile::new('y', Some(5)),
					Tile::new('y', Some(6)),
					Tile::new('y', Some(7)),
					Tile::new('y', Some(8)),
				],
				vec![
					Tile::new('y', Some(5)),
					Tile::new('y', Some(6)),
					Tile::new('y', Some(7)),
					Tile::new('y', Some(8)),
					Tile::new('y', Some(9)),
				],
				vec![
					Tile::new('y', Some(6)),
					Tile::new('y', Some(7)),
					Tile::new('y', Some(8)),
					Tile::new('y', Some(9)),
					Tile::new('y', Some(10)),
				],
				vec![
					Tile::new('y', Some(7)),
					Tile::new('y', Some(8)),
					Tile::new('y', Some(9)),
					Tile::new('y', Some(10)),
					Tile::new('y', Some(11)),
				],
				vec![
					Tile::new('y', Some(8)),
					Tile::new('y', Some(9)),
					Tile::new('y', Some(10)),
					Tile::new('y', Some(11)),
					Tile::new('y', Some(12)),
				],
				vec![
					Tile::new('y', Some(9)),
					Tile::new('y', Some(10)),
					Tile::new('y', Some(11)),
					Tile::new('y', Some(12)),
					Tile::new('y', Some(13)),
				],
			],
			generate_sequential_runs('y', 5, 0)
		);
	}

	// TODO: do these tests better
	#[test]
	fn parse_number_range_zero() {
		assert_eq!(
			vec![0],
			parse_number_range("0").collect::<Vec<u8>>()
		);
	}

	#[test]
	fn parse_number_range_seven() {
		assert_eq!(
			vec![7],
			parse_number_range("7").collect::<Vec<u8>>()
		);
	}

	#[test]
	fn parse_number_range_zero_to_zero() {
		assert_eq!(
			vec![0],
			parse_number_range("0-0").collect::<Vec<u8>>()
		);
	}

	#[test]
	fn parse_number_range_seven_to_thirteen() {
		assert_eq!(
			vec![7,8,9,10,11,12,13],
			parse_number_range("7-13").collect::<Vec<u8>>()
		);
	}

	#[test]
	fn parse_number_range_zero_to_eleven() {
		assert_eq!(
			vec![0,1,2,3,4,5,6,7,8,9,10,11],
			parse_number_range("0-11").collect::<Vec<u8>>()
		);
	}

	#[test]
	fn parse_number_range_min_to_max() {
		assert_eq!(
			(u8::min_value()..u8::max_value()).collect::<Vec<u8>>(),
			parse_number_range("0-254").collect::<Vec<u8>>()
		);
	}
}
