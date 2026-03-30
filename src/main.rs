mod cli;
mod io;
mod language;
mod transform;
mod walker;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    let source = io::read_input(&args.input);
    let lang = language::resolve_language(&args.lang, &args.input);

    let output = walker::process(&source, lang, args.from, args.to);

    io::write_output(&args.input, &output, args.in_place);
}
