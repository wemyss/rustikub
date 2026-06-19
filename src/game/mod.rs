use crate::game::tile_runs::color::generate_all_color_runs;
use crate::game::tile_runs::sequential::generate_all_sequential_runs;

use self::tile::*;
pub mod tile;
pub mod utils;
pub mod tile_runs;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};

type CountsKey = Vec<(Tile, u8)>;
type SearchKey = (u8, CountsKey, CountsKey);

struct RunInfo {
	run: Vec<Tile>,
	counts: HashMap<Tile, u8>,
	length: u8,
}

fn sorted_counts(map: &HashMap<Tile, u8>) -> CountsKey {
	let mut entries: Vec<(Tile, u8)> = map.iter().map(|(&tile, &count)| (tile, count)).collect();
	entries.sort_by_key(|(tile, _)| *tile);
	entries
}

fn subtract_counts(
	source: &HashMap<Tile, usize>,
	subtract: &HashMap<Tile, usize>,
) -> Option<HashMap<Tile, usize>> {
	let mut result = source.clone();
	for (tile, &count) in subtract.iter() {
		let current = result.get(tile).copied().unwrap_or(0);
		if count > current {
			return None;
		}
		let remaining = current - count;
		if remaining > 0 {
			result.insert(*tile, remaining);
		} else {
			result.remove(tile);
		}
	}
	Some(result)
}

fn subtract_board_counts(
	board_remaining: &HashMap<Tile, usize>,
	run_counts: &HashMap<Tile, usize>,
) -> HashMap<Tile, usize> {
	let mut updated = board_remaining.clone();
	for (tile, &count) in run_counts.iter() {
		if let Some(current) = updated.get_mut(tile) {
			let used = std::cmp::min(*current, count);
			*current -= used;
			if *current == 0 {
				updated.remove(tile);
			}
		}
	}
	updated
}

fn search_runs(
	index: usize,
	remaining_total: &HashMap<Tile, usize>,
	remaining_board: &HashMap<Tile, usize>,
	run_infos: &[RunInfo],
	memo: &mut HashMap<SearchKey, Option<(usize, Vec<Vec<Tile>>) >>,
) -> Option<(usize, Vec<Vec<Tile>>)> {
	if index == run_infos.len() {
		return if remaining_board.is_empty() {
			Some((0, Vec::new()))
		} else {
			None
		};
	}

	let key = (
		index,
		sorted_counts(remaining_total),
		sorted_counts(remaining_board),
	);
	if let Some(cached) = memo.get(&key) {
		return cached.clone();
	}

	let mut best = search_runs(index + 1, remaining_total, remaining_board, run_infos, memo);
	let run_info = &run_infos[index];

	if let Some(next_total) = subtract_counts(remaining_total, &run_info.counts) {
		let next_board = subtract_board_counts(remaining_board, &run_info.counts);
		if let Some((score, mut runs)) = search_runs(index + 1, &next_total, &next_board, run_infos, memo) {
			let score = score + run_info.length;
			if best.as_ref().map_or(true, |(best_score, _)| score > *best_score) {
				runs.insert(0, run_info.run.clone());
				best = Some((score, runs));
			}
		}
	}

	memo.insert(key, best.clone());
	best
}


#[derive(Debug)]
pub struct Game {
	board: Vec<Tile>,
	hand: Vec<Tile>,
}

impl Game {

