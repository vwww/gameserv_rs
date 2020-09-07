use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec2<T>(pub T, pub T);

impl<T: Add<U>, U> Add<Vec2<U>> for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Vec2<U>) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<U>, U> Sub<Vec2<U>> for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn sub(self, rhs: Vec2<U>) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<U>, U: Copy> Mul<U> for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div<U>, U: Copy> Div<U> for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

pub trait Hypot {
    fn hypot(self, other: Self) -> Self;
}

impl Hypot for f32 {
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }
}

impl Hypot for f64 {
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }
}

impl<T: Mul<T> + Copy> Vec2<T>
where
    T::Output: Add<T::Output>,
{
    pub fn dot(self, other: &Self) -> <T::Output as Add>::Output {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn len_sq(self) -> <T::Output as Add>::Output {
        self.dot(&self)
    }
}

impl<T: Hypot + Copy> Vec2<T> {
    pub fn len(self) -> T {
        self.0.hypot(self.1)
    }
}

impl<T: Hypot + Copy + Div<T>> Vec2<T> {
    pub fn normalize(self) -> Vec2<T::Output> {
        self / self.len()
    }
}
