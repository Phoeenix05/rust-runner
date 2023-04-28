use clap::Parser;
// use std::process::{Command, Stdio};

mod args;

fn exec_command(cmd: &str, args: &[&str]) {}

fn main() {
    let args = args::Arguments::parse();
    println!("{:?}", args);

    // if args.target.is_some() {
    //     let target_list = Command::new("rustup")
    //         // .args(["target", "list"])
    //         .arg("target")
    //         .arg("list")
    //         .stdout(Stdio::piped())
    //         .spawn()
    //         .expect("Failed to execute 'rustup' command");

    //     let arg = format!("/{}/ && /installed/ {{print}}", args.target.unwrap());
    //     let targets = Command::new("awk")
    //         .arg(arg)
    //         .stdin(target_list.stdout.unwrap())
    //         .output()
    //         .expect("Failed to execute 'awk' command");

    //     print!("{}", String::from_utf8_lossy(&targets.stdout))
    // }
}
