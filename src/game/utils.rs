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
	let b = range.next().unwrap_or(a);

	(a..(b+1))
}



#[cfg(test)]
mod tests {
	use super::*;
	// TODO: do these tests better
	#[test]
	fn parse_number_range_zero() {
		assert_eq!(0, parse_number_range("0").sum::<u8>());
	}

	#[test]
	fn parse_number_range_seven() {
		assert_eq!(7, parse_number_range("7").sum::<u8>());
	}

	#[test]
	fn parse_number_range_zero_to_zero() {
		assert_eq!(0, parse_number_range("0-0").sum::<u8>());
	}

	#[test]
	fn parse_number_range_seven_to_thirteen() {
		assert_eq!(25, parse_number_range("12-13").sum::<u8>());
	}

	#[test]
	fn parse_number_range_zero_to_eleven() {
		assert_eq!((0..11), parse_number_range("0-11"));
	}

	#[test]
	fn parse_number_range_zero_to_max() {
		assert_eq!((0..255), parse_number_range("0-254"));
	}
}
