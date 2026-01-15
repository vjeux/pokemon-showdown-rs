// NOTE: This method is NOT in JavaScript - Rust-specific implementation
//
// JavaScript's Move class in dex-moves.ts defaults critRatio to 1:
//   this.critRatio = Number(data.critRatio) || 1;
//
// This means moves without an explicit critRatio have critRatio = 1,
// which gives a 1/24 (critMult[1] = 24) crit chance for Gen 7+.
//
// A crit_ratio of 0 would skip the crit roll entirely.
// A crit_ratio of 1 means critMult[1] = 24, so 1/24 crit chance.

pub fn default_crit_ratio() -> i32 {
    1
}
