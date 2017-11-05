
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

#[derive(Debug, PartialEq)]
pub struct Tile {
	value: Option<u8>,
	color: Color,
}

impl Tile {
	pub fn new(color: char, num: Option<u8>) -> Tile {

		let val = match color {
			'j' => None,
			_ => num,
		};

		Tile {
			color: new_color(color).unwrap(),
			value: val,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	// fn new_color
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

	// struct Tile
	#[test]
	fn tile_new_black_one() {
		assert_eq!(Tile { color: Color::Black, value: Some(1) }, Tile::new('b', Some(1)));
	}

	#[test]
	fn tile_new_blue_two() {
		assert_eq!(Tile { color: Color::Blue, value: Some(2) }, Tile::new('l', Some(2)));
	}

	#[test]
	fn tile_new_red_three() {
		assert_eq!(Tile { color: Color::Red, value: Some(3) }, Tile::new('r', Some(3)));
	}

	#[test]
	fn tile_new_yellow_four() {
		assert_eq!(Tile { color: Color::Yellow, value: Some(4) }, Tile::new('y', Some(4)));
	}

	#[test]
	fn tile_new_joker_five() {
		assert_eq!(Tile { color: Color::Joker, value: None }, Tile::new('j', Some(5)));
	}

	#[test]
	fn tile_new_joker_none() {
		assert_eq!(Tile { color: Color::Joker, value: None }, Tile::new('j', None));
	}
}
