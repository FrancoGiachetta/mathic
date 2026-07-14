//! Symbolic Loop Dominance Lowering Pass.
//!
//! This file holds the implementation of the pass resposible of making the
//! symbolic operations in [MATHIR](crate::lowering::Ir) loop dominant.
//!
//! # What is Loop Dominance?
//!
//!

use std::collections::HashSet;

use crate::lowering::{
    Ir,
    ir::{basic_block::Terminator, function::Function, instruction::LValInstruct},
    passes::MathicPass,
};

pub struct SymbolicLoopDominance;

impl MathicPass for SymbolicLoopDominance {
    fn apply(&self, mut ir: Ir) -> Ir {
        for f in ir.get_functions_mut() {
            for i in 0..f.basic_blocks.len() {
                let (true_block, false_block) = match f.basic_blocks[i].terminator {
                    Terminator::CondBranch {
                        true_block,
                        false_block,
                        ..
                    } => (true_block, false_block),
                    _ => continue,
                };

                let args = self.collect_symbolic_assigns(f, true_block);

                if !args.is_empty() {
                    if let Terminator::CondBranch {
                        ref mut true_successor_args,
                        ref mut false_successor_args,
                        ..
                    } = f.basic_blocks[i].terminator
                    {
                        true_successor_args.extend(&args);
                        false_successor_args.extend(&args);
                    }

                    if let Terminator::Branch {
                        ref mut successor_args,
                        ..
                    } = f.basic_blocks[i - 1].terminator
                    {
                        successor_args.extend(&args);
                    }

                    let true_block_target = match f.basic_blocks[true_block].terminator {
                        Terminator::Branch {
                            target,
                            ref mut successor_args,
                            ..
                        } => {
                            successor_args.extend(&args);
                            target
                        }
                        _ => panic!(),
                    };

                    for arg in args {
                        let local = f.get_local(arg).unwrap().clone();
                        f.basic_blocks[true_block_target].args.push(local.clone());
                        f.basic_blocks[true_block].args.push(local.clone());
                        f.basic_blocks[false_block].args.push(local);
                    }
                }
            }
        }

        ir
    }
}

impl SymbolicLoopDominance {
    /// Returns the index of every local being assigned in a block.
    ///
    /// Uses a [HashSet](HashSet) to avoid duplicates.
    fn collect_symbolic_assigns(&self, ir_func: &Function, target_block: usize) -> HashSet<usize> {
        let mut hash_set = HashSet::new();

        for inst in ir_func.basic_blocks[target_block].instructions.iter() {
            if let LValInstruct::Assign {
                local_idx, value, ..
            } = inst
                && value.ty.is_local
                && ir_func.get_type(value.ty.idx).unwrap().is_symbolic()
            {
                hash_set.insert(*local_idx);
            }
        }

        hash_set
    }
}
