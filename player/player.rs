extern crate collections;

use elo::elo::Elo;
use collections::HashMap;

struct Lobby {
	players: HashMap<&str, ~Elo>
}

impl Lobby {
	fn create(&self, name:&str) {
		self.players.insert(name, Elo{rating: 1500});
	}
}

pub fn new() -> Lobby {
	Lobby {
		players: vec![]
	}
}

