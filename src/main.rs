extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process::Command;

fn main() {
	let matches = App::new("Tycho")
		.version("0.1")
		.arg(Arg::with_name("initialize")
				.short("i")
				.long("init")
				.value_name("NAME")
				.takes_value(true)
				.help("init a new C project"))
		.get_matches();
	if let Some(init_name) =  matches.value_of("initialize") {
		init_project(init_name);
	}
}

fn init_project(init_name: &str) {
	let status = Command::new("mkdir")
		.arg(init_name)
		.status()
		.expect("");
	if status.success() {
		let _ = Command::new("mkdir")
			.arg(format!("{}/src", init_name))
			.arg(format!("{}/inc", init_name))
			.spawn();
		let _ = Command::new("touch")
			.arg(format!("{}/Makefile", init_name))
			.arg(format!("{}/src/main.c", init_name))
			.arg(format!("{0}/inc/{0}.h", init_name))
			.spawn();
	} else {
		println!("failed to create {} directory", init_name);
		std::process::exit(1);
	}
}
