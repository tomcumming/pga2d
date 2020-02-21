pub mod line;
pub mod point;
pub mod pseudoscalar;

pub type Scalar = f32;

pub trait Meet<Rhs> {
    type Output;

    fn meet(self, rhs: Rhs) -> Self::Output;
}

pub trait Inner<Rhs> {
    type Output;

    fn inner(self, rhs: Rhs) -> Self::Output;
}

pub trait Dual {
    type Output;

    fn dual(self) -> Self::Output;
}

pub trait Join<Rhs> {
    type Output;

    fn join(self, rhs: Rhs) -> Self::Output;
}
