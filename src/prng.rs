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
/// JavaScript equivalent: PRNGSeed (sim/prng.ts)
pub enum PRNGSeed {
    /// Sodium/ChaCha20-based RNG seed (hex string)
    Sodium(String),
    /// Gen 5 RNG seed (4 values, can be > 16-bit initially like JavaScript)
    /// JavaScript allows values > 65535 and only masks during multiplyAdd operations
    Gen5([u32; 4]),
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
                u32::from_str_radix(&hex[0..4], 16).map_err(|e| e.to_string())?,
                u32::from_str_radix(&hex[4..8], 16).map_err(|e| e.to_string())?,
                u32::from_str_radix(&hex[8..12], 16).map_err(|e| e.to_string())?,
                u32::from_str_radix(&hex[12..16], 16).map_err(|e| e.to_string())?,
            ];
            Ok(PRNGSeed::Gen5(seed))
        } else if s
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            // Old format: comma-separated numbers (can be > 65535 to match JavaScript)
            let parts: Result<Vec<u32>, _> = s.split(',').map(|p| p.parse::<u32>()).collect();
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
/// JavaScript equivalent: Gen5RNG (sim/prng.ts)
/// 1 field in JavaScript
pub struct Gen5RNG {
    /// Current seed state (4 values, can be > 16-bit initially)
    /// JavaScript stores full values and masks during multiplyAdd operations
    /// JavaScript: seed: [number, number, number, number]
    seed: [u32; 4],
}

impl Gen5RNG {
    /// Create a new Gen5RNG with the given seed
    pub fn new(seed: [u32; 4]) -> Self {
        Self { seed }
    }

