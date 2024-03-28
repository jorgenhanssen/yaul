use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "YAUL")]
#[command(author = "Jørgen Hanssen")]
#[command(version = "0.1")]
#[command(about = "Runtime for the YAUL language", long_about = None)]
pub struct CLI {
    #[arg(long)]
    pub logs: Option<String>,

    #[arg(required = true)]
    pub file: PathBuf,
}
