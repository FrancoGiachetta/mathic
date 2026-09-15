use crate::lowering::ir::{Ir, function::Function};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SintTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericTy {
    Sint(SintTy),
    Uint(UintTy),
    Float(FloatTy),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MathicType {
    Numeric(NumericTy),
    Adt { index: usize, is_local: bool },
    Bool,
    Char,
    Str,
    SymbolicExpr(NumericTy),
    Void,
}

impl NumericTy {
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Sint(_) | Self::Float(_))
    }

    pub fn bit_width(&self) -> u32 {
        match self {
            Self::Sint(ty) => match ty {
                SintTy::Isize => isize::BITS,
                SintTy::I8 => 8,
                SintTy::I16 => 16,
                SintTy::I32 => 32,
                SintTy::I64 => 64,
                SintTy::I128 => 128,
            },
            Self::Uint(ty) => match ty {
                UintTy::Usize => usize::BITS,
                UintTy::U8 => 8,
                UintTy::U16 => 16,
                UintTy::U32 => 32,
                UintTy::U64 => 64,
                UintTy::U128 => 128,
            },
            Self::Float(ty) => match ty {
                FloatTy::F32 => 32,
                FloatTy::F64 => 64,
            },
        }
    }
}

impl MathicType {
    pub fn bit_width(&self) -> u32 {
        match self {
            Self::Numeric(inner) => inner.bit_width(),
            Self::Bool => 1,
            Self::Char => 8,
            Self::Void => 0,
            Self::Str | Self::SymbolicExpr(_) | Self::Adt { .. } => todo!(),
        }
    }

    /// Returns the align of a type expressed in bits.
    pub fn align(&self, ir: &Ir, func: &Function) -> usize {
        match self {
            Self::Numeric(inner) => inner.bit_width() as usize,
            Self::Bool => 1,
            Self::Str => 8,
            Self::Char => 8,
            Self::SymbolicExpr(_) => 0,
            Self::Void => 0,
            Self::Adt { index, is_local } => {
                let adt_fields_tys: Vec<MathicType> = {
                    if *is_local {
                        let adt = func
                            .get_adt(*index)
                            .expect("internal error: invalid local ADT index in type alignment");
                        adt.get_fields_tys()
                            .iter()
                            .map(|t| {
                                if t.is_local {
                                    func.get_type(t.idx)
                                } else {
                                    ir.get_type(t.idx)
                                }
                                .expect(
                                    "internal error: invalid local type index in type alignment",
                                )
                            })
                            .collect()
                    } else {
                        let adt = ir
                            .get_adt(*index)
                            .expect("internal error: invalid global ADT index in type alignment");
                        adt.get_fields_tys()
                            .iter()
                            .map(|t| {
                                ir.get_type(t.idx).expect(
                                    "internal error: invalid global type index in type alignment",
                                )
                            })
                            .collect()
                    }
                };

                let mut align = 0;

                for ty in adt_fields_tys.iter() {
                    align = align.max(ty.align(ir, func));
                }

                align
            }
        }
    }

    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Self::Numeric(NumericTy::Sint(_) | NumericTy::Float(_))
        )
    }

    #[inline(always)]
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Numeric(NumericTy::Sint(_) | NumericTy::Uint(_)))
    }

    #[inline(always)]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Numeric(NumericTy::Float(_)))
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    #[inline(always)]
    pub fn is_symbolic(&self) -> bool {
        matches!(self, Self::SymbolicExpr(_))
    }
}
