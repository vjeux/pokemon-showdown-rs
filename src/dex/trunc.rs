use crate::*;

impl Dex {

    /// Truncate a number into an unsigned 32-bit integer
    /// 1:1 port of trunc() from dex.ts
    // TypeScript source:
    // /**
    // 	 * Truncate a number into an unsigned 32-bit integer, for
    // 	 * compatibility with the cartridge games' math systems.
    // 	 */
    // 	trunc(this: void, num: number, bits = 0) {
    // 		if (bits) return (num >>> 0) % (2 ** bits);
    // 		return num >>> 0;
    // 	}
    //
    /// JavaScript's >>> 0 converts to unsigned 32-bit integer
    /// Rust: num as u32 is equivalent to >>> 0
    pub fn trunc(num: f64, bits: usize) -> u32 {
        let truncated = num as u32;  // Equivalent to >>> 0
        if bits > 0 {
            truncated % (1u32 << bits)  // Equivalent to % (2 ** bits)
        } else {
            truncated
        }
    }
}
