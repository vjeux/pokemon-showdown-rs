use crate::*;

impl Dex {

    /// Truncate a number (floor towards zero)
    /// Equivalent to trunc() in dex.ts
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
    pub fn trunc(num: f64, bits: i32) -> i32 {
        if bits == 0 {
            if num > 0.0 {
                num.floor() as i32
            } else {
                num.ceil() as i32
            }
        } else {
            let mult = 1 << bits;
            ((num * mult as f64) as i32) / mult
        }
    }
}
