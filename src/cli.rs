use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "yaul")]
#[command(author = "Jørgen Hanssen")]
#[command(version = "0.1.0")]
#[command(about = "Yet Another Useless Language")]
pub struct CLI {
    #[arg(long, default_value_t = 8192, value_parser = clap::value_parser!(u64).range(1..))]
    pub registers: u64,

    #[arg(long)]
    pub logs: Option<String>,

    #[arg(required = true)]
    pub file: PathBuf,
}
