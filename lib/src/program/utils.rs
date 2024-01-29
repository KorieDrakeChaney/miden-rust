use math::{fields::f64::BaseElement, FieldElement};

use crate::Operand;

pub fn is_binary(n: &BaseElement) -> bool {
    *n == BaseElement::ONE || *n == BaseElement::ZERO
}

pub fn is_arithmetic_operand(op: &Operand) -> bool {
    match op {
        Operand::Add
        | Operand::AddImm(_)
        | Operand::Sub
        | Operand::SubImm(_)
        | Operand::Mul
        | Operand::MulImm(_)
        | Operand::Div
        | Operand::DivImm(_)
        | Operand::Neg
        | Operand::Inv
        | Operand::Pow2
        | Operand::Exp
        | Operand::ExpBitLength(_)
        | Operand::ExpImm(_) => true,
        _ => false,
    }
}

pub fn is_boolean_operand(op: &Operand) -> bool {
    match op {
        Operand::And | Operand::Or | Operand::Xor | Operand::Not => true,
        _ => false,
    }
}

pub fn is_comparison_operand(op: &Operand) -> bool {
    match op {
        Operand::Eq
        | Operand::EqImm(_)
        | Operand::EqW
        | Operand::Neq
        | Operand::NeqImm(_)
        | Operand::Lt
        | Operand::Lte
        | Operand::Gt
        | Operand::IsOdd
        | Operand::Gte => true,
        _ => false,
    }
}

pub fn is_extension_field_operand(op: &Operand) -> bool {
    match op {
        Operand::Ext2Add
        | Operand::Ext2Sub
        | Operand::Ext2Mul
        | Operand::Ext2Neg
        | Operand::Ext2Inv
        | Operand::Ext2Div => true,
        _ => false,
    }
}

pub fn is_advice_operand(op: &Operand) -> bool {
    match op {
        Operand::AdvPush(_) | Operand::AdvLoadW | Operand::AdvPipe => true,
        _ => false,
    }
}

pub fn is_memory_operand(op: &Operand) -> bool {
    match op {
        Operand::MemLoad
        | Operand::MemStore
        | Operand::MemLoadW
        | Operand::MemStoreW
        | Operand::LocLoad(_)
        | Operand::LocStore(_)
        | Operand::LocLoadW(_)
        | Operand::LocStoreW(_) => true,
        _ => false,
    }
}

pub fn is_manipulation_operand(op: &Operand) -> bool {
    match op {
        Operand::Drop
        | Operand::DropW
        | Operand::Push(_)
        | Operand::PadW
        | Operand::Dup(_)
        | Operand::DupW(_)
        | Operand::Swap(_)
        | Operand::SwapW(_)
        | Operand::SwapDw(_)
        | Operand::MovUp(_)
        | Operand::MovUpW(_)
        | Operand::MovDn(_)
        | Operand::MovDnW(_) => true,
        _ => false,
    }
}

pub fn is_conversion_operand(op: &Operand) -> bool {
    todo!()
}

pub fn is_u32_arithmetic_operand(op: &Operand) -> bool {
    match op {
        Operand::U32CheckedAdd
        | Operand::U32CheckedAddImm(_)
        | Operand::U32OverflowingAdd
        | Operand::U32OverflowingAddImm(_)
        | Operand::U32WrappingAdd
        | Operand::U32WrappingAddImm(_)
        | Operand::U32OverflowingAdd3
        | Operand::U32WrappingAdd3
        | Operand::U32CheckedSub
        | Operand::U32CheckedSubImm(_)
        | Operand::U32OverflowingSub
        | Operand::U32OverflowingSubImm(_)
        | Operand::U32WrappingSub
        | Operand::U32WrappingSubImm(_)
        | Operand::U32CheckedMul
        | Operand::U32CheckedMulImm(_)
        | Operand::U32OverflowingMul
        | Operand::U32OverflowingMulImm(_)
        | Operand::U32WrappingMul
        | Operand::U32WrappingMulImm(_)
        | Operand::U32OverflowingMadd
        | Operand::U32WrappingMadd
        | Operand::U32CheckedDiv
        | Operand::U32CheckedDivImm(_)
        | Operand::U32UncheckedDiv
        | Operand::U32UncheckedDivImm(_)
        | Operand::U32CheckedMod
        | Operand::U32CheckedModImm(_)
        | Operand::U32UncheckedMod
        | Operand::U32UncheckedModImm(_)
        | Operand::U32CheckedDivMod
        | Operand::U32CheckedDivModImm(_)
        | Operand::U32UncheckedDivMod
        | Operand::U32UncheckedDivModImm(_) => true,

        _ => false,
    }
}

pub fn is_u32_bitwise_operand(op: &Operand) -> bool {
    match op {
        Operand::U32CheckedAnd
        | Operand::U32CheckedOr
        | Operand::U32CheckedXor
        | Operand::U32CheckedNot
        | Operand::U32CheckedShl
        | Operand::U32CheckedShlImm(_)
        | Operand::U32CheckedShr
        | Operand::U32CheckedShrImm(_)
        | Operand::U32CheckedRotl
        | Operand::U32CheckedRotlImm(_)
        | Operand::U32CheckedRotr
        | Operand::U32CheckedRotrImm(_)
        | Operand::U32UncheckedShl
        | Operand::U32UncheckedShlImm(_)
        | Operand::U32UncheckedShr
        | Operand::U32UncheckedShrImm(_)
        | Operand::U32UncheckedRotl
        | Operand::U32UncheckedRotlImm(_)
        | Operand::U32UncheckedRotr
        | Operand::U32UncheckedRotrImm(_)
        | Operand::U32CheckedPopcnt
        | Operand::U32UncheckedPopcnt => true,
        _ => false,
    }
}

pub fn is_u32_comparison_operand(op: &Operand) -> bool {
    match op {
        Operand::U32CheckedEq
        | Operand::U32CheckedEqImm(_)
        | Operand::U32CheckedNeq
        | Operand::U32CheckedNeqImm(_)
        | Operand::U32CheckedLt
        | Operand::U32UncheckedLt
        | Operand::U32CheckedLte
        | Operand::U32UncheckedLte
        | Operand::U32CheckedGt
        | Operand::U32UncheckedGt
        | Operand::U32CheckedGte
        | Operand::U32UncheckedGte
        | Operand::U32CheckedMin
        | Operand::U32UncheckedMin
        | Operand::U32CheckedMax
        | Operand::U32UncheckedMax => true,

        _ => false,
    }
}
