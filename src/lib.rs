use std::time::{SystemTime, UNIX_EPOCH};
/// Linear Congruential Generator (LCG) for pseudo-random number generation
pub struct RNG {
    seed: u64,
}

impl RNG {
    /// Constructs a new RNG with the given seed
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Constructs a new RNG seeded from the current system time
    pub fn from_time() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = now.as_nanos() as u64;
        Self { seed }
    }

    /// Advances the RNG and returns the next random u64 value
    pub fn next(&mut self) -> u64 {
        self.seed = lcg(self.seed);
        self.seed
    }

    /// Returns a random integer in the range [min, max] (inclusive)
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        if max <= min {
            panic!("The maximum value must always be greater than the minimum value.")
        }
        let range = max - min + 1;
        (self.next() % range) + min
    }

    /// Returns a random floating-point value in [0.0, 1.0)
    pub fn gen_float(&mut self) -> f64 {
        (self.next() as f64) / (u32::MAX as f64 + 1.0)
    }

    /// Returns a random boolean value
    pub fn gen_bool(&mut self) -> bool {
        self.next() & 1 == 1
    }

    /// Returns a random unsigned integer of the specified bit size
    pub fn gen_unsigned(&mut self, size: u8) -> usize {
        match size {
            8 => self.next() as u8 as usize,
            16 => self.next() as u16 as usize,
            32 => self.next() as u32 as usize,
            64 => self.next() as u64 as usize,
            _ => panic!("Unsupported size"),
        }
    }

    /// Returns a random signed integer of the specified bit size
    pub fn gen_signed(&mut self, size: u8) -> isize {
        match size {
            8 => self.next() as i8 as isize,
            16 => self.next() as i16 as isize,
            32 => self.next() as i32 as isize,
            64 => self.next() as i64 as isize,
            _ => panic!("Unsupported size"),
        }
    }

    /// Returns a random f32 value in [0.0, 1.0)
    pub fn gen_f32(&mut self) -> f32 {
        (self.next() as f32) / (u32::MAX as f32 + 1.0)
    }

    /// Selects a random element from a non-empty slice, or None if empty
    pub fn pick_random<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.gen_range(0, slice.len() as u64 - 1) as usize;
            slice.get(idx)
        }
    }
}

// Linear Congruential Generator function (private)
fn lcg(seed: u64) -> u64 {
    let a: u64 = 1664525;
    let c: u64 = 1013904223;
    (a.wrapping_mul(seed).wrapping_add(c) as u32) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Verifies that calling next() changes the RNG's seed
    fn test_next_changes_seed() {
        let mut rng = RNG::new(123);
        let old_seed = rng.seed;
        let _ = rng.next();
        assert_ne!(rng.seed, old_seed);
    }

    #[test]
    /// Ensures gen_range returns a value within the specified bounds
    fn test_gen_range_bounds() {
        let mut rng = RNG::new(42);
        let val = rng.gen_range(10, 20);
        assert!(val >= 10 && val < 20);
    }

    #[test]
    /// Checks that gen_bool produces both true and false values over many samples
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

    #[test]
    /// Tests that elements can be randomly removed from a vector until empty,
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
