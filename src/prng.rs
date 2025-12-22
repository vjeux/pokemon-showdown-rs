//! PRNG - Pseudo Random Number Generator
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This simulates the on-cartridge PRNG used in the real games.
//!
//! In addition to potentially allowing us to read replays from in-game,
//! this also makes it possible to record an "input log" (a seed +
//! initial teams + move/switch decisions) and "replay" a simulation to
//! get the same result.

use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A PRNG seed can be either a sodium seed or a Gen5 seed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PRNGSeed {
    /// Sodium/ChaCha20-based RNG seed (hex string)
    Sodium(String),
    /// Gen 5 RNG seed (4 x 16-bit values)
    Gen5([u16; 4]),
}

impl PRNGSeed {
    /// Parse a seed string into a PRNGSeed
    pub fn from_string(s: &str) -> Result<Self, String> {
        if s.starts_with("sodium,") {
            Ok(PRNGSeed::Sodium(s[7..].to_string()))
        } else if s.starts_with("gen5,") {
            let hex = &s[5..];
            if hex.len() < 16 {
                return Err(format!("Gen5 seed too short: {}", s));
            }
            let seed = [
                u16::from_str_radix(&hex[0..4], 16).map_err(|e| e.to_string())?,
                u16::from_str_radix(&hex[4..8], 16).map_err(|e| e.to_string())?,
                u16::from_str_radix(&hex[8..12], 16).map_err(|e| e.to_string())?,
                u16::from_str_radix(&hex[12..16], 16).map_err(|e| e.to_string())?,
            ];
            Ok(PRNGSeed::Gen5(seed))
        } else if s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            // Old format: comma-separated numbers
            let parts: Result<Vec<u16>, _> = s.split(',').map(|p| p.parse::<u16>()).collect();
            let parts = parts.map_err(|e| e.to_string())?;
            if parts.len() != 4 {
                return Err(format!("Expected 4 parts in seed, got {}", parts.len()));
            }
            Ok(PRNGSeed::Gen5([parts[0], parts[1], parts[2], parts[3]]))
        } else {
            Err(format!("Unrecognized RNG seed: {}", s))
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            PRNGSeed::Sodium(hex) => format!("sodium,{}", hex),
            PRNGSeed::Gen5(seed) => format!("{},{},{},{}", seed[0], seed[1], seed[2], seed[3]),
        }
    }
}

/// Low-level source of 32-bit random numbers
pub trait RNG {
    /// Get the current seed
    fn get_seed(&self) -> PRNGSeed;
    /// Get the next random 32-bit number
    fn next(&mut self) -> u32;
}

/// A PRNG intended to emulate the on-cartridge PRNG for Gen 5 with a 64-bit
/// initial seed.
#[derive(Debug, Clone)]
pub struct Gen5RNG {
    seed: [u16; 4],
}

impl Gen5RNG {
    /// Create a new Gen5RNG with the given seed
    pub fn new(seed: [u16; 4]) -> Self {
        Self { seed }
    }

    /// Generate a random seed
    pub fn generate_seed() -> [u16; 4] {
        let mut rng = rand::thread_rng();
        [
            rng.gen::<u16>(),
            rng.gen::<u16>(),
            rng.gen::<u16>(),
            rng.gen::<u16>(),
        ]
    }

    /// Calculates `a * b + c` (with 64-bit 2's complement integers)
    fn multiply_add(a: [u16; 4], b: [u16; 4], c: [u16; 4]) -> [u16; 4] {
        let mut out = [0u16; 4];
        let mut carry: u32 = 0;

        for out_index in (0..4).rev() {
            for b_index in out_index..4 {
                let a_index = 3 - (b_index - out_index);
                carry = carry.wrapping_add((a[a_index] as u32).wrapping_mul(b[b_index] as u32));
            }
            carry = carry.wrapping_add(c[out_index] as u32);
            out[out_index] = (carry & 0xFFFF) as u16;
            carry >>= 16;
        }

        out
    }

    /// The RNG is a Linear Congruential Generator (LCG) in the form: `x_{n + 1} = (a x_n + c) % m`
    ///
    /// Where: `x_0` is the seed, `x_n` is the random number after n iterations,
    ///
    /// ```text
    /// a = 0x5D588B656C078965
    /// c = 0x00269EC3
    /// m = 2^64
    /// ```
    fn next_frame(&mut self) {
        const A: [u16; 4] = [0x5D58, 0x8B65, 0x6C07, 0x8965];
        const C: [u16; 4] = [0, 0, 0x26, 0x9EC3];
        self.seed = Self::multiply_add(self.seed, A, C);
    }
}

impl RNG for Gen5RNG {
    fn get_seed(&self) -> PRNGSeed {
        PRNGSeed::Gen5(self.seed)
    }

    fn next(&mut self) -> u32 {
        self.next_frame();
        // Use the upper 32 bits
        ((self.seed[0] as u32) << 16) | (self.seed[1] as u32)
    }
}

