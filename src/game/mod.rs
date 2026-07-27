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

        let initial_solution = Solution {
            runs: Vec::new(),
            remaining_tiles: input_tiles,
            remaining_runs: valid_runs.clone(),
        };
        let mut solutions_pool: HashMap<SolutionKey, Solution> = HashMap::new();

        let mut solutions: VecDeque<SolutionKey> = valid_runs
            .iter()
            .map(|run| {
                create_next_solution_state_in_pool(&mut solutions_pool, &initial_solution, run)
            })
            .collect();

        while let Some(curr_key) = solutions.pop_front() {
            let curr = solutions_pool[&curr_key].clone();
            // Perfect solution found, don't need to explore further - ignore other permutations of the same solution.
            if curr.remaining_tiles.is_empty() {
                return Ok(curr.runs.clone());
            }

            // No runs left, can't explore this solution further.
            if curr.remaining_runs.is_empty() {
                continue;
            }

            for run in &curr.remaining_runs {
                let next_solution =
                    create_next_solution_state_in_pool(&mut solutions_pool, &curr, run);
                if solutions_pool.contains_key(&next_solution) {
                    continue;
                }

                solutions.push_back(next_solution);
            }
        }
        solutions
            .iter()
            .max_by(|a, b| {
                let a_score = solutions_pool[a].score();
                let b_score = solutions_pool[b].score();
                a_score.cmp(&b_score)
            })
            .map(|best_key| solutions_pool[best_key].runs.clone())
            .ok_or("No solution found".to_string())
    }
}

fn create_next_solution_state_in_pool(
    solutions_pool: &mut HashMap<SolutionKey, Solution>,
    prev_solution: &Solution,
    run_to_consume: &Vec<Tile>,
) -> SolutionKey {
    let solution = create_next_solution_state(prev_solution, run_to_consume);
    let key = solution.key();
    solutions_pool.insert(key, solution);
    key
}

#[derive(Clone)]
struct Solution {
    runs: Vec<Vec<Tile>>,
    remaining_tiles: HashMap<Tile, u8>,
    remaining_runs: Vec<Vec<Tile>>,
}

/// A key that uniquely identifies a solution state based on the tiles used in the runs. A solution state only includes tiles used in runs on the board, not tiles remaining.
/// - Each index in the array corresponds to a tile value, and the value at that index is a bitfield of u2 integers representing the count of each color used in the solution.
/// - Jokers are represented by the first index and their count is stored as a u8 since they have no color.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SolutionKey([u8; 14]);

const BITS_PER_COLOR_COUNTER: u8 = 2;

impl Solution {
    // Returns true iff the solution does not result in any tiles remaining that were not already present in the hand.
    // This is only useful to prune leaf node solutions (solutions without derivatives).
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
                }
                None => {
                    key.0[0] += 1;
                }
            };
        }
        key
    }
}

/// Creates a new solution state by removing the tiles in `run_to_consume` from the remaining tiles and runs in `previous_solution`.
fn create_next_solution_state(
    previous_solution: &Solution,
    run_to_consume: &Vec<Tile>,
) -> Solution {
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
    runs.push(run_to_consume.clone());

    Solution {
        runs,
        remaining_tiles,
        remaining_runs,
    }
}

/// Filters the given runs to include only those that can be formed from the available tiles.
fn filter_runs(tiles: &HashMap<Tile, u8>, runs: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    runs.iter()
        .filter(|&run| {
            let run_tile_counts = tiles_to_counts(run);

            run_tile_counts
                .iter()
                .all(|(key, &count)| tiles.get(key).is_some_and(|&v| count <= v))
        })
        .map(|run| run.clone())
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
}
