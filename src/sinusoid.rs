use std::cmp::{PartialOrd};
use ::num_traits::{Trig, ArithmeticOps, FractionOps};

/// A data structure representing a sinusoid. AKA the sin or cos functions.
///
/// The general formula for a sinusoid is A cos(2πf + θ)
///
/// The number type is generic, but realistically it's only useful for
/// floats.
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
impl<T> Sinusoid<T> where T: FractionOps + Copy {
    /// The period is the time taken for each repetition of the
    /// sinusoid
    ///
    /// ```
    /// use worthe_signals::sinusoid::Sinusoid;
    /// use std::f32;
    ///
    /// let sinusoid = Sinusoid::new(1.0 as f32, 0.5, 0.0);
    /// assert!((sinusoid.period()-2.0) < f32::EPSILON);
    /// ```
    pub fn period(&self) -> T {
        self.frequency.recip()
    }
}
impl<T> Sinusoid<T> where T: FractionOps + ArithmeticOps + Trig + Copy {
    /// Frequency can be considered in terms of the signal's number of
    /// repetitions per second (referred to just as the frequency), or
    /// the frequency in radians.
    /// ```
    /// use worthe_signals::sinusoid::Sinusoid;
    /// use std::f32;
    ///
    /// let sinusoid = Sinusoid::new(1.0 as f32, 1.0, 0.0);
    /// assert!((sinusoid.radial_frequency()-2.0*f32::consts::PI) < f32::EPSILON);
    /// ```
    pub fn radial_frequency(&self) -> T {
        T::two_pi()*self.frequency
    }

    /// A sinusoid can be sampled to get its value at a given point in
    /// time.
    ///
    /// ```
    /// use worthe_signals::sinusoid::Sinusoid;
    /// use std::f32;
    ///
    /// let sinusoid = Sinusoid::new(1.0 as f32, 1.0, -f32::consts::FRAC_PI_2); //AKA sin
    /// assert!((sinusoid.sample(0.0)-0.0) < f32::EPSILON);
    /// assert!((sinusoid.sample(0.25)-1.0) < f32::EPSILON);
    /// assert!((sinusoid.sample(0.5)-0.0) < f32::EPSILON);
    /// assert!((sinusoid.sample(0.75)+1.0) < f32::EPSILON);
    /// assert!((sinusoid.sample(1.0)-0.0) < f32::EPSILON);
    /// ```
    pub fn sample(&self, t: T) -> T {
        (self.radial_frequency()*(t%self.period()) + self.phase).cos() * self.amplitude
    }
}
impl<T> Sinusoid<T> where T: FractionOps + ArithmeticOps + From<u16> + Trig + Copy + PartialOrd {
    /// Sometimes, it's useful to sample at all of the points in a range
    ///
    /// Start value is inclusive. End value is exclusive.
    ///
    /// ```
    /// use worthe_signals::sinusoid::Sinusoid;
    /// use std::f32;
    ///
    /// let sinusoid = Sinusoid::new(1.0 as f32, 1.0, -f32::consts::FRAC_PI_2); //AKA sin
    /// let samples = sinusoid.sample_range(0.0, 100.0, 4.0);
    /// assert_eq!(samples.len(), 400);
    /// for i in (0..100).map(|i| i*4) {
    ///     assert!((samples[i+0]-0.0) < f32::EPSILON, "Sample {} was {}", i+0, samples[i+0]);
    ///     assert!((samples[i+1]-1.0) < f32::EPSILON, "Sample {} was {}", i+1, samples[i+1]);
    ///     assert!((samples[i+2]-0.0) < f32::EPSILON, "Sample {} was {}", i+2, samples[i+2]);
    ///     assert!((samples[i+3]+1.0) < f32::EPSILON, "Sample {} was {}", i+3, samples[i+3]);
    /// }
    /// ```
    pub fn sample_range(&self, start: T, end: T, sample_rate: T) -> Vec<T> {
        let mut result = Vec::new();
        let mut i: u16 = 0;
        loop {
            let t = start + T::from(i)/sample_rate;
            if t >= end {
                break;
            }
            result.push(self.sample(t));
            i += 1;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

}
