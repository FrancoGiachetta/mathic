use crate::lowering::{
    Ir,
    ir::{basic_block::Terminator, instruction::LValInstruct},
    passes::MathicPass,
};

pub struct LoopDominance;

impl MathicPass for LoopDominance {
    fn apply(&self, mut ir: Ir) -> Ir {
        for f in ir.get_functions_mut() {
            for i in 0..f.basic_blocks.len() - 1 {
                let true_block = match &f.basic_blocks[i].terminator {
                    Terminator::CondBranch { true_block, .. } => *true_block,
                    _ => continue,
                };

                let args: Vec<usize> = f.basic_blocks[true_block]
                    .instructions
                    .iter()
                    .filter_map(|inst| {
                        if let LValInstruct::Assign {
                            local_idx, value, ..
                        } = inst
                            && value.ty.is_local
                            && (f.get_type(dbg!(value.ty.idx)).unwrap().is_symbolic())
                        {
                            Some(*local_idx)
                        } else {
                            None
                        }
                    })
                    .collect();

                if !args.is_empty()
                    && let Terminator::CondBranch {
                        ref mut true_block_args,
                        ..
                    } = f.basic_blocks[i].terminator
                {
                    true_block_args.extend(args);
                }
            }
        }

        ir
    }
}