	/// Creates struct representing the game in the file
	pub fn load(filename: &str) -> Game {
		let f = File::open(filename)
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

    #[allow(dead_code)]
	pub fn solve(&self) -> Result<Vec<Vec<Tile>>, String> {
		let mut input_tiles: HashMap<Tile, u8> = tiles_to_counts(self.board.iter().chain(self.hand.iter()).collect());
		let mut board_only_tiles: HashMap<Tile, u8> = tiles_to_counts(self.board);

		let mut candidates = generate_all_sequential_runs()
			.into_iter()
			.chain(generate_all_color_runs())
			.collect::<Vec<_>>();

		let mut run_infos = Vec::new();
		for run in candidates {
			let mut counts: HashMap<Tile, u8> = tiles_to_counts(run);

			if counts.iter().all(|(key, &count)| input_tiles.get(key).is_some_and(|&v| count < v)) {
				let length = run.len();
				run_infos.push(RunInfo { run, counts, length });
			}
		}

		run_infos.sort_by_key(|info| usize::MAX - info.length);

		let mut memo: HashMap<SearchKey, Option<(usize, Vec<Vec<Tile>>)>> = HashMap::new();
		let solution = search_runs(0, &input_tiles, &board_only_tiles, &run_infos, &mut memo)
			.ok_or_else(|| String::from("No valid tile placement found"))?;

		Ok(solution.1)
	}
}


fn tiles_to_counts(tiles: impl IntoIterator<Item = Tile>) -> HashMap<Tile, u8> {
	let mut counts = HashMap::new();
	for tile in tiles {
		*counts.entry(tile).or_default() += 1;
	}
	counts
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

	#[test]
	fn solve_returns_best_hand_run() {
		let game = Game {
			board: Vec::new(),
			hand: parse_tiles("b1 b2 b3"),
		};

		let solution = game.solve().expect("Solver should succeed");
		assert_eq!(solution.len(), 1);
		assert_eq!(solution[0], parse_tiles("b1 b2 b3"));
	}

	#[test]
	fn solve_extends_valid_board_when_possible() {
		let game = Game {
			board: parse_tiles("b1 b2 b3"),
			hand: parse_tiles("b4"),
		};

		let solution = game.solve().expect("Solver should succeed");
		assert_eq!(solution.len(), 1);
		assert_eq!(solution[0], parse_tiles("b1 b2 b3 b4"));
	}

	#[test]
	fn solve_completes_color_set_with_hand_tile() {
		let game = Game {
			board: parse_tiles("b7 r7 y7"),
			hand: parse_tiles("l7"),
		};

		let solution = game.solve().expect("Solver should succeed");
		assert_eq!(solution.len(), 1);
		assert_eq!(solution[0], parse_tiles("b7 l7 r7 y7"));
	}

	#[test]
	fn solve_restructures_board_runs_to_place_hand_tile() {
		let game = Game {
			board: parse_tiles("b1 b2 b3 b4 b5 b6 b7"),
			hand: parse_tiles("b4"),
		};

		let solution = game.solve().expect("Solver should succeed");
		assert_eq!(solution.len(), 2);
		assert!(solution.contains(&parse_tiles("b1 b2 b3 b4")));
		assert!(solution.contains(&parse_tiles("b4 b5 b6 b7")));
	}

	#[test]
	fn solve_shifts_tile_between_runs_to_place_hand_tile() {
		let game = Game {
			board: parse_tiles("r3 r4 r5 r6 y3 y4 y5 y6"),
			hand: parse_tiles("b3"),
		};

		let solution = game.solve().expect("Solver should succeed");
		assert_eq!(solution.len(), 3);

		assert!(solution.contains(&parse_tiles("r4 r5 r6")));
		assert!(solution.contains(&parse_tiles("y4 y5 y6")));
		assert!(solution.contains(&parse_tiles("b3 r3 y3")));
	}

	#[test]
	fn solve_only_places_one_of_two_hand_tiles_when_one_is_invalid() {
		let game = Game {
			board: parse_tiles("b1 b2 b3 b6 b7 b8"),
			hand: parse_tiles("b4 b10"),
		};

		let solution = game.solve().expect("Solver should succeed");
		let hand_tiles = parse_tiles("b4 b10");
		let placed_hand_count = solution
			.iter()
			.flat_map(|run| run.iter())
			.filter(|tile| hand_tiles.contains(tile))
			.count();

		assert_eq!(placed_hand_count, 1);
		assert!(solution.iter().any(|run| *run == parse_tiles("b1 b2 b3 b4")));
	}
}
