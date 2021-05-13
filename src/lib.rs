pub mod context;
mod function;
pub mod overall;

use crate::overall::ModuleStats;

use self::context::Context;
use std::{
    collections::{HashMap, HashSet},
    fs::ReadDir,
    path::Path,
};

use anyhow::anyhow;
use colored::Colorize;
use function::FunctionStats;
use llvm_ir::Module;
use llvm_ir_analysis::CrossModuleAnalysis;
use overall::EverythingStats;
use regex::Regex;
use rustc_demangle::demangle;

pub struct PassCheck {
    modules: Vec<Module>,
}

impl PassCheck {
    pub fn new(modules: Vec<Module>) -> Self {
        Self { modules }
    }

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

    pub fn analyze_everything<'m>(
        &self,
        context: &'m mut Context<'m>,
    ) -> anyhow::Result<EverythingStats> {
        let mut module_stats = Vec::new();

        for module in context.analysis.modules() {
            let mut stats = ModuleStats::default();
            stats.name = module.name.clone();

            let mut func_stats = HashSet::new();

            for func in &module.functions {
                let func_stat = context.analyze_function(&func);
                func_stats.insert(func_stat);
            }

            stats.functions = func_stats.into_iter().collect();
            stats.functions.sort();

            module_stats.push(stats);
        }

        module_stats.sort();

        Ok(EverythingStats {
            modules: module_stats,
        })
    }

    pub fn analyze_module<'m>(
        &self,
        module_name: &str,
        context: &'m mut Context<'m>,
    ) -> anyhow::Result<ModuleStats> {
        let module = context
            .module_by_pretty_name(&module_name)
            .ok_or_else(|| anyhow!("Unable to find module: {}", module_name))?;

        let mut stats = ModuleStats::default();
        stats.name = module.name.clone();

        let mut func_stats = HashMap::new();

        for func in &module.functions {
            let func_stat = context.analyze_function(&func);
            func_stats.insert(func_stat.name.clone(), func_stat);
        }

        stats.functions = func_stats.values().into_iter().cloned().collect();
        stats.functions.sort();

        Ok(stats)
    }
}

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
