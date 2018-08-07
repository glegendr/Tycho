use std::io;
use std::str;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use toml;
use std::collections::BTreeMap;
use std::path::Path;
use std::{thread, time};

#[derive(Deserialize, Debug)]
struct Config {
dependencies: BTreeMap<String, String>,
}

fn read_toml(path: &str) -> io::Result<Config> {
	let mut file = File::open(path)?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;
	let config = match toml::from_slice(&buffer) {
		Ok(v) => v,
			Err(_) => Config {dependencies: BTreeMap::new()},
	};
	println!("{:?}", config);
	Ok(config)
}

fn get_pod_name(url: &str) -> &str {
	url.split("/").last().unwrap().split(".").next().unwrap()
}

pub fn deploy_pod(pod: &str) {
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
	}
}

pub fn deploy_dependencies() {
	match read_toml("pod.toml") {
		Ok(config) => {
			for pod in config.dependencies {
				let url = pod.1;
				deploy_pod(&url);
			}
		}
		Err(e) => println!("Error: {}", e),
	};
}

pub fn reset_makefile() {
	let _ = Command::new("rm")
		.arg(format!("-rf"))
		.arg(format!("makefile"))
		.spawn();
	let _ = Command::new("cp")
		.arg(format!("{}/template/Makefile", env::var("TYCHO_PATH").unwrap()))
		.arg(format!("./Makefile"))
		.spawn();
}

pub fn update_makefile() -> io::Result<()> {
	let ten_millis = time::Duration::from_millis(10);
	thread::sleep(ten_millis);
	let _ = Command::new("sed")
		.arg(format!("-i"))
		.arg(format!("-e"))
		.arg(format!("s/SRCNAME=.*/SRCNAME=/g"))
		.arg(format!("Makefile"))
		.spawn();
	let path = Path::new("src/");
	for entry in path.read_dir().expect("read_dir call failed") {
		if let Ok(entry) = entry {
			let path = entry.path();
			let ext = path.extension();
			if ext == None {
				continue;
				}
			if ext.unwrap() == "c" {
			let file_name = path.file_name().unwrap();
			let str = file_name.to_str().unwrap();
			let (str_end, _) = str.split_at(str.len() - 2);
			thread::sleep(ten_millis);
			let _ = Command::new("sed")
				.arg(format!("-i"))
				.arg(format!("-e"))
				.arg(format!("s/SRCNAME=/SRCNAME= {}/g",  str_end))
				.arg(format!("./Makefile"))
				.spawn();
			}
		}
	}
	thread::sleep(ten_millis);
	let _ = Command::new("rm")
		.arg(format!("-rf"))
		.arg(format!("Makefile-e"))
		.spawn();
	Ok(())
}
