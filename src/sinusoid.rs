use std;
use std::ops::{Add, Sub, Mul, Div, Neg};
use ::num_traits::{Trig, Pow, Float};

// generic number type, but realistically it's only useful for
// floats. Maybe also complex<float>

pub struct Sinusoid<T> {
    amplitude: T,
    frequency: T,
    phase: T
}

impl<T> Sinusoid<T> {
    pub fn new(amplitude:T, frequency: T, phase: T) -> Sinusoid<T> {
        Sinusoid {
            amplitude: amplitude,
            frequency: frequency,
            phase: phase
        }
    }
}
impl<T> Sinusoid<T> where T: Float + Copy {
    pub fn period(&self) -> T {
        self.frequency.recip()
    }
}
impl<T> Sinusoid<T> where T: Float + Add<Output=T> + Mul<Output=T> + From<u8> + Trig + Copy + std::fmt::Display {
    pub fn radial_frequency(&self) -> T {
        (T::from(2))*T::PI()*self.frequency
    }
    pub fn sample(&self, t: T) -> T {
        (self.radial_frequency()*t + self.phase).cos() * self.amplitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    #[test]
    fn period() {
        let sinusoid = Sinusoid::new(1.0 as f32, 0.5, 0.0);
        assert!((sinusoid.period()-2.0) < f32::EPSILON);
    }
    #[test]
    fn radial_f() {
        let sinusoid = Sinusoid::new(1.0 as f32, 1.0, 0.0);
        assert!((sinusoid.radial_frequency()-2.0*f32::consts::PI) < f32::EPSILON);
    }

    #[test]
    fn sample() {
        let sinusoid = Sinusoid::new(1.0 as f32, 1.0, -f32::consts::PI/2.0); //AKA sin

        assert!((sinusoid.sample(0.0)-0.0) < f32::EPSILON);
        assert!((sinusoid.sample(0.25)-1.0) < f32::EPSILON);
        assert!((sinusoid.sample(0.5)-0.0) < f32::EPSILON);
        assert!((sinusoid.sample(0.75)+1.0) < f32::EPSILON);
        assert!((sinusoid.sample(1.0)-0.0) < f32::EPSILON);
    }
}
