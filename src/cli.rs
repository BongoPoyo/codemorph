use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg()]
    pub input: Option<String>, // positional file

    #[arg(short, long)]
    pub lang: Option<String>, // now optional

    #[arg(short = 'f', long)]
    pub from: String,

    #[arg(short = 't', long)]
    pub to: String,

    #[arg(long)]
    pub in_place: bool,
}
