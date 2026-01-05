# simple_rng

A minimal, dependency-free pseudo-random number generator (PRNG) library for Rust, based on a Linear Congruential Generator (LCG).

## Features
- Seedable random number generator
- Generate random integers, floats, booleans
- Generate random numbers in a range
- Pick random elements from slices
- No external dependencies

## Usage
Add to your `Cargo.toml`:

```toml
[dependencies]
simple_rng = "0.1.0"
```

Import and use in your code:

```rust
use simple_rng::RNG;

fn main() {
    let mut rng = RNG::from_time();
    let random_number = rng.gen_range(1, 100);
    println!("Random number: {}", random_number);
}
```

## API Overview
- `RNG::new(seed: u64)` - Create with a custom seed
- `RNG::from_time()` - Create seeded from system time
- `next()` - Next random u64
- `gen_range(min, max)` - Random integer in [min, max]
- `gen_float()` / `gen_f32()` - Random float in [0.0, 1.0)
- `gen_bool()` - Random boolean
- `pick_random(slice)` - Pick random element from slice

## License
MIT

