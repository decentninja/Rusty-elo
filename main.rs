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
	let ops = parse_args();
	if ops.opt_present("h") {
		print_usage();
		return
	}
	let filename: &str = if !ops.free.is_empty() {
        (*ops.free.get(0)).clone()
    } else {
        print_usage();
        return;
    };
    // TODO load players
    println!("Loading {}", filename);
    if ops.opt_present("e") {
    	let v = ops.free.clone();
    	if v.len() != 4 {
    		print_usage();
    		return;
    	}
    	let (name, ascore, bscore) = (v.get(1), v.get(2), v.get(3));
    	// defu no pattern matching on vec?
    	// TODO end game
    	println!("End game {} {} {}", name, ascore, bscore);
    }
	if ops.opt_present("a") {
    	let name = ops.opt_str("a");
    	// TODO add
    	println!("Add {}", name);
    }
    if ops.opt_present("s") {
    	let v = ops.free.clone();
    	if v.len() != 3 {
    		print_usage();
    		return;
    	}    	
    	let (aname, bname) = (v.get(1), v.get(2));
    	// TODO start
    	println!("Match between {} and {}", aname, bname);
    }
    if ops.opt_present("m") {
    	// TODO matchmake
    	println!("Matchmake!");
    }
    if ops.opt_present("l") {
    	// TODO list
    	println!("List");
    }
}

fn print_usage() {
	println!("rusty-elo <database> --list | --add <name> | ..."); // TODO
}

fn parse_args() -> Matches {
	getopts(args().tail(), [
		optflag(
			"l",
			"list",
			"list all active games and players"
		),
		optopt(
			"a",
			"add",
			"add user <name>",
			"NAME"
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
	]).unwrap()
}