    /// Generate a random seed (uses u16 values for compatibility)
    pub fn generate_seed() -> [u32; 4] {
        let mut rng = rand::thread_rng();
        [
            rng.gen::<u16>() as u32,
            rng.gen::<u16>() as u32,
            rng.gen::<u16>() as u32,
            rng.gen::<u16>() as u32,
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
    /// Calculates `a * b + c` using the full input values (can be > 16-bit)
    /// Output values are always masked to 16 bits to match JavaScript behavior
    fn multiply_add(a: [u32; 4], b: [u32; 4], c: [u32; 4]) -> [u32; 4] {
        let mut out = [0u32; 4];
        let mut carry: u64 = 0;

        for out_index in (0..4).rev() {
            for (local_b_index, &b_val) in b[out_index..].iter().enumerate() {
                let a_index = 3 - local_b_index;
                // Use full values in multiplication (matches JavaScript behavior)
                carry = carry.wrapping_add((a[a_index] as u64).wrapping_mul(b_val as u64));
            }
            carry = carry.wrapping_add(c[out_index] as u64);
            // Output is masked to 16 bits (matches JavaScript: out[outIndex] = carry & 0xFFFF)
            out[out_index] = (carry & 0xFFFF) as u32;
            // JavaScript's >>>= 16 truncates to 32 bits before shifting
            // This matches: carry = (carry as u32) >> 16
            carry = ((carry as u32) >> 16) as u64;
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
        const A: [u32; 4] = [0x5D58, 0x8B65, 0x6C07, 0x8965];
        const C: [u32; 4] = [0, 0, 0x26, 0x9EC3];
        self.seed = Self::multiply_add(self.seed, A, C);
    }
}

impl RNG for Gen5RNG {
    fn get_seed(&self) -> PRNGSeed {
        PRNGSeed::Gen5(self.seed)
    }

    fn next(&mut self) -> u32 {
        self.next_frame();
        // Use the upper 32 bits (seed values are already masked to 16 bits after first frame)
        ((self.seed[0]) << 16) | (self.seed[1])
    }
}

/// Sodium-compatible RNG using ChaCha20
/// This is a drop-in replacement for libsodium's randombytes_buf_deterministic
#[derive(Debug, Clone)]
/// JavaScript equivalent: SodiumRNG (sim/prng.ts)
/// 1 field in JavaScript
pub struct SodiumRNG {
    /// Seed bytes (32 bytes = 256 bits)
    /// JavaScript: seed: Uint8Array (32 bytes)
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
/// JavaScript equivalent: PRNG (sim/prng.ts)
/// 2 fields in JavaScript
pub struct PRNG {
    /// Starting seed value
    /// JavaScript: startingSeed: PRNGSeed
    pub starting_seed: PRNGSeed,
    /// RNG implementation (Gen5RNG or SodiumRNG)
    /// JavaScript: rng: Gen5RNG | SodiumRNG
    rng: PRNGImpl,
    // TODO: DELETE - Not in JavaScript PRNG (Rust-specific field for debugging)
    /// Call count for debugging PRNG calls
    pub call_count: usize,
}

impl Default for PRNG {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Internal RNG implementation enum
/// TODO: Not in JavaScript - Rust-specific enum for holding Gen5RNG or SodiumRNG
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
        Self {
            starting_seed,
            rng,
            call_count: 0,
        }
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
            call_count: self.call_count,
        }
    }

    /// Get the next random 32-bit number
    fn next_raw(&mut self) -> u32 {
        self.call_count += 1;
        let value = match &mut self.rng {
            PRNGImpl::Gen5(rng) => rng.next(),
            PRNGImpl::Sodium(rng) => rng.next(),
        };

        // Parse TRACE_PRNG environment variable for configurable tracing
        // Examples: TRACE_PRNG=1-5 (trace calls 1 through 5)
        //           TRACE_PRNG=10 (trace call 10)
        //           TRACE_PRNG=1,5,10 (trace calls 1, 5, and 10)
        if let Ok(trace_spec) = std::env::var("TRACE_PRNG") {
            let should_trace = trace_spec.split(',').any(|part| {
                if part.contains('-') {
                    // Range like "1-5"
                    let parts: Vec<&str> = part.split('-').collect();
                    if parts.len() == 2 {
                        if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            return self.call_count >= start && self.call_count <= end;
                        }
                    }
                    false
                } else {
                    // Single number like "10"
                    part.parse::<usize>().ok() == Some(self.call_count)
                }
            });

            if should_trace {
                let bt = std::backtrace::Backtrace::force_capture();
                let bt_str = format!("{}", bt);

                // Extract compact function names from backtrace
                let frames: Vec<String> = bt_str.lines()
                    .filter(|line| {
                        // Filter out noise: prng module internals, std library, runtime
                        let trimmed = line.trim();
                        !trimmed.contains("prng::PRNG::next_raw") &&
                        !trimmed.contains("prng::PRNG::random") &&
                        !trimmed.contains("std::") &&
                        !trimmed.contains("core::") &&
                        !trimmed.contains("rustc") &&
                        !trimmed.contains("alloc::") &&
                        trimmed.chars().next().map_or(false, |c| c.is_digit(10))
                    })
                    .take(4)
                    .filter_map(|line| {
                        // Extract useful part: "10: pokemon_showdown::battle::random_chance<impl...>::random_chance"
                        // Keep last 2-3 segments for readability
                        if let Some(colon_pos) = line.find(':') {
                            let func_part = &line[colon_pos + 1..].trim();
                            let parts: Vec<&str> = func_part.split("::").collect();
                            if parts.len() >= 2 {
                                // Take last 2 parts or module::function
                                let relevant = parts[parts.len().saturating_sub(2)..].join("::");
                                return Some(relevant.split('<').next().unwrap_or(&relevant).to_string());
                            }
                        }
                        None
                    })
                    .collect();

                let _compact = frames.join(" <- ");
                debug_elog!("[Rust PRNG #{}] raw={}", self.call_count, value);
                debug_elog!("  Stack: {}", _compact);
            }
        }

        value
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
        // Log parameters before calling next_raw() for TRACE_PRNG
        if let Ok(trace_spec) = std::env::var("TRACE_PRNG") {
            // Check if we should trace the next call
            let next_call = self.call_count + 1;
            let should_trace = trace_spec.split(',').any(|part| {
                if part.contains('-') {
                    let parts: Vec<&str> = part.split('-').collect();
                    if parts.len() == 2 {
                        if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            return next_call >= start && next_call <= end;
                        }
                    }
                    false
                } else {
                    part.parse::<usize>().ok() == Some(next_call)
                }
            });

            if should_trace {
                let _from_val = match from {
                    Some(f) => format!("{}", f),
                    None => "None".to_string(),
                };
                let _to_val = match to {
                    Some(t) => format!("{}", t),
                    None => "None".to_string(),
                };
                debug_elog!("  [random(from={}, to={})]", _from_val, _to_val);
            }
        }

        let result = self.next_raw();

        let value = match (from, to) {
            (None, None) => (result as f64) / (2f64.powi(32)),
            (Some(n), None) => {
                ((result as f64) * (n as f64) / (2f64.powi(32))).floor()
            }
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
    pub fn random_chance(&mut self, numerator: f64, denominator: i32) -> bool {
        // JS: return this.random(denominator) < numerator;
        // NOTE: In JS, numerator can be a float (e.g., 67.5 for accuracy checks with boosts).
        // The comparison `roll < 67.5` where roll=67 returns true (hit) in JS.
        // We must use f64 comparison to match this behavior.
        let roll = self.random(Some(denominator), None);
        let result = (roll as f64) < numerator;
        debug_elog!("[RANDOM_CHANCE] numerator={}, denominator={}, roll={}, result={}", numerator, denominator, roll, result);
        result
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
