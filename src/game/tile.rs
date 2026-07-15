#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Color {
	Black=1,
	Blue=2,
	Red=3,
	Yellow=4,
	Joker=0,
}
pub const ALL_COLORS: [Color; 4] = [Color::Black, Color::Blue, Color::Red, Color::Yellow];

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


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tile {
	pub value: Option<u8>,
	pub color: Color,
}

impl Tile {
	pub fn new(color: char, num: Option<u8>) -> Tile {
		Tile::create(new_color(color).unwrap(), num)
	}

	pub fn create(color: Color, num: Option<u8>) -> Tile {
		let val = match color {
			Color::Joker => None,
			_ => num,
		};

		Tile {
			color: color,
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

	#[test]
	fn tile_display_black_one() {
		assert_eq!("b1", Tile::new('b', Some(1)).to_string());
	}

	#[test]
	fn tile_display_yellow_ten() {
		assert_eq!("y10", Tile::new('y', Some(10)).to_string());
	}

	#[test]
	fn tile_display_joker() {
		assert_eq!("j", Tile::new('j', Some(5)).to_string());
	}
}
