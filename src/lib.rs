#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// Supported random number generator algorithms
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    /// Linear Congruential Generator (default)
    Lcg,
    /// Permuted Congruential Generator (requires `pcg` feature)
    #[cfg(feature = "pcg")]
    Pcg,
}

/// A simple, seedable pseudo-random number generator
///
/// # Example
/// ```rust
/// use simple_rng::RNG;
/// let mut rng = RNG::new(0);
/// let value = rng.next();
/// println!("{}", value);
/// ```
pub struct RNG {
    seed: u64,
    algorithm: Algorithm,
}

impl RNG {
    /// Create a new RNG with the given seed
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::new(84);
    /// ```
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            algorithm: Algorithm::Lcg,
        }
    }

    /// Create a new RNG seeded from the current system time
    ///
    /// Only available with the `std` feature.
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::from_time();
    /// ```
    #[cfg(feature = "std")]
    pub fn from_time() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = now.as_nanos() as u64;
        Self {
            seed,
            algorithm: Algorithm::Lcg,
        }
    }

    /// Set the RNG algorithm (LCG or PCG)
    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }

    /// Advance the RNG and return the next random u64 value
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::from_time();
    /// let value = rng.next();
    /// println!("{}", value);
    /// ```
    pub fn next(&mut self) -> u64 {
        self.seed = match self.algorithm {
            Algorithm::Lcg => lcg(self.seed),
            #[cfg(feature = "pcg")]
            Algorithm::Pcg => pcg(self.seed),
        };
        self.seed
    }

    /// Generate a random integer in the range [min, max] (inclusive)
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::from_time();
    /// let value = rng.gen_range(1, 10);
    /// println!("{}", value);
    /// ```
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        if max <= min {
            panic!("max must be greater than min")
        }
        let range = max - min + 1;
        (self.next() % range) + min
    }

    /// Generate a random floating-point value in [0.0, 1.0)
    pub fn gen_float(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX as f64 + 1.0)
    }

    /// Generate a random boolean value
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::from_time();
    /// let side = rng.gen_bool();
    /// println!("{}", if side { "Heads" } else { "Tails" });
    /// ```
    pub fn gen_bool(&mut self) -> bool {
        self.next() & 1 == 1
    }

    /// Generate a random unsigned integer of the specified bit size (8, 16, 32, 64)
    pub fn gen_unsigned(&mut self, size: u8) -> usize {
        match size {
            8 => self.next() as u8 as usize,
            16 => self.next() as u16 as usize,
            32 => self.next() as u32 as usize,
            64 => self.next() as u64 as usize,
            _ => panic!("Unsupported size"),
        }
    }

    /// Generate a random signed integer of the specified bit size (8, 16, 32, 64)
    pub fn gen_signed(&mut self, size: u8) -> isize {
        match size {
            8 => self.next() as i8 as isize,
            16 => self.next() as i16 as isize,
            32 => self.next() as i32 as isize,
            64 => self.next() as i64 as isize,
            _ => panic!("Unsupported size"),
        }
    }

    /// Pick a random element from a non-empty slice, or None if empty
    ///
    /// # Example
    /// ```rust
    /// use simple_rng::RNG;
    /// let mut rng = RNG::new(123);
    /// let v = vec![1, 2, 3, 4];
    /// let pick = rng.pick_random(&v);
    /// println!("{:?}", pick);
    /// ```
    pub fn pick_random<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.gen_range(0, slice.len() as u64 - 1) as usize;
            slice.get(idx)
        }
    }
}

// Linear Congruential Generator (LCG) function
fn lcg(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

/// Permuted Congruential Generator (PCG-XSH-RR)
///
/// Uses LCG as the internal engine, then scrambles output for improved randomness.
/// Only available with the `pcg` feature.
#[cfg(feature = "pcg")]
fn pcg(seed: u64) -> u64 {
    let state = lcg(seed);
    let xorshifted = ((state >> 18) ^ state) >> 27;
    let rot = (state >> 59) as u32;
    xorshifted.rotate_right(rot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// next() should change the RNG's seed
    fn test_next_changes_seed() {
        let mut rng = RNG::new(123);
        let old_seed = rng.seed;
        let _ = rng.next();
        assert_ne!(rng.seed, old_seed);
    }

    #[test]
    /// gen_range returns a value within the specified bounds
    fn test_gen_range_bounds() {
        let mut rng = RNG::new(42);
        let val = rng.gen_range(10, 20);
        assert!(val >= 10 && val <= 20);
    }

    #[test]
    /// gen_bool produces both true and false values over many samples
    fn test_gen_bool_distribution() {
        let mut rng = RNG::new(1);
        let mut trues = 0;
        let mut falses = 0;
        for _ in 0..1000 {
            if rng.gen_bool() {
                trues += 1;
            } else {
                falses += 1;
            }
        }
        assert!(trues > 0 && falses > 0);
    }
}

#[cfg(all(test, feature = "std"))]
mod std_tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_shuffle() {
        let mut rng = RNG::new(123);
        let mut v = vec![1, 2, 3, 4];
        while v.len() > 1 {
            let idx = rng.gen_range(0, v.len() as u64 - 1) as usize;
            v.remove(idx);
        }
        if !v.is_empty() {
            v.remove(0);
        }
        assert!(v.is_empty());
    }
}
