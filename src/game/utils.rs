use std::io::{BufRead};

pub fn read_line<R: BufRead>(reader: &mut R) -> String {
	let mut line = String::new();

	reader.read_line(&mut line)
		.expect("Couldn't read line");

	String::from(line.trim())
}



pub fn parse_number_range(range: &str) -> Vec<u8> {
	range
		.split('-')
		.map(|x| x.parse::<u8>().unwrap())
		.collect()
}
