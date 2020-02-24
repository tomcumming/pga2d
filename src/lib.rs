pub mod direction;
pub mod line;
pub mod point;
pub mod pseudoscalar;

/// Grade-0 Blade
pub type Scalar = f32;

/// The outer product, also known as the wedge product.
pub trait Meet<Rhs> {
    type Output;

    fn meet(self, rhs: Rhs) -> Self::Output;
}

/// The left contraction inner product.
pub trait Inner<Rhs> {
    type Output;

    fn inner(self, rhs: Rhs) -> Self::Output;
}

pub trait Dual {
    type Output;

    fn dual(self) -> Self::Output;
}

/// The regressive product.
pub trait Join<Rhs> {
    type Output;

    fn join(self, rhs: Rhs) -> Self::Output;
}
