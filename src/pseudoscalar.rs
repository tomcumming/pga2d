use std::ops::Mul;

use crate::Scalar;

/// Grade-3 blade, also known as I
pub struct PseudoScalar {
    pub e012: Scalar,
}

impl Mul<Scalar> for PseudoScalar {
    type Output = PseudoScalar;

    fn mul(self, s: Scalar) -> PseudoScalar {
        PseudoScalar::from(self.e012 * s)
    }
}

impl Mul<PseudoScalar> for Scalar {
    type Output = PseudoScalar;

    fn mul(self, ps: PseudoScalar) -> PseudoScalar {
        ps * self
    }
}

impl From<Scalar> for PseudoScalar {
    fn from(s: Scalar) -> PseudoScalar {
        PseudoScalar { e012: s }
    }
}

impl From<PseudoScalar> for Scalar {
    fn from(ps: PseudoScalar) -> Scalar {
        ps.e012
    }
}

impl PseudoScalar {
    pub fn ideal_norm(self) -> Scalar {
        self.e012
    }
}
