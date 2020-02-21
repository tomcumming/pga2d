use std::convert::TryFrom;

use super::direction::Direction;
use super::Point;
use super::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct UnitDirection {
    e01: Scalar,
    e20: Scalar,
    _secret: (),
}

impl From<UnitDirection> for Point {
    fn from(ud: UnitDirection) -> Point {
        Point {
            e01: ud.e01,
            e20: ud.e20,
            e12: 0f32,
        }
    }
}

impl TryFrom<Direction> for UnitDirection {
    type Error = ();

    fn try_from(d: Direction) -> Result<UnitDirection, ()> {
        let n = Point::from(d).ideal_norm();
        let ud = d * (1f32 / n);
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
