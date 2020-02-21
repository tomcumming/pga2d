use std::convert::TryFrom;

use super::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub e01: Scalar,
    pub e20: Scalar,
    pub e12: Scalar,
}

#[derive(Debug, Copy, Clone)]
pub struct UnitPoint {
    pub e01: Scalar,
    pub e20: Scalar,
    pub e12: Scalar,
}

#[derive(Debug, Copy, Clone)]
pub struct Direction {
    pub e01: Scalar,
    pub e20: Scalar,
}

#[derive(Debug, Copy, Clone)]
pub struct UnitDirection {
    pub e01: Scalar,
    pub e20: Scalar,
    _secret: (),
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Point {
        Point {
            e01: d.e01,
            e20: d.e20,
            e12: 0 as Scalar,
        }
    }
}

impl From<UnitDirection> for Point {
    fn from(ud: UnitDirection) -> Point {
        Point {
            e01: ud.e01,
            e20: ud.e20,
            e12: 0 as Scalar,
        }
    }
}

impl From<UnitPoint> for Point {
    fn from(up: UnitPoint) -> Point {
        Point {
            e01: up.e01,
            e20: up.e20,
            e12: up.e12,
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

impl TryFrom<Point> for UnitPoint {
    type Error = ();

    fn try_from(p: Point) -> Result<UnitPoint, ()> {
        let up = p * (1 as Scalar / p.euc_norm());
        if up.is_finite() {
            Ok(UnitPoint {
                e01: up.e01,
                e20: up.e20,
                e12: up.e12,
            })
        } else {
            Err(())
        }
    }
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

impl TryFrom<Direction> for UnitDirection {
    type Error = ();

    fn try_from(d: Direction) -> Result<UnitDirection, ()> {
        let n = Point::from(d).ideal_norm();
        let ud = d * (1 as Scalar / n);
        if Point::from(ud).is_finite() {
            Ok(UnitDirection {
                e01: ud.e01,
                e20: ud.e20,
                _secret: (),
            })
        } else {
            Err(())
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
