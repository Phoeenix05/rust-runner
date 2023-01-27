use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[arg(short, long)]
    pub target: Option<String>,
}
