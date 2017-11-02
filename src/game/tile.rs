use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Color {
	Black,
	Blue,
	Red,
	Yellow,
	Joker,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    	match s {
    		"b" => Ok(Color::Black),
			"l" => Ok(Color::Blue),
			"r" => Ok(Color::Red),
			"y" => Ok(Color::Yellow),
			"j" => Ok(Color::Joker),
			_ => Err("not a valid value"),
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


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn parse_color_black() {
		assert_eq!(Color::Black, "b".parse::<Color>().unwrap());
    }

	#[test]
    fn parse_color_blue() {
		assert_eq!(Color::Blue, "l".parse::<Color>().unwrap());
    }

	#[test]
    fn parse_color_red() {
		assert_eq!(Color::Red, "r".parse::<Color>().unwrap());
    }

	#[test]
    fn parse_color_yellow() {
		assert_eq!(Color::Yellow, "y".parse::<Color>().unwrap());
    }

	#[test]
    fn parse_color_joker() {
		assert_eq!(Color::Joker, "j".parse::<Color>().unwrap());
    }

	//TODO: fuzz test
}
