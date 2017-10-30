use std::num::ParseIntError;
use std::str::FromStr;

pub enum Color {
	Black,
	Blue,
	Red,
	Yellow,
	Joker,
}

impl FromStr for Color {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    	match s {
    		"b" => Ok(Color::Black),
			"l" => Ok(Color::Blue),
			"r" => Ok(Color::Red),
			"y" => Ok(Color::Yellow),
			"j" => Ok(Color::Joker),
			_ => Err(()),
		}
    }
}

pub struct Tile {
	value: u8,
	color: Color,
}

impl Tile {
	pub fn new(tile: &str) -> Tile {
		// Split string into 2. e.g. B11 -> B, 11
		let v: Vec<&str> = tile.splitn(2, |c: char| c.is_digit(10)).collect();

		Tile {
			value: v[1].parse().unwrap(),
			color: v[0].parse::<Color>().unwrap(),
		}
	}
}
