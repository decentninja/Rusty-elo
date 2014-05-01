/*
	ELO implementation
	by Andreas linn
	https://en.wikipedia.org/wiki/Elo_rating_system
 */

struct Player {
	name: ~str,
	rating: f32
}

impl Player {
	fn change_to_win(&self, other:&Player) -> f32 {
		let diff = (other.rating - self.rating) / 400.0;
		1.0 / (1.0 + 10f32.powf(diff))
	}

	fn battle(&mut self, other:&mut Player, a: f32, b: f32) {
		let tot = a + b;
		let chance = self.change_to_win(other);
		self.update_rating(a - tot * chance);
		other.update_rating(b - tot * (1.0 - chance));
	}

	fn update_rating(&mut self, distance_from_prediction: f32) {
		let k = match self.rating {
			0.0..2100.0 => 32.0,
			2100.0..2400.0 => 24.0,
			_ => 10.0
		};
		self.rating += k * distance_from_prediction;
		let lower = 100.0;
		if self.rating < lower {
			self.rating = lower;
		}
	}
}

#[test]
fn it_predicts_draw() {
	let a = Player{name: ~"A", rating: 1500.0};
	let b = Player{name: ~"B", rating: 1500.0};
	if a.change_to_win(&b) != 0.5 {
		fail!();
	}
}

#[test]
fn law_of_total_probability() {
	let a = Player{name: ~"A", rating: 1500.0};
	let b = Player{name: ~"B", rating: 100.0};
	let a_chance = a.change_to_win(&b);
	let b_chance = b.change_to_win(&a);
	if a_chance + b_chance != 1.0 {
		fail!("{} + {} = {} == 1", a_chance, b_chance, a_chance + b_chance);
	}
}

#[test]
fn draw_between_same_rated_players() {
	let mut a = ~Player{name: ~"A", rating: 1500.0};
	let mut b = ~Player{name: ~"B", rating: 1500.0};
	a.battle(b, 1.0, 1.0);
	if a.rating != b.rating {
		fail!("{} {}", a.rating, b.rating);
	}
}

#[test]
fn draw_between_diffrent_rated_players() {
	let mut a = ~Player{name: ~"A", rating: 1500.0};
	let mut b = ~Player{name: ~"B", rating: 100.0};
	a.battle(b, 1.0, 1.0);
	if !(a.rating < 1500.0 && b.rating > 100.0) {
		fail!();
	}
}