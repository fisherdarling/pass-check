pub mod context;
mod function;

use self::context::Context;
use std::{collections::HashMap, fs::ReadDir, path::Path};

use anyhow::anyhow;
use colored::Colorize;
use function::FunctionStats;
use llvm_ir::Module;
use llvm_ir_analysis::CrossModuleAnalysis;
use regex::Regex;
use rustc_demangle::demangle;

pub struct PassCheck {
    modules: Vec<Module>,
}

impl PassCheck {
    pub fn new(modules: Vec<Module>) -> Self {
        Self { modules }
    }

    // pub fn new(path: &Path) -> anyhow::Result<Self> {

    //     Ok(PassCheck { modules })
    // }

    pub fn modules(&self) -> &[Module] {
        &self.modules
    }

    pub fn analysis<'s>(&'s self) -> CrossModuleAnalysis<'s> {
        CrossModuleAnalysis::new(self.modules())
    }

    pub fn demangle(&self, path: &str) -> String {
        format!("{:#}", demangle(path))
    }

    pub fn search_for_func(&self, regex: &Regex, context: &Context<'_>) -> Vec<String> {
        let mut matches = Vec::new();

        for function in context.analysis.functions() {
            let demangled = format!("{:#}", demangle(&function.name));
            if regex.is_match(&demangled) {
                matches.push(demangled);
            }
        }

        matches
    }

    pub fn search_for_module(&self, regex: &Regex, context: &Context<'_>) -> Vec<String> {
        let mut matches = Vec::new();

        for module in context.analysis.modules() {
            if regex.is_match(&module.name) {
                matches.push(module.name.to_string());
            }
        }

        matches
    }

    pub fn analyze_function<'m>(
        &self,
        func_name: &'m str,
        context: &'m mut Context<'m>,
    ) -> anyhow::Result<FunctionStats> {
        context.generate_mangle_map();

        context
            .analyze_function_by_name(func_name)
            .ok_or_else(|| anyhow!("Unable to analyze function: {}", func_name))
    }
}

// pub fn run(directory: ReadDir, entry_point: String) -> anyhow::Result<()> {
//     let modules = read_modules(directory)?;
//     let analysis = CrossModuleAnalysis::new(&modules);

//     let mut mangle_map: HashMap<String, &str> = HashMap::new();

//     for function in analysis.functions() {
//         let demangled = format!("{:#}", demangle(&function.name));
//         mangle_map.insert(demangled, &function.name);
//     }

//     let entry_point = if let Some(name) = mangle_map.get(&entry_point) {
//         println!("{}", "Demangled and found Function:".green().bold());
//         println!(
//             "    {} => {}",
//             entry_point.white().bold(),
//             name.white().bold()
//         );

//         name
//     } else {
//         println!(
//             "{}: {}",
//             "Could not map given function".yellow().bold(),
//             entry_point
//         );
//         entry_point.as_str()
//     };
//     println!();

//     let mut context = Context::new(analysis);
//     let stats = context.analyze_function_by_name(entry_point).unwrap();

//     println!("{:#?}", stats);

//     Ok(())
// }

pub fn read_modules(directory: ReadDir, silent: bool) -> anyhow::Result<Vec<Module>> {
    let mut modules = Vec::new();

    for entry in directory.flatten() {
        if entry.file_type()?.is_file() && is_bitcode(&entry.path()) {
            let path = entry.path();

            if !silent {
                println!(
                    "    {} {} {}",
                    "Adding Module".green().bold(),
                    pretty_crate(&path),
                    &path.display().to_string().dimmed(),
                );
            }

            match Module::from_bc_path(&entry.path()) {
                Ok(m) => modules.push(m),
                Err(e) => eprintln!("Error extracting bc for {:?}: {}", entry.path(), e),
            }
        }
    }

    if !silent {
        println!();
    }

    Ok(modules)
}

fn is_bitcode(path: &Path) -> bool {
    path.extension()
        .map(|str| str.to_str().map(|s| s.ends_with("bc")).unwrap_or_default())
        .unwrap_or_default()
}

fn pretty_crate(path: &Path) -> String {
    let nice = path.file_name().unwrap().to_string_lossy().to_string();
    let name = nice.split('-').next().unwrap().to_string();

    name
}
