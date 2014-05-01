pub struct Player {
	pub name: ~str,
	pub rating: f32
}

impl Player {
	pub fn change_to_win(&self, other:&Player) -> f32 {
		let diff = (other.rating - self.rating) / 400.0;
		1.0 / (1.0 + 10f32.powf(diff))
	}

	pub fn battle(&mut self, other:&mut Player, a: f32, b: f32) {
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