extern crate clap;
use clap::{Arg, App, SubCommand};

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
	println!("{:?}", matches);
}
