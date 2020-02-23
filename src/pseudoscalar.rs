use crate::Scalar;

pub struct PseudoScalar {
    pub e012: Scalar,
}

impl From<Scalar> for PseudoScalar {
    fn from(s: Scalar) -> PseudoScalar {
        PseudoScalar { e012: s }
    }
}
