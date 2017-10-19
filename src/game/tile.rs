pub enum Color {
	Black,
	Blue,
	Red,
	Yellow,
}

pub struct Tile {
	value: u8,
	color: Color,
}

impl Tile {
	fn new(tile: &str) -> Tile {
		let v: Vec<&str> = tile.splitn(2, |c: char| c.is_digit(10)).collect();
		Tile {
			value: 0,
			color: Color::Black,
		}
	}
}
