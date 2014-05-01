/*
	ELO implementation
	by Andreas linn
	https://en.wikipedia.org/wiki/Elo_rating_system
 */
#![feature(globs)]

extern crate getopts;
use getopts::*;
use std::os::{args};


//mod elo;
//mod player;

fn main() {
	let (help, ops) = parse_args();
	let filename: &str = if !ops.free.is_empty() && !ops.opt_present("h") {
        (*ops.free.get(0)).clone()
    } else {
        println!("{}", help);
        return;
    };
    // TODO load players
    println!("Loading {}", filename);
    handle_create(&ops);
    handle_end(&ops);
    handle_add(&ops);
    handle_start(&ops);
    handle_matchmake(&ops);
    handle_list(&ops);
}

fn handle_create(ops: &Matches) {
	if ops.opt_present("c") {
		let dbname = ops.opt_str("c");
		// TODO Create new db
		println!("Create db {}", dbname);
	}
}

fn handle_end(ops: &Matches) {
	if ops.opt_present("e") {
    	let v = ops.free.clone();
    	if v.len() != 4 {
    		return;
    	}
    	let (name, ascore, bscore) = (v.get(1), v.get(2), v.get(3));
    	// defu no pattern matching on vec?
    	// TODO end game
    	println!("End game {} {} {}", name, ascore, bscore);
    }
}

fn handle_add(ops: &Matches) {
	if ops.opt_present("a") {
    	let name = ops.opt_str("a");
    	// TODO add
    	println!("Add {}", name);
    }
}

fn handle_start(ops: &Matches) {
	if ops.opt_present("s") {
    	let v = ops.free.clone();
    	if v.len() != 3 {
    		return;
    	}    	
    	let (aname, bname) = (v.get(1), v.get(2));
    	// TODO start
    	println!("Match between {} and {}", aname, bname);
    }
}

fn handle_matchmake(ops: &Matches) {
    if ops.opt_present("m") {
    	// TODO matchmake
    	println!("Matchmake!");
    }
}

fn handle_list(ops: &Matches) {
    if ops.opt_present("l") {
    	// TODO list
    	println!("List");
    }
}

fn parse_args() -> (~str, Matches) {
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