/// Sodium-compatible RNG using ChaCha20
/// This is a drop-in replacement for libsodium's randombytes_buf_deterministic
#[derive(Debug, Clone)]
pub struct SodiumRNG {
    seed: [u8; 32],
}

impl SodiumRNG {
    /// Nonce chosen to be compatible with libsodium's randombytes_buf_deterministic
    const NONCE: [u8; 12] = *b"LibsodiumDRG";

    /// Create a new SodiumRNG with the given seed (hex string)
    pub fn new(hex_seed: &str) -> Self {
        let mut seed = [0u8; 32];
        // Pad with zeros if needed (generateSeed generates 16 bytes = 32 hex chars)
        let padded = format!("{:0<64}", hex_seed);
        for (i, chunk) in padded.as_bytes().chunks(2).enumerate().take(32) {
            if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16) {
                seed[i] = byte;
            }
        }
        Self { seed }
    }

    /// Generate a random seed
    pub fn generate_seed() -> String {
        let mut rng = rand::thread_rng();
        let seed: [u32; 4] = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
        format!(
            "{:08x}{:08x}{:08x}{:08x}",
            seed[0], seed[1], seed[2], seed[3]
        )
    }
}

impl RNG for SodiumRNG {
    fn get_seed(&self) -> PRNGSeed {
        let hex: String = self.seed.iter().map(|b| format!("{:02x}", b)).collect();
        PRNGSeed::Sodium(hex)
    }

    fn next(&mut self) -> u32 {
        // Create 36 bytes of zeros, encrypt to get random bytes
        let mut buf = [0u8; 36];

        // Create ChaCha20 cipher with our seed as key and the nonce
        let mut cipher = ChaCha20::new((&self.seed).into(), (&Self::NONCE).into());
        cipher.apply_keystream(&mut buf);

        // Use the first 32 bytes for the next seed
        self.seed.copy_from_slice(&buf[0..32]);

        // Use bytes 32-36 for the output (big-endian)
        ((buf[32] as u32) << 24)
            | ((buf[33] as u32) << 16)
            | ((buf[34] as u32) << 8)
            | (buf[35] as u32)
    }
}

/// High-level PRNG API, for getting random numbers.
///
/// Chooses the RNG implementation based on the seed passed to the constructor.
/// Seeds starting with 'sodium' use sodium. Other seeds use the Gen 5 RNG.
/// If a seed isn't given, defaults to sodium.
#[derive(Debug, Clone)]
pub struct PRNG {
    pub starting_seed: PRNGSeed,
    rng: PRNGImpl,
}

#[derive(Debug, Clone)]
enum PRNGImpl {
    Gen5(Gen5RNG),
    Sodium(SodiumRNG),
}

impl PRNG {
    /// Create a new PRNG with the given seed
    pub fn new(seed: Option<PRNGSeed>) -> Self {
        let seed = seed.unwrap_or_else(|| PRNGSeed::Sodium(SodiumRNG::generate_seed()));
        let starting_seed = seed.clone();
        let rng = match &seed {
            PRNGSeed::Sodium(hex) => PRNGImpl::Sodium(SodiumRNG::new(hex)),
            PRNGSeed::Gen5(s) => PRNGImpl::Gen5(Gen5RNG::new(*s)),
        };
        Self { starting_seed, rng }
    }

    /// Create a new PRNG from a seed string
    pub fn from_seed_string(seed: &str) -> Result<Self, String> {
        let parsed = PRNGSeed::from_string(seed)?;
        Ok(Self::new(Some(parsed)))
    }

    /// Get the current seed
    pub fn get_seed(&self) -> PRNGSeed {
        match &self.rng {
            PRNGImpl::Gen5(rng) => rng.get_seed(),
            PRNGImpl::Sodium(rng) => rng.get_seed(),
        }
    }

    /// Set the seed
    pub fn set_seed(&mut self, seed: PRNGSeed) {
        self.rng = match &seed {
            PRNGSeed::Sodium(hex) => PRNGImpl::Sodium(SodiumRNG::new(hex)),
            PRNGSeed::Gen5(s) => PRNGImpl::Gen5(Gen5RNG::new(*s)),
        };
    }

    /// Clone the PRNG with the current seed
    pub fn clone_with_current_seed(&self) -> Self {
        Self {
            starting_seed: self.starting_seed.clone(),
            rng: self.rng.clone(),
        }
    }

    /// Get the next random 32-bit number
    fn next_raw(&mut self) -> u32 {
        match &mut self.rng {
            PRNGImpl::Gen5(rng) => rng.next(),
            PRNGImpl::Sodium(rng) => rng.next(),
        }
    }

