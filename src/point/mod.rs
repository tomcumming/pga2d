pub mod unit;

use crate::direction::Direction;
use crate::line::Line;
use crate::pseudoscalar::PseudoScalar;
use crate::Scalar;

/// Grade-2 blade
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

impl std::ops::Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        p * self
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

impl super::Meet<Line> for Point {
    type Output = PseudoScalar;

    fn meet(self, l: Line) -> PseudoScalar {
        l.meet(self)
    }
}

impl super::Inner<Point> for Point {
    type Output = Scalar;

    fn inner(self, rhs: Point) -> Scalar {
        -self.e12 * rhs.e12
    }
}

impl super::Inner<Line> for Point {
    type Output = Line;

    fn inner(self, l: Line) -> Line {
        Line {
            e0: self.e01 * l.e1 + -self.e20 * l.e2,
            e1: self.e12 * l.e2,
            e2: -self.e12 * l.e1,
        }
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
    pub fn euc_norm(&self) -> Scalar {
        self.e12
    }

    pub fn ideal_norm(&self) -> Scalar {
        Scalar::sqrt(self.e20.powi(2) + self.e01.powi(2))
    }

    pub fn is_finite(&self) -> bool {
        self.e01.is_finite() && self.e20.is_finite() && self.e12.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;
    use crate::Inner;

    #[test]
    fn point_line_inner_simple_example() {
        let p = Point {
            e01: 2f32,
            e20: 3f32,
            e12: 4f32,
        };
        let l = Line {
            e0: 5f32,
            e1: 6f32,
            e2: 7f32,
        };
        let expected = Line {
            e0: 2f32 * 6f32 + -3f32 * 7f32,
            e1: 4f32 * 7f32,
            e2: -4f32 * 6f32,
        };
        let actual = p.inner(l);
        assert_eq!(actual.e0, expected.e0);
        assert_eq!(actual.e1, expected.e1);
        assert_eq!(actual.e2, expected.e2);
    }
}
