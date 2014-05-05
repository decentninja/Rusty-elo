use getopts::*;
use std::os::{args};

pub fn parse_args() -> (~str, Matches) {
	let opts = [
		optflag(
			"l",
			"list",
			"list all active games and players"
		),
		optopt(
			"a",
			"add",
			"add user <name>",
			""
		),
		optopt(
			"c",
			"create",
			"create new database <name>",
			""
		),
		optflag(
			"s",
			"start",
			"start game between <name> <name>",
		),
		optflag(
			"m",
			"match",
			"match players not in games"
		),
		optflag(
			"e",
			"end",
			"ends game between <name> with score <score> <score>",
		),
		optflag(
			"h",
			"help",
			"print this help menu"
		)
	];
	let help = "
ELO implementation
by Andreas linn
https://en.wikipedia.org/wiki/Elo_rating_system

rusty-elo <database> [options]";
	(
		usage(help, opts),
		getopts(args().tail(), opts).unwrap()
	)
}