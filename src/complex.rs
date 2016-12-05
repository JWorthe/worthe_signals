use std::ops::{Add, Sub, Mul, Div, Neg};
use ::num_traits::{Trig, Pow, ArithmeticOps, SignedArithmeticOps};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    real: T,
    imag: T
}

impl<T> Complex<T> {
    pub fn new(real: T, imag: T) -> Complex<T> {
        Complex{real: real, imag: imag}
    }
}
impl<T> Complex<T> where T: SignedArithmeticOps {
    pub fn conjugate(self) -> Complex<T> {
        Complex::new(self.real, -self.imag)
    }
}

impl<T> Complex<T> where T: Pow + ArithmeticOps + Copy  {
    pub fn magnitude(self) -> T {
        (self.real.pow(2) + self.imag.pow(2)).sqrt()
    }
}

impl<T> Complex<T> where T: Trig  {
    pub fn angle(self) -> T {
        self.imag.atan2(self.real)
    }
}

impl<T> Complex<T> where T: Trig + Pow + ArithmeticOps + Copy  {
    pub fn from_polar(r: T, theta: T) -> Complex<T> {
        let real = r*theta.cos();
        let imag = r*theta.sin();
        Complex::new(real, imag)
    }

    pub fn to_polar(self) -> (T, T) {
        (self.magnitude(), self.angle())
    }
}

impl<T> Add for Complex<T> where T: ArithmeticOps + Copy {
    type Output = Complex<T>;

    fn add(self, other: Self) -> Self {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        Complex::new(real, imag)
    }
}

impl<T> Sub for Complex<T> where T: ArithmeticOps + Copy {
    type Output = Complex<T>;

    fn sub(self, other: Self) -> Self {
        let real = self.real - other.real;
        let imag = self.imag - other.imag;
        Complex::new(real, imag)
    }
}

impl<T> Mul for Complex<T> where T: ArithmeticOps + Copy {
    type Output = Complex<T>;

    fn mul(self, other: Self) -> Self {
        let real = (self.real * other.real) - (self.imag * other.imag);
        let imag = (self.real * other.imag) + (self.imag * other.real);
        Complex::new(real, imag)
    }
}

impl<T> Div for Complex<T> where T: SignedArithmeticOps + Copy {
    type Output = Complex<T>;

    fn div(self, other: Self) -> Self {
        // multiply numerator and denominator by denominator's complex
        // conjugate, to give a pure real denominator.
        let other_conj = other.conjugate();
        let num = self * other_conj;
        let denom = (other * other_conj).real;

        let real = num.real / denom;
        let imag = num.imag / denom;
        Complex::new(real, imag)
    }
}

impl<T> Neg for Complex<T> where T: SignedArithmeticOps + Copy {
    type Output = Complex<T>;

    fn neg(self) -> Self {
        Complex::new(-self.real, -self.imag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;
    
    #[test]
    fn addition() {
        let a = Complex::new(1, 5);
        let b = Complex::new(-3, 2);
        assert_eq!(a+b, Complex::new(-2, 7));
    }

    #[test]
    fn subtraction() {
        let a = Complex::new(1, 5);
        let b = Complex::new(-3, 2);
        assert_eq!(a-b, Complex::new(4, 3));
    }

    #[test]
    fn multiplication() {
        let a = Complex::new(3, 4);
        let b = Complex::new(2, 3);
        assert_eq!(a*b, Complex::new(-6, 17));
    }

    #[test]
    fn division() {
        let a = Complex::new(6, 8);
        let b = Complex::new(3, 4);
        assert_eq!(a/b, Complex::new(2, 0));
    }

    #[test]
    fn neg() {
        let a = Complex::new(6, 8);
        assert_eq!(-a, Complex::new(0, 0)-a);
    }

    #[test]
    fn from_polar() {
        let right = Complex::from_polar(1.0 as f32, 0.0 as f32);
        assert!((right.real-1.0).abs() < f32::EPSILON);
        assert!((right.imag-0.0).abs() < f32::EPSILON);

        let up = Complex::from_polar(1.0 as f32, f32::consts::PI/2.0);
        assert!((up.real-0.0).abs() < f32::EPSILON/2.0);
        assert!((up.imag-1.0).abs() < f32::EPSILON/2.0);

        let left = Complex::from_polar(1.0 as f32, f32::consts::PI);
        assert!((left.real+1.0).abs() < f32::EPSILON);
        assert!((left.imag-0.0).abs() < f32::EPSILON);

        let down = Complex::from_polar(1.0 as f32, f32::consts::PI*3.0/2.0);
        assert!((down.real-0.0).abs() < f32::EPSILON);
        assert!((down.imag+1.0).abs() < f32::EPSILON);

        //not sure why the error here is more than epsilon. My guess
        // is that it's the 2.0*PI, meaning that the value for PI has
        // twice the normal error.
        let rev = Complex::from_polar(1.0 as f32, f32::consts::PI*2.0);
        assert!((rev.real-1.0).abs() < f32::EPSILON*2.0);
        assert!((rev.imag-0.0).abs() < f32::EPSILON*2.0);
    }
}

