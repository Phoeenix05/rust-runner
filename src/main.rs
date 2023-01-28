use clap::Parser;
use std::process::{Command, Stdio};

mod args;

/// if the user has specified a target, then print the list of installed targets that match the user's
/// target
fn main() {
    let args = args::Args::parse();

    if args.target.is_some() {
        // let target = args.target.unwrap();

        let target_list = Command::new("rustup")
            // .args(["target", "list"])
            .arg("target")
            .arg("list")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute 'rustup' command");

        let arg = format!("/{}/ && /installed/ {{print}}", args.target.unwrap());
        let targets = Command::new("awk")
            .arg(arg)
            .stdin(target_list.stdout.unwrap())
            .output()
            .expect("Failed to execute 'awk' command");

        print!("{}", String::from_utf8_lossy(&targets.stdout))

        // let installed = Command::new("grep")
        //     .arg("installed")
        //     .stdin(targets.stdout.unwrap())
        //     .output()
        //     .expect("Failed to execute 'grep' command");

        // println!("{}", String::from_utf8_lossy(&installed.stdout))
    }
}
