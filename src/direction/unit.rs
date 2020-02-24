use std::convert::TryFrom;

use super::Direction;
use crate::point::Point;

/// Unit direction with an ideal norm of 1
#[derive(Debug, Copy, Clone)]
pub struct UnitDirection(Direction);

impl From<UnitDirection> for Point {
    fn from(UnitDirection(ud): UnitDirection) -> Point {
        Point {
            e01: ud.e01,
            e20: ud.e20,
            e12: 0f32,
        }
    }
}

impl From<UnitDirection> for Direction {
    fn from(UnitDirection(ud): UnitDirection) -> Direction {
        ud
    }
}

impl TryFrom<Direction> for UnitDirection {
    type Error = ();

    fn try_from(d: Direction) -> Result<UnitDirection, ()> {
        let n = Point::from(d).ideal_norm();
        let ud = d * (1f32 / n);
        if Point::from(ud).is_finite() {
            Ok(UnitDirection(ud))
        } else {
            Err(())
        }
    }
}
