use std::convert::TryFrom;

use super::Line;

pub struct UnitLine(Line);

impl TryFrom<Line> for UnitLine {
    type Error = ();

    fn try_from(l: Line) -> Result<UnitLine, ()> {
        let l2 = l * l.euc_norm();
        if l2.is_finite() {
            Ok(UnitLine(l2))
        } else {
            Err(())
        }
    }
}

impl From<UnitLine> for Line {
    fn from(UnitLine(l): UnitLine) -> Line {
        l
    }
}
