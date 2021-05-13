use llvm_ir::{Function, Instruction, Name};
use llvm_ir_analysis::{CFGNode, ControlFlowGraph, FunctionAnalysis};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionStats {
    pub(crate) name: String,
    pub(crate) instrs: InstructionStats,
    pub(crate) cfg: CFGStats,
}

pub fn compute_function_stats(func: &Function) -> FunctionStats {
    let analysis = FunctionAnalysis::new(func);

    let instr_stats = compute_instruction_stats(func);
    let dep_stats = compute_cfg_stats(&analysis.control_flow_graph());

    FunctionStats {
        name: func.name.clone(),
        instrs: instr_stats,
        cfg: dep_stats,
    }
}

#[derive(
    Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct InstructionStats {
    loads: usize,
    stores: usize,
    allocas: usize,
    calls: usize,
    atomic_ops: usize,
    instrs: usize,
}

fn compute_instruction_stats(func: &Function) -> InstructionStats {
    let mut stats = InstructionStats::default();

    for block in &func.basic_blocks {
        for instr in &block.instrs {
            match instr {
                Instruction::Load(_) => stats.loads += 1,
                Instruction::Store(_) => stats.stores += 1,
                Instruction::Alloca(_) => stats.allocas += 1,
                Instruction::Call(_) => stats.calls += 1,
                Instruction::AtomicRMW(_) => stats.atomic_ops += 1,
                Instruction::CmpXchg(_) => stats.atomic_ops += 1,
                _ => {}
            }
            stats.instrs += 1;
        }
        stats.instrs += 1;
    }

    stats
}

#[derive(
    Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct CFGStats {
    blocks: usize,
    depth: usize,
    branches: usize,
}

fn compute_cfg_stats<'m>(graph: &ControlFlowGraph<'m>) -> CFGStats {
    let mut stats = CFGStats::default();

    let mut seen: HashSet<&Name> = HashSet::new();
    let mut queue: VecDeque<&Name> = VecDeque::new();
    queue.push_back(graph.entry());
    seen.insert(graph.entry());

    // BFS
    while let Some(block) = queue.pop_front() {
        // Add children to the queue
        for child in graph.succs(block) {
            stats.branches += 1;
            match child {
                CFGNode::Block(b) => {
                    if !seen.contains(b) {
                        queue.push_back(b);
                        seen.insert(b);
                    }
                }
                CFGNode::Return => {}
            }
        }

        stats.blocks += 1;
    }

    // DFS to calculate depth:
    stats.depth = compute_depth(graph, CFGNode::Block(graph.entry()), &mut HashSet::new());

    stats
}

fn compute_depth<'m>(
    graph: &ControlFlowGraph<'m>,
    node: CFGNode<'m>,
    seen: &mut HashSet<&'m Name>,
) -> usize {
    let block = match node {
        CFGNode::Block(b) => b,
        CFGNode::Return => return 1,
    };

    if seen.contains(block) {
        return 0;
    }

    seen.insert(block);

    let max = graph
        .succs(block)
        .map(|n| compute_depth(graph, n, seen))
        .max()
        .unwrap_or_default();

    max + 1
}
