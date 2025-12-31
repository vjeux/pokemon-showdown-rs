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
        if let Some(hex) = s.strip_prefix("sodium,") {
            Ok(PRNGSeed::Sodium(hex.to_string()))
        } else if let Some(hex) = s.strip_prefix("gen5,") {
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
        } else if s
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
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
}

impl std::fmt::Display for PRNGSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PRNGSeed::Sodium(hex) => write!(f, "sodium,{}", hex),
            PRNGSeed::Gen5(seed) => write!(f, "{},{},{},{}", seed[0], seed[1], seed[2], seed[3]),
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
    // TypeScript source:
    // /**
    // 	 * Calculates `a * b + c` (with 64-bit 2's complement integers)
    // 	 */
    // 	multiplyAdd(a: Gen5RNGSeed, b: Gen5RNGSeed, c: Gen5RNGSeed) {
    // 		// If you've done long multiplication, this is the same thing.
    // 		const out: Gen5RNGSeed = [0, 0, 0, 0];
    // 		let carry = 0;
    //
    // 		for (let outIndex = 3; outIndex >= 0; outIndex--) {
    // 			for (let bIndex = outIndex; bIndex < 4; bIndex++) {
    // 				const aIndex = 3 - (bIndex - outIndex);
    //
    // 				carry += a[aIndex] * b[bIndex];
    // 			}
    // 			carry += c[outIndex];
    //
    // 			out[outIndex] = carry & 0xFFFF;
    // 			carry >>>= 16;
    // 		}
    //
    // 		return out;
    // 	}
    //
    fn multiply_add(a: [u16; 4], b: [u16; 4], c: [u16; 4]) -> [u16; 4] {
        let mut out = [0u16; 4];
        let mut carry: u32 = 0;

        for out_index in (0..4).rev() {
            for (local_b_index, &b_val) in b[out_index..].iter().enumerate() {
                let a_index = 3 - local_b_index;
                carry = carry.wrapping_add((a[a_index] as u32).wrapping_mul(b_val as u32));
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
    // TypeScript source:
    // /**
    // 	 * The RNG is a Linear Congruential Generator (LCG) in the form: `x_{n + 1} = (a x_n + c) % m`
    // 	 *
    // 	 * Where: `x_0` is the seed, `x_n` is the random number after n iterations,
    // 	 *
    // 	 * ````
    // 	 * a = 0x5D588B656C078965
    // 	 * c = 0x00269EC3
    // 	 * m = 2^64
    // 	 * ````
    // 	 */
    // 	nextFrame(seed: Gen5RNGSeed, framesToAdvance = 1): Gen5RNGSeed {
    // 		const a: Gen5RNGSeed = [0x5D58, 0x8B65, 0x6C07, 0x8965];
    // 		const c: Gen5RNGSeed = [0, 0, 0x26, 0x9EC3];
    //
    // 		for (let i = 0; i < framesToAdvance; i++) {
    // 			// seed = seed * a + c
    // 			seed = this.multiplyAdd(seed, a, c);
    // 		}
    //
    // 		return seed;
    // 	}
    //
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
        let result = ((self.seed[0] as u32) << 16) | (self.seed[1] as u32);
        eprintln!("[RS PRNG] next() -> {} (seed: {},{},{},{})", result, self.seed[0], self.seed[1], self.seed[2], self.seed[3]);
        result
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

    // TypeScript source:
    //
    //
    // 	next() {
    // 		const zeroBuf = new Uint8Array(36);
    // 		// tested to do the exact same thing as
    // 		// sodium.randombytes_buf_deterministic(buf, this.seed);
    // 		const buf = new Chacha20(this.seed, SodiumRNG.NONCE).encrypt(zeroBuf);
    //
    // 		// use the first 32 bytes for the next seed, and the next 4 bytes for the output
    // 		this.seed = buf.slice(0, 32);
    // 		// reading big-endian
    // 		return buf.slice(32, 36).reduce((a, b) => a * 256 + b);
    // 	}
    //
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

impl Default for PRNG {
    fn default() -> Self {
        Self::new(None)
    }
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
    // 	getSeed(): PRNGSeed {
    // 		return this.rng.getSeed();
    // 	}
    //
    pub fn get_seed(&self) -> PRNGSeed {
        match &self.rng {
            PRNGImpl::Gen5(rng) => rng.get_seed(),
            PRNGImpl::Sodium(rng) => rng.get_seed(),
        }
    }

    /// Set the seed
    //
    // 	setSeed(seed: PRNGSeed) {
    // 		if (seed.startsWith('sodium,')) {
    // 			this.rng = new SodiumRNG(seed.split(',') as SodiumRNGSeed);
    // 		} else if (seed.startsWith('gen5,')) {
    // 			const gen5Seed = [seed.slice(5, 9), seed.slice(9, 13), seed.slice(13, 17), seed.slice(17, 21)];
    // 			this.rng = new Gen5RNG(gen5Seed.map(n => parseInt(n, 16)) as Gen5RNGSeed);
    // 		} else if (/[0-9]/.test(seed.charAt(0))) {
    // 			this.rng = new Gen5RNG(seed.split(',').map(Number) as Gen5RNGSeed);
    // 		} else {
    // 			throw new Error(`Unrecognized RNG seed ${seed}`);
    // 		}
    // 	}
    //
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
    // TypeScript source:
    // /**
    // 	 * Retrieves the next random number in the sequence.
    // 	 * This function has three different results, depending on arguments:
    // 	 * - random() returns a real number in [0, 1), just like Math.random()
    // 	 * - random(n) returns an integer in [0, n)
    // 	 * - random(m, n) returns an integer in [m, n)
    // 	 * m and n are converted to integers via Math.floor. If the result is NaN, they are ignored.
    // 	 */
    // 	random(from?: number, to?: number): number {
    // 		const result = this.rng.next();
    //
    // 		if (from) from = Math.floor(from);
    // 		if (to) to = Math.floor(to);
    // 		if (from === undefined) {
    // 			return result / 2 ** 32;
    // 		} else if (!to) {
    // 			return Math.floor(result * from / 2 ** 32);
    // 		} else {
    // 			return Math.floor(result * (to - from) / 2 ** 32) + from;
    // 		}
    // 	}
    //
    pub fn random(&mut self, from: Option<i32>, to: Option<i32>) -> f64 {
        let result = self.next_raw();

        let value = match (from, to) {
            (None, None) => (result as f64) / (2f64.powi(32)),
            (Some(n), None) => ((result as f64) * (n as f64) / (2f64.powi(32))).floor(),
            (Some(m), Some(n)) => {
                ((result as f64) * ((n - m) as f64) / (2f64.powi(32))).floor() + (m as f64)
            }
            (None, Some(_)) => (result as f64) / (2f64.powi(32)), // Invalid case, treat as random()
        };

        value
    }

    /// Get a random integer in [0, n)
    pub fn random_int(&mut self, n: i32) -> i32 {
        self.random(Some(n), None) as i32
    }

    /// Get a random integer in [from, to)
    pub fn random_range(&mut self, from: i32, to: i32) -> i32 {
        self.random(Some(from), Some(to)) as i32
    }

    /// Flip a coin (two-sided die), returning true or false.
    ///
    /// This function returns true with probability `P`, where `P = numerator / denominator`.
    // TypeScript source:
    // /**
    // 	 * Flip a coin (two-sided die), returning true or false.
    // 	 *
    // 	 * This function returns true with probability `P`, where `P = numerator
    // 	 * / denominator`. This function returns false with probability `1 - P`.
    // 	 *
    // 	 * The numerator must be a non-negative integer (`>= 0`).
    // 	 *
    // 	 * The denominator must be a positive integer (`> 0`).
    // 	 */
    // 	randomChance(numerator: number, denominator: number): boolean {
    // 		return this.random(denominator) < numerator;
    // 	}
    //
    pub fn random_chance(&mut self, numerator: i32, denominator: i32) -> bool {
        // JS: return this.random(denominator) < numerator;
        let roll = self.random(Some(denominator), None) as i32;
        roll < numerator
    }

    /// Return a random item from the given slice.
    // TypeScript source:
    // /**
    // 	 * Return a random item from the given array.
    // 	 *
    // 	 * This function chooses items in the array with equal probability.
    // 	 *
    // 	 * If there are duplicate items in the array, each duplicate is
    // 	 * considered separately. For example, sample(['x', 'x', 'y']) returns
    // 	 * 'x' 67% of the time and 'y' 33% of the time.
    // 	 *
    // 	 * The array must contain at least one item.
    // 	 *
    // 	 * The array must not be sparse.
    // 	 */
    // 	sample<T>(items: readonly T[]): T {
    // 		if (items.length === 0) {
    // 			throw new RangeError(`Cannot sample an empty array`);
    // 		}
    // 		const index = this.random(items.length);
    // 		const item = items[index];
    // 		if (item === undefined && !Object.prototype.hasOwnProperty.call(items, index)) {
    // 			throw new RangeError(`Cannot sample a sparse array`);
    // 		}
    // 		return item;
    // 	}
    //
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let index = self.random_int(items.len() as i32) as usize;
        Some(&items[index])
    }

    /// A Fisher-Yates shuffle. This is how the game resolves speed ties.
    // TypeScript source:
    // /**
    // 	 * A Fisher-Yates shuffle. This is how the game resolves speed ties.
    // 	 *
    // 	 * At least according to V4 in
    // 	 * https://github.com/smogon/pokemon-showdown/issues/1157#issuecomment-214454873
    // 	 */
    // 	shuffle<T>(items: T[], start = 0, end: number = items.length) {
    // 		while (start < end - 1) {
    // 			const nextIndex = this.random(start, end);
    // 			if (start !== nextIndex) {
    // 				[items[start], items[nextIndex]] = [items[nextIndex], items[start]];
    // 			}
    // 			start++;
    // 		}
    // 	}
    //
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        self.shuffle_range(items, 0, items.len());
    }

    /// Shuffle a range of the slice
    pub fn shuffle_range<T>(&mut self, items: &mut [T], start: usize, end: usize) {
        eprintln!("PRNG [shuffle]: shuffling array length={}, start={}, end={}", items.len(), start, end);
        let mut i = start;
        while i < end.saturating_sub(1) {
            let next_index = self.random_range(i as i32, end as i32) as usize;
            if i != next_index {
                items.swap(i, next_index);
            }
            i += 1;
        }
    }

    /// Generate a random seed (sodium format)
    //
    // 	static generateSeed(): PRNGSeed {
    // 		return PRNG.convertSeed(SodiumRNG.generateSeed());
    // 	}
    //
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

    /// Test that our Gen5 PRNG matches the JavaScript implementation exactly
    #[test]
    fn test_gen5_matches_javascript() {
        // Test case 1: Seed [0x1234, 0x5678, 0x9ABC, 0xDEF0]
        // Expected from JS: [85,74,94,18,28,70,9,0,38,13,6,60,1,55,88,37,21,97,15,12]
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0x1234, 0x5678, 0x9ABC, 0xDEF0])));
        let expected = [
            85, 74, 94, 18, 28, 70, 9, 0, 38, 13, 6, 60, 1, 55, 88, 37, 21, 97, 15, 12,
        ];
        for (i, &exp) in expected.iter().enumerate() {
            let val = prng.random_int(100);
            assert_eq!(val, exp, "Mismatch at index {} for seed1", i);
        }

        // Test case 2: Seed [1, 2, 3, 4]
        // Expected from JS: [47,88,66,13,84,57,95,38,77,82,10,22,6,84,31,85,14,93,14,1]
        let mut prng2 = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));
        let expected2 = [
            47, 88, 66, 13, 84, 57, 95, 38, 77, 82, 10, 22, 6, 84, 31, 85, 14, 93, 14, 1,
        ];
        for (i, &exp) in expected2.iter().enumerate() {
            let val = prng2.random_int(100);
            assert_eq!(val, exp, "Mismatch at index {} for seed2", i);
        }

        // Test case 3: Raw 32-bit values with seed [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]
        // Expected from JS: [2795446293,2744431784,3501875646,746470024,1664743198,3087483402,966965533,2002677306,2482317012,1229306196]
        let mut rng3 = Gen5RNG::new([0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
        let expected_raw: [u32; 10] = [
            2795446293, 2744431784, 3501875646, 746470024, 1664743198, 3087483402, 966965533,
            2002677306, 2482317012, 1229306196,
        ];
        for (i, &exp) in expected_raw.iter().enumerate() {
            let val = rng3.next();
            assert_eq!(val, exp, "Raw value mismatch at index {}", i);
        }
    }
}
