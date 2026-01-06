# simple_rng

A minimal, dependency-free pseudo-random number generator (PRNG) library for Rust, based on a Linear Congruential Generator (LCG).

## Features
- Seedable random number generator
- Generate random integers, floats, booleans
- Generate random numbers in a range
- Pick random elements from slices
- Generate random signed/unsigned integers of specific bit sizes
- No external dependencies
- `no_std` compatible (default feature: `std`)

## Usage
Add to your `Cargo.toml`:

```toml
[dependencies]
simple_rng = "0.2.0"
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
- `RNG::from_time()` - Create seeded from system time (requires `std` feature)
- `next()` - Next random u64
- `gen_range(min, max)` - Random integer in [min, max]
- `gen_float()` / `gen_f32()` - Random float in [0.0, 1.0)
- `gen_bool()` - Random boolean
- `gen_unsigned(size: u8)` - Random unsigned integer (8, 16, 32, 64 bits)
- `gen_signed(size: u8)` - Random signed integer (8, 16, 32, 64 bits)
- `pick_random(slice)` - Pick random element from slice, returns `Option<&T>`

## Features
- `std` (enabled by default): Enables seeding from system time and other standard library features.
- `no_std`: Use in embedded or constrained environments.

## Minimum Supported Rust Edition
2024

## crates.io
https://crates.io/crates/simple_rng

## License
MIT
