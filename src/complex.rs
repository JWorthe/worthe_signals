use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    real: T,
    imag: T
}

impl<T: Clone> Complex<T> {
    fn new(real: T, imag: T) -> Complex<T> {
        Complex{real: real, imag: imag}
    }
}

impl<T> Add for Complex<T> where T: Add<Output=T> + Copy {
    type Output = Complex<T>;

    fn add(self, other: Self) -> Self {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        Complex::new(real, imag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition() {
        let a = Complex::new(1, 5);
        let b = Complex::new(-3, 2);
        assert_eq!(a+b, Complex::new(-2, 7));
    }
}

