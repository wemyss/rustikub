use self::tile::*;
pub mod tile;
pub mod utils;
pub mod tile_runs;

use std::collections::HashMap;
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

	pub fn validate_set() {

	}

	pub fn solve(&self) -> Result<Vec<Vec<Tile>>, String> {
		use good_lp::{default_solver, variable, variables, Expression, IntoAffineExpression, Solution, SolverModel};
		use self::tile_runs::{color, sequential};

		type TileKey = (Color, Option<u8>);

		let mut total_counts: HashMap<TileKey, usize> = HashMap::new();
		let mut hand_counts: HashMap<TileKey, usize> = HashMap::new();
		let mut board_counts: HashMap<TileKey, usize> = HashMap::new();

		let tile_key = |tile: &Tile| -> TileKey {
			(tile.color(), tile.value())
		};

		// Count how many copies of each tile exist in the full pool (board + hand),
		// and separately count board-only and hand-only inventory.
		for tile in self.board.iter().chain(self.hand.iter()) {
			*total_counts.entry(tile_key(tile)).or_default() += 1;
		}

		for tile in self.hand.iter() {
			*hand_counts.entry(tile_key(tile)).or_default() += 1;
		}

		for tile in self.board.iter() {
			*board_counts.entry(tile_key(tile)).or_default() += 1;
		}

		// Generate all candidate runs from the available run generators.
		// Only keep candidates that can be assembled from the available pieces.
		let candidates = sequential::generate_all_sequential_runs()
			.into_iter()
			.chain(color::generate_all_color_runs())
			.collect::<Vec<_>>();

		let mut run_infos = Vec::new();
		for run in candidates {
			let mut counts: HashMap<TileKey, usize> = HashMap::new();
			for tile in run.iter() {
				*counts.entry(tile_key(tile)).or_default() += 1;
			}

			if counts.iter().all(|(key, &count)| total_counts.get(key).copied().unwrap_or(0) >= count) {
				run_infos.push((run, counts));
			}
		}

		// One binary variable per candidate run: selected or not.
		let mut vars = variables!();
		let run_vars: Vec<_> = run_infos.iter().map(|_| vars.add(variable().binary())).collect();

		// One continuous variable per hand tile key representing how many of that
		// tile type are used from the hand in selected runs.
		let hand_vars: HashMap<TileKey, _> = hand_counts
			.iter()
			.map(|(key, &count)| (*key, vars.add(variable().min(0.0).max(count as f64))))
			.collect();

		// Maximise usage of hand tiles.
		let objective = hand_vars.values().fold(Expression::from(0.0), |acc, variable| acc + *variable);

		let mut problem = vars.maximise(objective).using(default_solver);

		for (tile_key, &total) in total_counts.iter() {
			let usage = run_vars
				.iter()
				.zip(run_infos.iter())
				.fold(Expression::from(0.0), |acc, (run_var, (_, counts))| {
					acc + *run_var * (*counts.get(tile_key).unwrap_or(&0) as f64)
				});

			// Do not use more tiles than exist in the pool.
			problem = problem.with(usage.clone().leq(total as f64));

			let board_total = board_counts.get(tile_key).copied().unwrap_or(0);
			if board_total > 0 {
				// Ensure all board tiles remain covered by selected runs.
				problem = problem.with(usage.clone().geq(board_total as f64));
			}

			// Link the hand usage variable to the total usage minus board coverage.
			if let Some(hand_var) = hand_vars.get(tile_key) {
				problem = problem.with(
					(*hand_var).into_expression().eq(usage - Expression::from(board_total as f64))
				);
			}
		}

		// Solve the ILP and return the selected runs.
		let solution = problem.solve().map_err(|e| e.to_string())?;
		let selected_runs = run_vars
			.iter()
			.zip(run_infos.into_iter())
			.filter_map(|(run_var, (run, _counts))| {
				if solution.value(*run_var) > 0.5 {
					Some(run)
				} else {
					None
				}
			})
			.collect();

		Ok(selected_runs)
	}
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
