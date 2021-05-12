use std::{fs::DirEntry, path::Path};

use colored::Colorize;
use llvm_ir::Module;
use llvm_ir_analysis::{CrossModuleAnalysis, ModuleAnalysis};
use rustc_demangle::demangle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = std::env::args().nth(1).unwrap();

    let mut modules = Vec::new();

    for entry in std::fs::read_dir(target_path)?.flatten() {
        if entry.file_type()?.is_file() && is_bitcode(&entry.path()) {
            let path = entry.path();
            println!(
                "    {} {} {}",
                "Adding Module".bright_green().bold(),
                pretty_path(&path),
                &path.display().to_string().dimmed(),
            );

            match Module::from_bc_path(&entry.path()) {
                Ok(m) => modules.push(m),
                Err(e) => eprintln!("Error extracting bc for {:?}: {}", entry.path(), e),
            }
        }
    }

    Ok(())
}

fn is_bitcode(path: &Path) -> bool {
    path.extension()
        .map(|str| str.to_str().map(|s| s.ends_with("bc")).unwrap_or_default())
        .unwrap_or_default()
}

fn pretty_path(path: &Path) -> String {
    let nice = path.file_name().unwrap().to_string_lossy().to_string();
    let name = nice.split('-').next().unwrap().to_string();

    name
}
