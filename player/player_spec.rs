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
fn matchmake() {
	let lobby = player::new();
	lobby.create("Mark");
	lobby.create("Daneil");
	let (a, _) = lobby.match();
	if a.name != "Mark" {
		fail!();
	}
}