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

pub fn deploy_pod(pod: &str, flag: bool) {
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
			.arg(name)
			.arg(format!("pods/{}/.git", name))
			.spawn();
		if flag {
			let sed = format!("s,wait,{}=\"{}\",g", name, pod);
			let ten_millis = time::Duration::from_millis(10);
			thread::sleep(ten_millis);
			let _ = Command::new("sh")
				.arg("-c")
				.arg("echo wait >> pod.toml")
				.spawn();
			thread::sleep(ten_millis);
			let _ = Command::new("sed")
				.arg("-i.tycho_save")
				.arg("-e")
				.arg(sed)
				.arg("pod.toml")
				.spawn();
		}
	} else {
		println!("failed to deploy your pod");
	}
}

pub fn deploy_dependencies(clone: bool) {
	let _ = Command::new("sed")
		.arg("-i.tycho_save")
		.arg("-e")
		.arg(format!("s/LIBS=.*/LIBS=/g"))
		.arg("Makefile")
		.spawn();
	match read_toml("pod.toml") {
		Ok(config) => {
			for pod in config.dependencies {
				let url = pod.1;
				let name = pod.0;
				while  {
					let sed_status = Command::new("sed")
						.arg("-i.tycho_save")
						.arg("-e")
						.arg(format!("/_LIBS=/! s,LIBS=,LIBS= ./pods/{0}/{0}.a,; s,CC_LIBS=,CC_LIBS= make -C ./pods/{0}/;,g ; s,INC_DIR_LIBS=,INC_DIR_LIBS= -I ./pods/{0}/inc,g", name))
						.arg("Makefile")
						.status()
						.expect("");
					!sed_status.success()
				} {}
				if clone {
					deploy_pod(&url, false);
				}
			}
		}
		Err(e) => println!("Error: {}", e),
	};
}

pub fn reset_makefile() {
	let _ = Command::new("rm")
		.arg("-rf")
		.arg("makefile")
		.spawn();
	let _ = Command::new("cp")
		.arg(format!("{}/template/Makefile", env::var("TYCHO_PATH").unwrap()))
		.arg("./Makefile")
		.spawn();
}

pub fn update_makefile_src() -> io::Result<()> {
	let ten_millis = time::Duration::from_millis(10);
	thread::sleep(ten_millis);
	let _ = Command::new("sed")
		.arg("-i.tycho_save")
		.arg("-e")
		.arg(format!("s/SRCNAME=.*/SRCNAME=/g"))
		.arg("Makefile")
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
					.arg("-i.tycho_save")
					.arg("-e")
					.arg(format!("s/SRCNAME=/SRCNAME= {}/g",  str_end))
					.arg("./Makefile")
					.spawn();
			}
		}
	}
	Ok(())
}

pub fn update_makefile() -> io::Result<()> {
	deploy_dependencies(false);
	update_makefile_src()
}
