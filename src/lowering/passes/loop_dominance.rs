use crate::lowering::{
    Ir,
    ir::{basic_block::Terminator, function::Function, instruction::LValInstruct},
    passes::MathicPass,
};

pub struct LoopDominance;

impl MathicPass for LoopDominance {
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
                        target,
                        ref mut successor_args,
                        ..
                    } = f.basic_blocks[true_block].terminator
                    {
                        successor_args.extend(&args);
                    }

                    if let Terminator::Branch {
                        target,
                        ref mut successor_args,
                        ..
                    } = f.basic_blocks[false_block].terminator
                    {
                        successor_args.extend(&args);
                    }

                    for arg in args {
                        let local = f.get_local(arg).unwrap().clone();
                        f.basic_blocks[i].args.push(local);
                    }
                }
            }
        }

        ir
    }
}

impl LoopDominance {
    fn collect_symbolic_assigns(&self, ir_func: &Function, target_block: usize) -> Vec<usize> {
        ir_func.basic_blocks[target_block]
            .instructions
            .iter()
            .filter_map(|inst| {
                if let LValInstruct::Assign {
                    local_idx, value, ..
                } = inst
                    && value.ty.is_local
                    && ir_func.get_type(value.ty.idx).unwrap().is_symbolic()
                {
                    Some(*local_idx)
                } else {
                    None
                }
            })
            .collect()
    }
}
