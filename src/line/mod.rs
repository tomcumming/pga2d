use super::point::Point;
use super::Scalar;

pub struct Line {
    pub e0: Scalar,
    pub e1: Scalar,
    pub e2: Scalar,
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
