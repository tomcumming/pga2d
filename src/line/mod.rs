pub mod unit;

use crate::point::Point;
use crate::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub e0: Scalar,
    pub e1: Scalar,
    pub e2: Scalar,
}

impl std::ops::Mul<Scalar> for Line {
    type Output = Line;

    fn mul(self, s: Scalar) -> Line {
        Line {
            e0: self.e0 * s,
            e1: self.e1 * s,
            e2: self.e2 * s,
        }
    }
}

impl std::ops::Mul<Line> for Scalar {
    type Output = Line;

    fn mul(self, l: Line) -> Line {
        l * self
    }
}

impl super::Meet<Line> for Line {
    type Output = Point;

    fn meet(self, rhs: Line) -> Point {
        Point {
            e01: self.e0 * rhs.e1 + -self.e1 * rhs.e0,
            e20: -self.e0 * rhs.e2 + self.e2 * rhs.e0,
            e12: self.e1 * rhs.e2 + -self.e2 * rhs.e1,
        }
    }
}

impl super::Inner<Line> for Line {
    type Output = Scalar;

    fn inner(self, rhs: Line) -> Scalar {
        self.e1 * rhs.e1 + self.e2 * rhs.e2
    }
}

impl super::Dual for Line {
    type Output = Point;

    fn dual(self) -> Point {
        Point {
            e01: self.e2,
            e20: self.e1,
            e12: self.e0,
        }
    }
}

impl Line {
    pub fn is_finite(self) -> bool {
        self.e0.is_finite() && self.e1.is_finite() && self.e2.is_finite()
    }

    pub fn euc_norm(self) -> Scalar {
        Scalar::sqrt(Scalar::powi(self.e1, 2) + Scalar::powi(self.e2, 2))
    }

    pub fn ideal_norm(self) -> Scalar {
        self.e0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::unit::UnitPoint;
    use crate::point::Point;
    use crate::Meet;
    use std::convert::TryFrom;

    #[test]
    fn test_line_meet_simple_example() {
        // y = -(1/2)x + 2
        let line1 = Line {
            e0: 2f32,
            e1: -(1f32 / 2f32),
            e2: -1f32,
        };
        // y = -(3/2)x + 4
        let line2 = Line {
            e0: 4f32,
            e1: -(3f32 / 2f32),
            e2: -1f32,
        };

        let met = line1.meet(line2);
        let up = UnitPoint::try_from(met).unwrap();
        let p = Point::from(up);

        // Should intersect at (2, 1)
        assert_eq!(p.e20, 2f32);
        assert_eq!(p.e01, 1f32);
    }
}
