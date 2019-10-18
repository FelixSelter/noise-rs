use crate::noise_fns::NoiseFn;
use rayon::prelude::*;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,
}

impl<'a, T> Abs<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Self { source }
    }
}

impl<'a, T> NoiseFn<T> for Abs<'a, T> {
    fn generate(&self, points: &[T]) -> Vec<f64> {
        self.source
            .generate(points)
            .par_iter()
            .map(|value| value.abs())
            .collect()
    }
}
