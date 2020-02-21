pub mod direction;
pub mod unitdirection;
pub mod unitpoint;

use super::Scalar;
use direction::Direction;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub e01: Scalar,
    pub e20: Scalar,
    pub e12: Scalar,
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Point {
        Point {
            e01: d.e01,
            e20: d.e20,
            e12: 0f32,
        }
    }
}

impl std::ops::Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, s: Scalar) -> Point {
        Point {
            e01: self.e01 * s,
            e20: self.e20 * s,
            e12: self.e12 * s,
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, d: Direction) -> Point {
        Point {
            e01: self.e01 + d.e01,
            e20: self.e20 + d.e20,
            e12: self.e12,
        }
    }
}

impl Point {
    fn euc_norm(&self) -> Scalar {
        Scalar::abs(self.e12)
    }

    fn ideal_norm(&self) -> Scalar {
        Scalar::sqrt(self.e20.powi(2) + self.e01.powi(2))
    }

    fn is_finite(&self) -> bool {
        self.e01.is_finite() && self.e20.is_finite() && self.e12.is_finite()
    }
}
