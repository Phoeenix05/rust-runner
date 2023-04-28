use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Anton Fredriksson", version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search targets from rust target list
    Search(SearchCommand),
    
    /// Default run target is system. Specify a target with '--target' flag
    Run(RunCommand),
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Rust build target
    pub target: String,
}

#[derive(Debug, Args)]
pub struct RunCommand {
    /// Rust build target
    #[arg(short, long)]
    pub target: Option<String>,
}
