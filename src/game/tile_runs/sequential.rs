use super::shared::*;
use game::tile::{ALL_COLORS, Color, Tile};
use std::cmp;


fn generate_sequential_runs(color: Color, length: u8, joker_count: u8) -> Vec<Vec<Tile>> {
	let mut runs: Vec<Vec<Tile>> = Vec::new();
	let range_length = (MAX_VALUE - length + joker_count + 1) as usize;

	for i in 1..(range_length+1) {
		let end = cmp::min(
			i + (length as usize) - 1,
			MAX_VALUE as usize
		);

		let combinations = generate_combinations(&VALUES[i..end], length - joker_count - 1);

		for mut c in combinations {
			c.insert(0, i as u8);
			runs.push(
				c.iter()
				 .map(|v| Tile::create(color, Some(*v)))
				 .collect()
			);
		}
	}

	// Add the joker/s in to each run
	for _ in 0..joker_count  {
		for r in &mut runs {
			(*r).push(Tile::new('j', None));
		}
	}

	runs
}

/// Generates all sequential runs for all colors. This includes runs with 0 or more jokers.
pub fn generate_all_sequential_runs() -> Vec<Vec<Tile>> {
	let mut runs = Vec::new();

	for color in ALL_COLORS.iter() {
		for &length in RUN_LENGTHS.iter() {
			for &joker_count in NUM_JOKERS.iter() {
				runs.append(&mut generate_sequential_runs(*color, length, joker_count));
			}
		}
	}

	runs
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn generate_all_sequential_runs_test() {
    // The paper says 901, but 900 is actually correct
		assert_eq!(generate_all_sequential_runs().len(), 900);
	}

	#[test]
	fn generate_sequential_runs_length3_joker2_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 3, 2).len(), 13);
	}

	#[test]
	fn generate_sequential_runs_length4_joker2_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 4, 2).len(), 33);
	}

	#[test]
	fn generate_sequential_runs_length5_joker2_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 5, 2).len(), 58);
	}

	#[test]
	fn generate_sequential_runs_length3_joker1_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 3, 1).len(), 23);
	}

	#[test]
	fn generate_sequential_runs_length4_joker1_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 4, 1).len(), 31);
	}

	#[test]
	fn generate_sequential_runs_length5_joker1_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 5, 1).len(), 37);
	}

	#[test]
	fn generate_sequential_runs_length3_joker0_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 3, 0).len(), 11);
	}

	#[test]
	fn generate_sequential_runs_length4_joker0_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 4, 0).len(), 10);
	}

	#[test]
	fn generate_sequential_runs_length5_joker0_test() {
		assert_eq!(generate_sequential_runs(Color::Black, 5, 0).len(), 9);
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
			generate_sequential_runs(Color::Yellow, 5, 0)
		);
	}
}
