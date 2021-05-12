use std::{fs::ReadDir, path::Path};

use colored::Colorize;
use llvm_ir::Module;

pub fn run(directory: ReadDir) -> Result<(), Box<dyn std::error::Error>> {
    let mut modules = Vec::new();

    for entry in directory.flatten() {
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
