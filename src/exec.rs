use crate::repos::Repos;
use std::process::Command;

pub fn exec(mut exec_args: Vec<String>, repos: Repos) {
	let args = exec_args.split_off(1);
	let cmd = &exec_args[0]; // only cmd remaining after split_off above
	for repo in &repos.repos {
		repo_exec(&repo.path, &cmd, &args);
	}
}

fn repo_exec(path: &str, cmd: &str, args: &Vec<String>) {
	println!("🏢 {}> {} {}", path, cmd, args.join(" "));
	let output = Command::new(cmd)
		.args(args)
		.current_dir(path)
		.output()
		.expect(&format!("Error running exec {}", cmd));

	let stdout = String::from_utf8(output.stdout).expect("Error converting stdout to string");
	let stderr = String::from_utf8(output.stderr).expect("Error converting stderr to string");
	println!("{}", stdout);
	println!("{}", stderr);
	println!();
}
