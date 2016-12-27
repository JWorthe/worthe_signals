use std::cmp::{PartialOrd};
use ::num_traits::{Trig, Pow, ArithmeticOps, FractionOps};
use ::complex::Complex;

/// A data structure representing a sinusoid. AKA the sin or cos functions.
///
/// The general formula for a sinusoid is A cos(2πf + θ)
///
/// The number type is generic, but realistically it's only useful for
/// floats.
#[derive(Debug, Clone, PartialEq)]
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
    /// assert!((sinusoid.period()-2.0).abs() < f32::EPSILON);
    /// ```
    pub fn period(&self) -> T {
        self.frequency.recip()
    }
}

#[derive(Debug, PartialEq)]
pub enum AddSinusoidError {
    DifferentFrequency
}

impl<T> Sinusoid<T> where T: Trig + Pow + ArithmeticOps + Copy {
    /// Converts the sinusoid to phasor form
    ///
    /// This discards the frequency. Take care to only compare phasors
    /// that had the same frequency before their conversion.
    ///
    /// ```
    /// use worthe_signals::sinusoid::Sinusoid;
    /// use worthe_signals::complex::Complex;
    /// use std::f32;
    
    /// let pure_real = Sinusoid::new(1.0 as f32, 0.5, 0.0);    
    /// let pure_real_phasor = pure_real.to_phasor();
    /// assert!((pure_real_phasor.real - 1.0).abs() < f32::EPSILON);
    /// assert!((pure_real_phasor.imag - 0.0).abs() < f32::EPSILON);
    ///
    /// let pure_imag = Sinusoid::new(1.0 as f32, 2.0, -f32::consts::FRAC_PI_2);
    /// let pure_imag_phasor = pure_imag.to_phasor();
    /// assert!((pure_imag_phasor.real - 0.0).abs() < f32::EPSILON);
    /// assert!((pure_imag_phasor.imag + 1.0).abs() < f32::EPSILON);
    /// ```
    pub fn to_phasor(self) -> Complex<T> {
        Complex::from_polar(self.amplitude, self.phase)
    }
}

impl<T> Sinusoid<T> where T: Trig + Pow + ArithmeticOps + PartialEq + Copy {
    /// Adds two sinusoids together into one sinusoid.
    ///
    /// # Errors
    ///
    /// This can only be done if the two sinusoids have the same
    /// frequency.
    ///
    /// ```
    /// use worthe_signals::sinusoid::{Sinusoid, AddSinusoidError};
    ///
    /// let sin1 = Sinusoid::new(1.0 as f32, 0.5, 0.0);
    /// let sin2 = Sinusoid::new(1.0 as f32, 2.0, 0.0);
    /// assert_eq!(Err(AddSinusoidError::DifferentFrequency), sin1.add(sin2));
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use worthe_signals::sinusoid::{Sinusoid, AddSinusoidError};
    /// use std::f32;
    ///
    /// let sin1 = Sinusoid::new(-3.0 as f32, 1.0, 0.0);
    /// let sin2 = Sinusoid::new(4.0 as f32, 1.0, -f32::consts::FRAC_PI_2);
    /// assert_eq!(Ok(Sinusoid::new(5.0, 1.0, -2.21429743558818100603413092035707408014009529080286529335)), sin1.add(sin2));
    /// ```
    pub fn add(self, other: Sinusoid<T>) -> Result<Self, AddSinusoidError> {
        if self.frequency != other.frequency {
            Err(AddSinusoidError::DifferentFrequency)
        }
        else {
            let frequency = self.frequency;
            let self_phasor = self.to_phasor();
            let other_phasor = other.to_phasor();
            let combined = self_phasor + other_phasor;
            let (amplitude, phase) = combined.to_polar();
            Ok(Sinusoid::new(amplitude, frequency, phase))
        }
        
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
    /// assert!((sinusoid.sample(0.0)-0.0).abs() < f32::EPSILON);
    /// assert!((sinusoid.sample(0.25)-1.0).abs() < f32::EPSILON);
    /// assert!((sinusoid.sample(0.5)-0.0).abs() < f32::EPSILON);
    /// assert!((sinusoid.sample(0.75)+1.0).abs() < f32::EPSILON);
    /// assert!((sinusoid.sample(1.0)-0.0).abs() < f32::EPSILON);
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
    ///     assert!((samples[i+0]-0.0).abs() < f32::EPSILON, "Sample {} was {}", i+0, samples[i+0]);
    ///     assert!((samples[i+1]-1.0).abs() < f32::EPSILON, "Sample {} was {}", i+1, samples[i+1]);
    ///     assert!((samples[i+2]-0.0).abs() < f32::EPSILON, "Sample {} was {}", i+2, samples[i+2]);
    ///     assert!((samples[i+3]+1.0).abs() < f32::EPSILON, "Sample {} was {}", i+3, samples[i+3]);
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

}
*/
