#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate toml;
mod get_config;
use std::process::Command;
use std::env;
use get_config::{deploy_dependencies, deploy_pod, update_makefile, update_makefile_src, reset_makefile};
use structopt::StructOpt;
use std::{thread, time};

#[derive(StructOpt, Debug)]
#[structopt(name = "Tycho")]

enum Opt {
	/// Initialize project
#[structopt(name = "init")]
	Init {
		/// Project's name
name: String,
	  /// Initialize with a git repository
#[structopt(short = "g", long  = "git")]
	  git: bool,
	},
	/// Update project
#[structopt(name = "update")]
	Update {
		/// update makefile
#[structopt(short = "m", long  = "makefile")]
	mak: bool,
		/// update src/ in makefile
#[structopt(short = "s", long  = "sources")]
	src: bool,
		/// update pods/ in makefile
#[structopt(short = "i", long  = "includes")]
	inc: bool,
	 /// deploy pod in toml
#[structopt(short = "p", long  = "pod")]
	 pod: bool,
	 /// reset Makefile
#[structopt(short = "r", long  = "reset")]
	 res: bool,
	},

	/// Deploy a pod
#[structopt(name = "deploy")]
	Deploy {
		/// Name of pod
deploy: String,
	},
}

fn main() {
	if env::var("TYCHO_PATH").is_err() {
		println!("TYCHO_PATH unset");
		std::process::exit(1);
	}
	match Opt::from_args() {
		Opt::Init { name, git } => {
			init_project(&name, git);
		}
		Opt::Update { mak, src, inc, pod, res } => {
			let mut ret = false;
			if res == true {
				reset_makefile();
				ret = true;
			}
			if pod == true {
				deploy_dependencies(true);
				ret = true;
			}
			if inc == true {
				deploy_dependencies(false);
				ret = true;
			}
			if src == true {
				let _ = update_makefile_src();
				ret = true;
			}
			if mak == true  || ret == false {
				let _ = update_makefile();
			}
		}
		Opt::Deploy { deploy } => {
			deploy_pod(&deploy, true);
		}
	};
	let ten_millis = time::Duration::from_millis(30);
	thread::sleep(ten_millis);
	let _ = Command::new("sh")
		.arg("-c")
		.arg("rm -rf *.tycho_save")
		.spawn();
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
		let _ = Command::new("cp")
			.arg(format!("{}/template/pod.toml", env::var("TYCHO_PATH").unwrap()))
			.arg(format!("{}/pod.toml", init_name))
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
