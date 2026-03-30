mod cli;
mod io;
mod language;
mod transform;
mod walker;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    // Show help if no arguments provided
    if args.input.is_none() && args.from.is_none() && args.to.is_none() {
        println!(
            "Usage: codemorph <input_file> [--from <case>] [--to <case>] [--lang <lang>] [--in-place]"
        );
        println!("Example 1 (bulk auto-convert): codemorph file.c");
        println!(
            "Example 2 (single conversion): codemorph file.rs --from snake_case --to camelCase"
        );
        println!("Supported cases: snake_case, camelCase, PascalCase, kebab-case");
        return;
    }

    let input_file = args.input.as_deref();

    let code = io::read_input(&args.input);

    let lang = language::resolve_language(&args.lang, &args.input);

    if let (Some(from_case), Some(to_case)) = (&args.from, &args.to) {
        let converted = walker::process(&code, lang, from_case.clone(), to_case.clone());
        io::write_output(&args.input, &converted, args.in_place);
        println!("Converted {} → {}!", from_case, to_case);
    } else if let Some(file) = input_file {
        let target_cases = [
            "snake_case",
            "camelCase",
            "PascalCase",
            "kebab-case",
            "SCREAMING_SNAKE_CASE",
            "SCREAMING-KEBAB-CASE",
        ];
        for target_case in target_cases.iter() {
            let converted =
                walker::process(&code, lang.clone(), "".to_string(), target_case.to_string());

            use std::path::Path;
            let path = Path::new(file);
            let stem = path.file_stem().unwrap().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            let out_file = format!("{}_{}.{}", stem, target_case, ext);
            std::fs::write(&out_file, converted).expect("Failed to write file");
            println!("Written {}", out_file);
        }
    } else {
        // stdin input without --from/--to → just print usage
        println!("No input file provided. Use: codemorph <input_file>");
    }
}
