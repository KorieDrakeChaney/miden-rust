use math::{fields::f64::BaseElement, FieldElement};

pub fn is_binary(n: &BaseElement) -> bool {
    *n == BaseElement::ONE || *n == BaseElement::ZERO
}
