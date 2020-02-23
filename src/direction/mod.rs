use crate::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct Direction {
    pub e01: Scalar,
    pub e20: Scalar,
}

impl std::ops::Mul<Scalar> for Direction {
    type Output = Direction;

    fn mul(self, s: Scalar) -> Direction {
        Direction {
            e01: self.e01 * s,
            e20: self.e20 * s,
        }
    }
}

impl std::ops::Mul<Direction> for Scalar {
    type Output = Direction;

    fn mul(self, d: Direction) -> Direction {
        d * self
    }
}

impl std::ops::Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, d: Direction) -> Direction {
        Direction {
            e01: self.e01 + d.e01,
            e20: self.e20 + d.e20,
        }
    }
}