    /// Get a random number
    /// - `random()` returns a float in [0, 1)
    /// - `random(n)` returns an integer in [0, n)
    /// - `random(m, n)` returns an integer in [m, n)
    pub fn random(&mut self, from: Option<u32>, to: Option<u32>) -> f64 {
        let result = self.next_raw();

        match (from, to) {
            (None, None) => (result as f64) / (2f64.powi(32)),
            (Some(n), None) => ((result as f64) * (n as f64) / (2f64.powi(32))).floor(),
            (Some(m), Some(n)) => {
                ((result as f64) * ((n - m) as f64) / (2f64.powi(32))).floor() + (m as f64)
            }
            (None, Some(_)) => (result as f64) / (2f64.powi(32)), // Invalid case, treat as random()
        }
    }

    /// Get a random integer in [0, n)
    pub fn random_int(&mut self, n: u32) -> u32 {
        self.random(Some(n), None) as u32
    }

    /// Get a random integer in [from, to)
    pub fn random_range(&mut self, from: u32, to: u32) -> u32 {
        self.random(Some(from), Some(to)) as u32
    }

    /// Flip a coin (two-sided die), returning true or false.
    ///
    /// This function returns true with probability `P`, where `P = numerator / denominator`.
    pub fn random_chance(&mut self, numerator: u32, denominator: u32) -> bool {
        self.random_int(denominator) < numerator
    }

    /// Return a random item from the given slice.
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let index = self.random_int(items.len() as u32) as usize;
        Some(&items[index])
    }

    /// A Fisher-Yates shuffle. This is how the game resolves speed ties.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        self.shuffle_range(items, 0, items.len());
    }

    /// Shuffle a range of the slice
    pub fn shuffle_range<T>(&mut self, items: &mut [T], start: usize, end: usize) {
        let mut i = start;
        while i < end.saturating_sub(1) {
            let next_index = self.random_range(i as u32, end as u32) as usize;
            if i != next_index {
                items.swap(i, next_index);
            }
            i += 1;
        }
    }

    /// Generate a random seed (sodium format)
    pub fn generate_seed() -> PRNGSeed {
        PRNGSeed::Sodium(SodiumRNG::generate_seed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen5_rng_deterministic() {
        let mut rng1 = Gen5RNG::new([0x1234, 0x5678, 0x9ABC, 0xDEF0]);
        let mut rng2 = Gen5RNG::new([0x1234, 0x5678, 0x9ABC, 0xDEF0]);

        for _ in 0..100 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }

    #[test]
    fn test_prng_random_range() {
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));

        for _ in 0..100 {
            let val = prng.random_int(100);
            assert!(val < 100);
        }
    }

    #[test]
    fn test_prng_shuffle() {
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));
        let mut items: Vec<i32> = (0..10).collect();
        let original = items.clone();

        prng.shuffle(&mut items);

        // Should still have same elements
        let mut sorted = items.clone();
        sorted.sort();
        assert_eq!(sorted, original);
    }

    #[test]
    fn test_seed_parsing() {
        let seed = PRNGSeed::from_string("1234,5678,9012,3456").unwrap();
        assert!(matches!(seed, PRNGSeed::Gen5(_)));

        let seed = PRNGSeed::from_string("sodium,abcdef0123456789").unwrap();
        assert!(matches!(seed, PRNGSeed::Sodium(_)));
    }

    /// Test that our Gen5 PRNG matches the JavaScript implementation exactly
    #[test]
    fn test_gen5_matches_javascript() {
        // Test case 1: Seed [0x1234, 0x5678, 0x9ABC, 0xDEF0]
        // Expected from JS: [85,74,94,18,28,70,9,0,38,13,6,60,1,55,88,37,21,97,15,12]
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0x1234, 0x5678, 0x9ABC, 0xDEF0])));
        let expected = [85, 74, 94, 18, 28, 70, 9, 0, 38, 13, 6, 60, 1, 55, 88, 37, 21, 97, 15, 12];
        for (i, &exp) in expected.iter().enumerate() {
            let val = prng.random_int(100);
            assert_eq!(val, exp, "Mismatch at index {} for seed1", i);
        }

        // Test case 2: Seed [1, 2, 3, 4]
        // Expected from JS: [47,88,66,13,84,57,95,38,77,82,10,22,6,84,31,85,14,93,14,1]
        let mut prng2 = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));
        let expected2 = [47, 88, 66, 13, 84, 57, 95, 38, 77, 82, 10, 22, 6, 84, 31, 85, 14, 93, 14, 1];
        for (i, &exp) in expected2.iter().enumerate() {
            let val = prng2.random_int(100);
            assert_eq!(val, exp, "Mismatch at index {} for seed2", i);
        }

        // Test case 3: Raw 32-bit values with seed [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]
        // Expected from JS: [2795446293,2744431784,3501875646,746470024,1664743198,3087483402,966965533,2002677306,2482317012,1229306196]
        let mut rng3 = Gen5RNG::new([0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
        let expected_raw: [u32; 10] = [
            2795446293, 2744431784, 3501875646, 746470024, 1664743198,
            3087483402, 966965533, 2002677306, 2482317012, 1229306196
        ];
        for (i, &exp) in expected_raw.iter().enumerate() {
            let val = rng3.next();
            assert_eq!(val, exp, "Raw value mismatch at index {}", i);
        }
    }
}
