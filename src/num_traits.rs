pub trait Trig<T> {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

impl Trig<f32> for f32 {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}
impl Trig<f64> for f64 {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}


pub trait Pow<T> {
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: T) -> Self;
    fn sqrt(self) -> Self;
}

impl Pow<f32> for f32 {
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Pow<f64> for f64 {
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
