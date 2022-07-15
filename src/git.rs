use std::process::Command;
use std::env;

pub fn get_local_branches() -> Vec<String> {
  let path = env::current_dir().unwrap();
  let output = Command::new("git")
    .arg("branch")
    .current_dir(path)
    .output()
    .expect("failed to execute process");

  let raw_str = std::str::from_utf8(&output.stdout).unwrap();
  println!("{}", raw_str);
  // let raw_str = raw_str.replace("\n", "");
  let list = raw_str.split("\n")
    .filter(|&x| !x.is_empty())
    .filter(|&x| !x.eq("*"))
    .map(|x| format!("{}", x.trim_matches('*').trim()))
    .collect::<Vec<String>>();
  list
}

pub fn delete_branch(name: &str) -> std::process::Output {
  let path = env::current_dir().unwrap();
  let output = Command::new("git")
    .args(["branch", "-D", name])
    .current_dir(path)
    .output()
    .expect("failed to delete branch");
  output
}
