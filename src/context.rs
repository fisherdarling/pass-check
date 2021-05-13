use std::collections::HashMap;

use llvm_ir::{Function, Module};
use llvm_ir_analysis::{CrossModuleAnalysis, FunctionAnalysis};
use rustc_demangle::demangle;

use crate::function::{compute_function_stats, FunctionStats};

pub struct Context<'m> {
    pub(crate) analysis: CrossModuleAnalysis<'m>,
    pub(crate) function_cache: HashMap<&'m str, FunctionStats>,
    pub(crate) mangle_map: HashMap<String, &'m str>,
}

impl<'m> Context<'m> {
    pub fn new(analysis: CrossModuleAnalysis<'m>) -> Context<'m> {
        Context {
            analysis,
            function_cache: HashMap::new(),
            mangle_map: HashMap::new(),
        }
    }

    pub fn generate_mangle_map(&mut self) {
        if self.mangle_map.is_empty() {
            for function in self.analysis.functions() {
                let demangled = format!("{:#}", demangle(&function.name));
                self.mangle_map.insert(demangled, &function.name);
            }
        }
    }

    pub fn get_func_by_name(&self, func_name: &str) -> Option<(&Function, &Module)> {
        self.mangle_map
            .get(func_name)
            .map(|name| self.analysis.get_func_by_name(name))
            .flatten()
    }
}

impl<'m> Context<'m> {
    pub fn analyze_function_by_name(&mut self, func_name: &'m str) -> Option<FunctionStats> {
        if let Some(stats) = self.function_cache.get(func_name) {
            Some(stats.clone())
        } else {
            let (function, _module) = self.get_func_by_name(func_name)?;
            let stats = compute_function_stats(function);

            self.function_cache.insert(func_name, stats.clone());

            Some(stats)
        }
    }

    pub fn analyze_function(&mut self, func: &'m Function) -> Option<FunctionStats> {
        if let Some(stats) = self.function_cache.get(func.name.as_str()) {
            Some(stats.clone())
        } else {
            let stats = compute_function_stats(func);
            self.function_cache
                .insert(func.name.as_str(), stats.clone());

            Some(stats)
        }
    }
}
