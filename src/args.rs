use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Anton Fredriksson", version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Search(SearchCommand),
    Run(RunCommand),
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub target: String,
}

#[derive(Debug, Args)]
pub struct RunCommand {
    /// build target for [rust]
    #[arg(short, long)]
    pub target: Option<String>,
}
