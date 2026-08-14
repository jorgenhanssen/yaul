use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "yaul")]
#[command(author = "Jørgen Hanssen")]
#[command(version = "0.1.0")]
#[command(about = "Yet Another Useless Language")]
pub struct CLI {
    #[arg(long)]
    pub logs: Option<String>,

    #[arg(required = true)]
    pub file: PathBuf,
}
