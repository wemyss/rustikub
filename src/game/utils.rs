use std::io::{BufRead};
use game::tile::Tile;
use std::ops::Range;

const COLORS: &'static str = "blry";
const NUM_JOKERS: [u8; 3] = [0, 1, 2];
const RUN_LENGTHS: [u8; 3] = [3, 4, 5];
const VALUES: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
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


/// Generate all number combinations for a given run size and a tile pool to choose from
fn generate_combinations(tile_set: &[u8], run_size: usize) -> Vec<Vec<u8>> {
	match tile_set.is_empty() {
		true =>
			match run_size {
				0 => vec![vec![]],
				_ => vec![],
			},
		false => {
			match run_size {
				0 => vec![vec![]],
				_ => {
					let (head, tail) = tile_set.split_at(1);
					let head = head[0].clone();

					let mut combinations = generate_combinations(&tail, run_size - 1);

					for c in &mut combinations {
						c.insert(0, head);
					}
					combinations.append(&mut generate_combinations(&tail, run_size));

					combinations
				}
			}
		}
	}
}

fn generate_sequential_runs(color: char, length: usize, joker_count: u8) -> Vec<Vec<Tile>> {
	let mut runs: Vec<Vec<Tile>> = vec![];

	for i in 0..(MAX_VALUE as usize - length + 1) {
		let combinations = generate_combinations(&VALUES[i..(i+length)], length - joker_count as usize);
		for c in combinations {
			runs.push(c
				.iter()
				.map(|t| Tile::new(color, Some(t.clone())))
				.collect()
			);
		}
	}


	if joker_count > 0 {
		// Add the joker/s in to each run
		for r in &mut runs {
			for _ in 0..joker_count  {
				(*r).push(Tile::new('j', None));
			}
		}
	}
	runs
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
				tiles.append(&mut generate_sequential_runs(color, length as usize, joker_count));
			}
		}
	}

	tiles
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_combinations_test_runsize_3() {
		assert_eq!(
			generate_combinations(&[1,2,3,4,5], 2),
			[[1,2,3],[1,2,4],[1,2,5],[1,3,4],[1,3,5],[1,4,5],[2,3,4],[2,3,5],[2,4,5],[3,4,5]]
		);
	}

	#[test]
	fn generate_combinations_test_runsize_4() {
		assert_eq!(
			generate_combinations(&[8,9,10,11,12,13], 4),
			[
				[8,9,10,11],
				[8,9,10,12],
				[8,9,10,13],
				[8,9,11,12],
				[8,9,11,13],
				[8,9,12,13],
				[8,10,11,12],
				[8,10,11,13],
				[8,10,12,13],
				[8,11,12,13],
				[9,10,11,12],
				[9,10,11,13],
				[9,10,12,13],
				[9,11,12,13],
				[10,11,12,13]
			]
		);
	}

	#[test]
	fn generate_combinations_test_runsize_5() {
		assert_eq!(
			generate_combinations(&[5,6,7,8,9,10], 5),
			[[5,6,7,8,9],[5,6,7,8,10],[5,6,7,9,10],[5,6,8,9,10],[5,7,8,9,10],[6,7,8,9,10]]
		);
	}


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
			generate_sequential_runs('y', 3, 1)
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
