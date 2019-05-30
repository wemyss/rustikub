use super::shared::*;
use game::tile::{ALL_COLORS, Color, Tile};

/// Generates the color combinations for a given set size and joker count to use
fn generate_color_runs(length: u8, joker_count: u8) -> Vec<Vec<Color>> {
	let mut runs = Vec::new();

	if length == 3 && joker_count == 2 {
		// NOTE: groups with 3 different colors and 2 jokers are already counted as a sequential run.
		return runs;
	}

	for mut combination in generate_combinations(&ALL_COLORS, length - joker_count) {
		for _ in 0..joker_count {
			combination.push(Color::Joker);
		}

		runs.push(combination);
	}

	runs
}

/// Generates all the color combinations possible in rummikub
pub fn generate_all_color_runs() -> Vec<Vec<Tile>> {
	let mut runs = Vec::new();

	for joker_count in NUM_JOKERS.iter() {
		for run_length in COLOR_RUN_LENGTHS.iter() {
			for color_set in generate_color_runs(*run_length, *joker_count) {
				for val in VALUES.iter() {
					runs.push(
						color_set
							.iter()
							.map(|c| Tile::create(*c, Some(*val)))
							.collect()
					);
				}
			}
		}
	}

	runs
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_all_color_runs_test() {
		// Same number/three different colors		|		130 +
		// Same number/four different colors		|		143 =
		assert_eq!(generate_all_color_runs().len(), 273);
	}

	#[test]
	fn generate_color_runs_length3_joker2_test() {
		// already included in sequential runs
		assert_eq!(generate_color_runs(3,2).len(), 0);
	}

	#[test]
	fn generate_color_runs_length4_joker2_test() {
		assert_eq!(generate_color_runs(4,2).len(), 6);
	}

	#[test]
	fn generate_color_runs_length3_joker1_test() {
		assert_eq!(generate_color_runs(3,1).len(), 6);
	}

	#[test]
	fn generate_color_runs_length4_joker1_test() {
		assert_eq!(generate_color_runs(4,1).len(), 4);
	}

	#[test]
	fn generate_color_runs_length3_joker0_test() {
		assert_eq!(generate_color_runs(3,0).len(), 4);
	}

	#[test]
	fn generate_color_runs_length4_joker0_test() {
		assert_eq!(generate_color_runs(4,0).len(), 1);
	}
}
