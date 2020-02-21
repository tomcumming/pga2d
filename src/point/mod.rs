pub mod direction;
pub mod unit;
pub mod unitdirection;

use super::line::Line;
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

impl super::Meet<Point> for Point {
    type Output = Direction;

    fn meet(self, rhs: Point) -> Direction {
        Direction {
            e01: self.e20 * rhs.e12 + -self.e12 * rhs.e20,
            e20: -self.e01 * rhs.e20 + self.e12 * rhs.e01,
        }
    }
}

impl super::Inner<Point> for Point {
    type Output = Scalar;

    fn inner(self, rhs: Point) -> Scalar {
        -self.e12 * rhs.e12
    }
}

impl super::Dual for Point {
    type Output = Line;

    fn dual(self) -> Line {
        Line {
            e0: self.e12,
            e1: self.e20,
            e2: self.e01,
        }
    }
}

impl super::Join<Point> for Point {
    type Output = Line;

    fn join(self, rhs: Point) -> Line {
        use crate::{Dual, Meet};

        self.dual().meet(rhs.dual()).dual()
    }
}

impl Point {
    fn euc_norm(&self) -> Scalar {
        self.e12
    }

    fn ideal_norm(&self) -> Scalar {
        Scalar::sqrt(self.e20.powi(2) + self.e01.powi(2))
    }

    fn is_finite(&self) -> bool {
        self.e01.is_finite() && self.e20.is_finite() && self.e12.is_finite()
    }
}
