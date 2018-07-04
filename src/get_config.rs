use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use toml;
use std::collections::BTreeMap;

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
		/*	for pod in config.dependencies {
				if let Some(url) = pod.as_str() {
					deploy_pod(url);
					}
			}*/
	unimplemented!();
		}
		Err(e) => println!("Error: {}", e),
	};
}
