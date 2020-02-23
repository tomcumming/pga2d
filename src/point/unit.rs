use std::convert::TryFrom;

use super::Point;
use crate::direction::Direction;
use crate::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct UnitPoint {
    e01: Scalar,
    e20: Scalar,
}

impl From<UnitPoint> for Point {
    fn from(up: UnitPoint) -> Point {
        Point {
            e01: up.e01,
            e20: up.e20,
            e12: 1f32,
        }
    }
}

impl TryFrom<Point> for UnitPoint {
    type Error = ();

    fn try_from(p: Point) -> Result<UnitPoint, ()> {
        let up = p * (1f32 / p.euc_norm());
        if up.is_finite() {
            Ok(UnitPoint {
                e01: up.e01,
                e20: up.e20,
            })
        } else {
            Err(())
        }
    }
}

impl std::ops::Mul<Scalar> for UnitPoint {
    type Output = UnitPoint;

    fn mul(self, s: Scalar) -> UnitPoint {
        UnitPoint {
            e01: self.e01 * s,
            e20: self.e20 * s,
        }
    }
}

impl std::ops::Mul<UnitPoint> for Scalar {
    type Output = UnitPoint;

    fn mul(self, up: UnitPoint) -> UnitPoint {
        up * self
    }
}

impl std::ops::Add<UnitPoint> for UnitPoint {
    type Output = UnitPoint;

    fn add(self, u: UnitPoint) -> UnitPoint {
        UnitPoint {
            e01: self.e01 + u.e01,
            e20: self.e20 + u.e20,
        }
    }
}

impl std::ops::Add<Direction> for UnitPoint {
    type Output = UnitPoint;

    fn add(self, d: Direction) -> UnitPoint {
        UnitPoint {
            e01: self.e01 + d.e01,
            e20: self.e20 + d.e20,
        }
    }
}
