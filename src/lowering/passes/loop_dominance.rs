use crate::lowering::{
    Ir,
    ir::{basic_block::Terminator, instruction::LValInstruct},
    passes::MathicPass,
};

pub struct LoopDominance;

impl MathicPass for LoopDominance {
    fn apply(&self, mut ir: Ir) -> Ir {
        for f in ir.get_functions_mut() {
            let copy_blocks = f.basic_blocks.clone();

            for bb in f.basic_blocks.iter_mut() {
                if let Terminator::CondBranch {
                    true_block,
                    ref mut true_block_args,
                    ..
                } = bb.terminator
                {
                    for i in &copy_blocks[true_block].instructions {
                        if let LValInstruct::Assign { local_idx, .. } = i {
                            true_block_args.push(*local_idx);
                        }
                    }
                }
            }
        }

        dbg!(ir)
    }
}
