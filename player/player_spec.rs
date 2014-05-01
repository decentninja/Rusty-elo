use player::Player;
mod player;

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