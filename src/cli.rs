use clap::Parser;

#[derive(Parser)]
#[command(
    name = "codemorph",
    version = "0.1.0",
    about = "Convert variable and function names to different cases"
)]
pub struct Args {
    #[arg(help = "Input file (optional; reads stdin if not provided)")]
    pub input: Option<String>,

    #[arg(short, long, help = "Specify language (optional)")]
    pub lang: Option<String>,

    #[arg(short = 'f', long, help = "Original case style (optional)")]
    pub from: Option<String>,

    #[arg(short = 't', long, help = "Target case style (optional)")]
    pub to: Option<String>,

    #[arg(long, help = "Overwrite original file if set")]
    pub in_place: bool,
}
