use math;
use math::{Point2, Point3, Point4};
use noise_fns::{MultiFractal, NoiseFn, Perlin, Seedable};
use std;

/// Noise function that outputs hybrid Multifractal noise.
///
/// The result of this multifractal noise is that valleys in the noise should
/// have smooth bottoms at all altitudes.
#[derive(Clone, Debug)]
pub struct HybridMulti {
    /// Total number of frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: usize,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: f64,

    seed: u32,
    sources: Vec<Perlin>,
}

impl HybridMulti {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVES: usize = 6;
    pub const DEFAULT_FREQUENCY: f64 = 2.0;
    pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.25;
    pub const MAX_OCTAVES: usize = 32;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            octaves: Self::DEFAULT_OCTAVES,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            sources: super::build_sources(Self::DEFAULT_SEED, Self::DEFAULT_OCTAVES),
        }
    }
}

impl Default for HybridMulti {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFractal for HybridMulti {
    fn set_octaves(self, mut octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        octaves = math::clamp(octaves, 1, Self::MAX_OCTAVES);
        Self {
            octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            ..self
        }
    }
}

impl Seedable for HybridMulti {
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        Self {
            seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional `HybridMulti` noise
impl NoiseFn<Point2<f64>> for HybridMulti {
    fn get(&self, mut point: Point2<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul2(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            weight = weight.max(1.0);

            // Raise the spatial frequency.
            point = math::mul2(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add it in, weighted by previous octave's noise value.
            result += weight * signal;

            // Update the weighting value.
            weight *= signal;
        }

        // Scale the result to the [-1,1] range
        result * 3.0
    }
}

/// 3-dimensional `HybridMulti` noise
impl NoiseFn<Point3<f64>> for HybridMulti {
    fn get(&self, mut point: Point3<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul3(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            weight = weight.max(1.0);

            // Raise the spatial frequency.
            point = math::mul3(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add it in, weighted by previous octave's noise value.
            result += weight * signal;

            // Update the weighting value.
            weight *= signal;
        }

        // Scale the result to the [-1,1] range
        result * 3.0
    }
}

/// 4-dimensional `HybridMulti` noise
impl NoiseFn<Point4<f64>> for HybridMulti {
    fn get(&self, mut point: Point4<f64>) -> f64 {
        // First unscaled octave of function; later octaves are scaled.
        point = math::mul4(point, self.frequency);
        let mut result = self.sources[0].get(point) * self.persistence;
        let mut weight = result;

        // Spectral construction inner loop, where the fractal is built.
        for x in 1..self.octaves {
            // Prevent divergence.
            weight = weight.max(1.0);

            // Raise the spatial frequency.
            point = math::mul4(point, self.lacunarity);

            // Get noise value.
            let mut signal = self.sources[x].get(point);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add it in, weighted by previous octave's noise value.
            result += weight * signal;

            // Update the weighting value.
            weight *= signal;
        }

        // Scale the result to the [-1,1] range
        result * 3.0
    }
}
