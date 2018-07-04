#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate toml;
mod get_config;
use clap::{Arg, App};
use std::process::Command;
use std::env;
use get_config::{deploy_dependencies, deploy_pod};

fn main() {
	if env::var("TYCHO_PATH").is_err() {
		println!("TYCHO_PATH unset");
		std::process::exit(1);
	}

	let matches = App::new("Tycho")
		.version("0.1")
		.arg(Arg::with_name("initialize")
				.short("i")
				.long("init")
				.value_name("NAME")
				.takes_value(true)
				.help("init a new C project"))
		.arg(Arg::with_name("update")
				.short("u")
				.long("update")
				.takes_value(false)
				.help("update the Makefile"))
		.arg(Arg::with_name("deploy pod")
				.short("d")
				.long("deploy-pod")
				.takes_value(true)
				.value_name("POD_URL")
				.help("deploy a pod in the solution"))
		.arg(Arg::with_name("create vessel")
				.short("g")
				.long("git")
				.takes_value(false)
				.help("create a repo git for your project (it has to be use with -i)"))
		.arg(Arg::with_name("pod.toml")
				.short("p")
				.long("pod")
				.takes_value(false)
				.help("read pod.toml to deploy all your depedencies"))
		.get_matches();
	if let Some(init_name) = matches.value_of("initialize") {
		if matches.is_present("create vessel") {
			init_project(init_name, true);
		} else {
			init_project(init_name, false);
		}
	} else if matches.is_present("update") {
		unimplemented!("it's build time !");
	} else if let Some(pod) =  matches.value_of("deploy pod") {
		deploy_pod(pod);
	} else if matches.is_present("pod.toml") {
		deploy_dependencies();
	} else {
		println!("If you need help use tycho [-h | --help]");
	}
}

fn init_project(init_name: &str, git: bool) {
	let tmp;
	if git == true {
		let status = Command::new("git")
			.arg("init")
			.arg(init_name)
			.status()
			.expect("");
			tmp = status;
	} else {
		let status = Command::new("mkdir")
			.arg(init_name)
			.status()
			.expect("");
			tmp = status;
	}
	if tmp.success() {
		let _ = Command::new("mkdir")
			.arg(format!("{}/src", init_name))
			.arg(format!("{}/inc", init_name))
			.spawn();
		let _ = Command::new("cp")
			.arg(format!("{}/template/Makefile", env::var("TYCHO_PATH").unwrap()))
			.arg(format!("{}/Makefile", init_name))
			.spawn();
		let _ = Command::new("cp")
			.arg(format!("{}/template/main.c", env::var("TYCHO_PATH").unwrap()))
			.arg(format!("{}/src/main.c", init_name))
			.spawn();
		let _ = Command::new("cp")
			.arg(format!("{}/template/inc.h", env::var("TYCHO_PATH").unwrap()))
			.arg(format!("{0}/inc/{0}.h", init_name))
			.spawn();
		if git == true {
			let _ = Command::new("cp")
				.arg(format!("{}/template/.gitignore", env::var("TYCHO_PATH").unwrap()))
				.arg(format!("{}/.gitignore", init_name))
				.spawn();
		}
	} else {
		println!("failed to create {} directory", init_name);
		std::process::exit(1);
	}
}
