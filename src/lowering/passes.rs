pub mod loop_dominance;

use super::Ir;

pub trait MathicPass {
    fn apply(&self, ir: Ir) -> Ir;
}
