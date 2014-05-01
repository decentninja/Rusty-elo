use elo::elo;

pub struct Player {
	name: ~str,
	elo: elo::Elo
}

pub fn new(name: ~str) -> Player {
	Player {
		name: name,
		elo: elo::Elo {
			rating: 1500.0
		}
	}
}