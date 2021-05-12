use std::collections::HashMap;

use llvm_ir::Function;
use llvm_ir_analysis::{CrossModuleAnalysis, FunctionAnalysis};

use crate::function::{compute_function_stats, FunctionStats};

pub struct Context<'m> {
    analysis: CrossModuleAnalysis<'m>,
    function_cache: HashMap<&'m str, FunctionStats>,
}

impl<'m> Context<'m> {
    pub fn new(analysis: CrossModuleAnalysis<'m>) -> Context<'m> {
        Context {
            analysis,
            function_cache: HashMap::new(),
        }
    }
}

impl<'m> Context<'m> {
    pub fn analyze_function_by_name(&mut self, func_name: &'m str) -> Option<FunctionStats> {
        if let Some(stats) = self.function_cache.get(func_name) {
            Some(stats.clone())
        } else {
            let (function, _module) = self.analysis.get_func_by_name(func_name)?;
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
