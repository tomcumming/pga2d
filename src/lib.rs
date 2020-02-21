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
