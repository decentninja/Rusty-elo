mod player;

// Stubbing
mod elo {
	pub mod elo {
		pub struct Elo {
			pub rating: f32,
		}
	}
}


#[test]
fn create_player() {
	let player = player::new(~"Andreas");
}