mod cli;
mod file_processor;
mod parser;
mod transformer;

use clap::Parser;
use cli::Cli;
use file_processor::{process_ts_file, process_vue_file};
use parser::process_script_setup;
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let args = Cli::parse();

    WalkDir::new(args.target)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "vue" || ext == "ts")
                .unwrap_or(false)
        })
        .for_each(|entry| {
            if entry
                .path()
                .extension()
                .map(|ext| ext == "vue")
                .unwrap_or(false)
            {
                process_vue_file(
                    entry.path(),
                    args.dry_run,
                    args.verbose,
                    process_script_setup,
                );
            } else {
                process_ts_file(
                    entry.path(),
                    args.dry_run,
                    args.verbose,
                    process_script_setup,
                );
            }
        });
}
