pub const NUM_JOKERS: [u8; 3] = [0, 1, 2];
pub const RUN_LENGTHS: [u8; 3] = [3, 4, 5];
pub const COLOR_RUN_LENGTHS: [u8; 2] = [3, 4];
pub const VALUES: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
pub const MIN_VALUE: u8 = 1;
pub const MAX_VALUE: u8 = 13;


/// Generate all combinations for a given run size and a tile pool to choose from
pub fn generate_combinations<T: Copy>(tile_set: &[T], run_size: u8) -> Vec<Vec<T>> {
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
				},
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_combinations_test_runsize_1() {
		assert_eq!(
			generate_combinations(&[1,2,3,4,5], 1),
			[[1],[2],[3],[4],[5]]
		);
	}

	#[test]
	fn generate_combinations_test_runsize_3() {
		assert_eq!(
			generate_combinations(&[1,2,3,4,5], 3),
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
}
