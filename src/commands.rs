use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required = true)]
    pub token: String,
    #[arg(short, long, required = true)]
    pub path: String,
    #[arg(short, long, default_value = "en")]
    pub locale: String,
    #[arg(short, long, required = true)]
    pub export_path: String,
    #[arg(short, long, num_args = 1..)]
    pub filters: Vec<String>,
    #[arg(long)]
    pub post: Option<bool>,
    #[arg(long)]
    pub get: Option<bool>,
    #[arg(short, long)]
    pub source: Option<String>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Import,
    Export,
    Update,
}
