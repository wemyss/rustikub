use std::io::{BufRead};
use std::ops::Range;

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



#[cfg(test)]
mod tests {
	use super::*;
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
