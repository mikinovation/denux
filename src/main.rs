mod cli;
mod file_processor;
mod parser;
mod transformer;

use clap::Parser;
use cli::Cli;
use file_processor::{process_ts_file, process_vue_file};
use parser::process_script_setup;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args = Cli::parse();

    WalkDir::new(&args.target)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter(is_target_file)
        .for_each(|entry| process_entry(entry, &args));
}

fn is_target_file(entry: &DirEntry) -> bool {
    matches!(
        entry.path().extension().and_then(|ext| ext.to_str()),
        Some("vue") | Some("ts")
    )
}

fn process_entry(entry: DirEntry, args: &Cli) {
    match entry.path().extension().and_then(|ext| ext.to_str()) {
        Some("vue") => process_vue_file(
            entry.path(),
            args.dry_run,
            args.verbose,
            process_script_setup,
        ),
        Some("ts") => process_ts_file(
            entry.path(),
            args.dry_run,
            args.verbose,
            process_script_setup,
        ),
        _ => (),
    }
}
