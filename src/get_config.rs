use std::io;
use std::str;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
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
	/*let mut file = File::open("Makefile")?;
	// Mettre dns une string
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	println!("{}", contents);
	// EOS
	let mut file2 = File::create("Makefile")?;
	file2.write(b"Hello")?;
	Ok(()
	 */
	let paths = fs::read_dir("src/").unwrap();

	for path in paths {
		println!("{}", path.unwrap().path().display())
	}
	Ok(())
}
