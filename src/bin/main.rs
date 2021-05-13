use std::{borrow::Borrow, path::PathBuf};

use clap::{AppSettings, Clap};
use colored::Colorize;
use pass_check::PassCheck;
use regex::Regex;

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Fisher D. <fdarling@mines.edu>, Jake V. <jvossen@mines.edu>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Target folder that contains the LLVM bitcode (.bc files).
    /// Generally under `target/<profile>/deps`
    #[clap(short, long, parse(from_os_str))]
    target_dir: PathBuf,
    /// Output json (used for comparisons).
    #[clap(short, long)]
    json: bool,
    /// Output to a file.
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Do not print `Adding Module`
    #[clap(short, long)]
    silent: bool,
    #[clap(subcommand)]
    command: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    /// Preform analyses
    Analyze(Analyze),
    /// Search for demangled functions, module names, etc.
    Search(Search),
    /// Compare two different analysis outputs
    Compare(Compare),
}

#[derive(Clap)]
enum Analyze {
    /// Analyze a single function. Expects a full demangled path. See `search` to find functions.
    Func { func_name: String },
    /// Analyze every function in a module. Expects the full module name. See `search` to find modules.
    Module { module_name: String },
    /// Analyze every module and function in the target folder
    Folder,
}

#[derive(Clap)]
enum Search {
    /// Demangle a given path
    Demangle { path: String },
    /// Search for demangled functions with the given regex
    Func {
        /// Function search regex
        regex: Regex,
    },
    /// Search for modules with the given regex
    Module {
        /// Module search regex
        regex: Regex,
    },
}

#[derive(Clap)]
enum Compare {
    Modules {
        original_json: PathBuf,
        new_json: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let dir = std::fs::read_dir(&opts.target_dir)?;
    let modules = pass_check::read_modules(dir, opts.silent)?;

    let pass_check = PassCheck::new(modules);
    let analysis = pass_check.analysis();

    match opts.command {
        Subcommand::Analyze(_) => {}
        Subcommand::Search(s) => match s {
            Search::Demangle { path } => {
                let demangled = pass_check.demangle(&path);

                println!("{}", demangled);
            }
            Search::Func { regex } => {
                let results = pass_check.search_for_func(&regex, &analysis);

                if results.is_empty() {
                    println!("{}", "No functions found.".red().bold());
                } else {
                    for result in results {
                        println!("{}", result);
                    }
                }
            }
            Search::Module { regex } => {
                let results = pass_check.search_for_module(&regex, &analysis);

                if results.is_empty() {
                    println!("{}", "No modules found.".red().bold());
                } else {
                    for result in results {
                        println!("{}", result);
                    }
                }
            }
        },
        Subcommand::Compare(_) => {}
    }

    Ok(())
}
