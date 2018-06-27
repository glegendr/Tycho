extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process::Command;
use std::env;
use std::path::Path;
use std::fs::ReadDir;

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
		.get_matches();
	if let Some(init_name) =  matches.value_of("initialize") {
		init_project(init_name);
	} else if matches.is_present("update") {
		unimplemented!("it's build time !");
	} else if let Some(pod) =  matches.value_of("deploy pod") {
		deploy_pod(pod);
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
	} else {
		println!("failed to create {} directory", init_name);
		std::process::exit(1);
	}
}

fn get_pod_name(url: &str) -> &str {
	url.split("/").last().unwrap().split(".").next().unwrap()
}

fn deploy_pod(pod: &str) {
	let _ = Command::new("mkdir")
		.arg("-p")
		.arg("pods")
		.status();
	let gcl_status = Command::new("git")
		.arg("clone")
		.arg(pod)
		.status()
		.expect("");
	if gcl_status.success() {
		let name = get_pod_name(pod);
		let _ = Command::new("mv")
			.arg(name)
			.arg("pods/")
			.spawn();
		let _ = Command::new("rm")
			.arg("-rf")
			.arg(format!("pods/{}/.git", name))
			.spawn();
	} else {
		println!("failed to deploy your pod");
		std::process::exit(1);
	}
}
