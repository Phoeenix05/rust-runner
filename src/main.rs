use clap::Parser;
use std::process::{Command, Stdio};

mod args;

fn main() {
    let args = args::Args::parse();

    if args.target != None {
        // let target = args.target.unwrap();

        let target_list = Command::new("rustup")
            .args(["target", "list"])
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
