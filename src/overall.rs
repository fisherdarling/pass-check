use crate::function::FunctionStats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EverythingStats {
    pub modules: Vec<ModuleStats>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModuleStats {
    pub(crate) name: String,
    pub(crate) functions: Vec<FunctionStats>,
}
