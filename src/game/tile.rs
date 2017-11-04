
#[derive(Debug, PartialEq)]
pub enum Color {
	Black,
	Blue,
	Red,
	Yellow,
	Joker,
}


pub fn new_color(s: char) -> Result<Color, &'static str>{
	match s {
		'b' => Ok(Color::Black),
		'l' => Ok(Color::Blue),
		'r' => Ok(Color::Red),
		'y' => Ok(Color::Yellow),
		'j' => Ok(Color::Joker),
		_ => Err("not a valid value"),
	}
}

#[derive(Debug)]
pub struct Tile {
	value: u8,
	color: Color,
}

impl Tile {
	pub fn new(color: char, num: u8) -> Tile {
		Tile {
			value: num,
			color: new_color(color).unwrap(),
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_color_black() {
		assert_eq!(Color::Black, new_color('b').unwrap());
	}

	#[test]
	fn new_color_blue() {
		assert_eq!(Color::Blue, new_color('l').unwrap());
	}

	#[test]
	fn new_color_red() {
		assert_eq!(Color::Red, new_color('r').unwrap());
	}

	#[test]
	fn new_color_yellow() {
		assert_eq!(Color::Yellow, new_color('y').unwrap());
	}

	#[test]
	fn new_color_joker() {
		assert_eq!(Color::Joker, new_color('j').unwrap());
	}

	//TODO: fuzz test
}
