use crate::game::tile_runs::color::generate_all_color_runs;
use crate::game::tile_runs::sequential::generate_all_sequential_runs;

use self::tile::*;
pub mod tile;
pub mod tile_runs;
pub mod utils;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct Game {
    board: Vec<Tile>,
    hand: Vec<Tile>,
}

impl Game {
    /// Creates struct representing the game in the file
    pub fn load(filename: &str) -> Game {
        let f = File::open(filename).expect("Failed to open file");

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
        let input_tiles: HashMap<Tile, u8> =
            tiles_to_counts(self.board.iter().chain(self.hand.iter()));

        let all_runs: Vec<Vec<Tile>> = generate_all_sequential_runs()
            .into_iter()
            .chain(generate_all_color_runs())
            .collect();

        // the set of runs that can actually be formed with the input tiles - ignoring duplicate tile usage.
        let valid_runs = filter_runs(&input_tiles, &all_runs);

        let explored: HashMap<SolutionKey, Solution> = HashMap::new();

        let initial_solution = Solution {
            runs: Vec::new(),
            remaining_tiles: input_tiles,
            remaining_runs: valid_runs.clone(),
        };

        let mut solutions: VecDeque<Solution> = valid_runs
            .iter()
            .map(|run| create_next_solution_state(&initial_solution, run))
            .collect();

        while let Some(curr) = solutions.pop_front() {
            // Perfect solution found, don't need to explore further - ignore other permutations of the same solution.
            if curr.remaining_tiles.is_empty() {
                return Ok(curr
                    .runs
                    .into_iter()
                    .map(|r| r.clone())
                    .collect());
            }

            // No runs left, can't explore this solution further.
            if curr.remaining_runs.is_empty() {
                continue;
            }

            for run in &curr.remaining_runs {
                let next_solution = create_next_solution_state(&curr, run);
                if explored.contains_key(&next_solution.key()) {
                    continue;
                }

                solutions.push_back(next_solution);
            }
            return Err("No valid solution found".into());
        }
        return Err("No valid solution found".into());
    }
}

struct Solution<'a> {
    runs: Vec<&'a Vec<Tile>>,
    remaining_tiles: HashMap<Tile, u8>,
    remaining_runs: Vec<&'a Vec<Tile>>,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SolutionKey([u8; 14]);

const BITS_PER_COLOR_COUNTER: u8 = 2;

impl<'a> Solution<'a> {
    // Returns true iff the solution does not result in any tiles remaining that were not already present in the hand.
    // This is only useful to prune leaf node solutions.
    fn is_valid(&self, hand: &[Tile]) -> bool {
        let counts = tiles_to_counts(hand);
        self.remaining_tiles.iter().all(|(tile, &count)| {
            counts
                .get(tile)
                .is_some_and(|&hand_count| count <= hand_count)
        })
    }

    fn score(&self) -> u8 {
        // Note there is at most 106 tiles in a game, so the sum of the lengths of all runs will fit in a u8 no worries.
        self.runs.iter().map(|run| run.len() as u8).sum()
    }

    fn key(&self) -> SolutionKey {
        let mut key = SolutionKey([0; 14]);

        for tile in self.runs.iter().flat_map(|run| run.iter()) {
            match tile.value {
                Some(v) => {
                    key.0[v as usize] += 1 << (BITS_PER_COLOR_COUNTER * tile.color as u8); 
                },
                None => {
                    key.0[0] += 1;
                },
            };
        }
        key
    }
}

/// Creates a new solution state by removing the tiles in `run_to_consume` from the remaining tiles and runs in `previous_solution`.
fn create_next_solution_state<'a>(
    previous_solution: &'a Solution<'a>,
    run_to_consume: &'a Vec<Tile>,
) -> Solution<'a> {
    let mut remaining_tiles = previous_solution.remaining_tiles.clone();
    for tile in run_to_consume {
        match remaining_tiles.get_mut(&tile) {
            Some(x) if *x > 1 => {
                *x -= 1;
            }
            Some(1) => {
                remaining_tiles.remove(&tile);
            }
            _ => panic!("The tile {:?} was not found in the remaining tiles", tile),
        }
    }

    let remaining_runs = filter_runs(&remaining_tiles, &previous_solution.remaining_runs);

    let mut runs = previous_solution.runs.clone();
    runs.push(run_to_consume);

    Solution {
        runs,
        remaining_tiles,
        remaining_runs,
    }
}

/// Filters the given runs to include only those that can be formed from the available tiles.
fn filter_runs<'a, T>(tiles: &HashMap<Tile, u8>, runs: &'a [T]) -> Vec<&'a Vec<Tile>>
where
    T: AsRef<Vec<Tile>>,
{
    runs.iter()
        .map(|r| r.as_ref())
        .filter(|&run| {
            let run_tile_counts = tiles_to_counts(run);

            run_tile_counts
                .iter()
                .all(|(key, &count)| tiles.get(key).is_some_and(|&v| count <= v))
        })
        .collect()
}

/// Returns a map with the number of occurrences of each tile as key value pairs
fn tiles_to_counts<'a>(tiles: impl IntoIterator<Item = &'a Tile>) -> HashMap<Tile, u8> {
    let mut counts = HashMap::new();
    for tile in tiles {
        *counts.entry(*tile).or_default() += 1;
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
            }
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
        assert!(
            solution
                .iter()
                .any(|run| *run == parse_tiles("b1 b2 b3 b4"))
        );
    }
}
