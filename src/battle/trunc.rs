use crate::*;

impl Battle {

    /// Truncate a number to an integer (equivalent to Math.trunc or this.trunc in JS)
    /// Note: JavaScript's dex.trunc() uses bitwise operations for 32-bit integer conversion
    /// Dex.trunc implementation from dex.ts:
    //
    // 	trunc(this: void, num: number, bits = 0) {
    // 		if (bits) return (num >>> 0) % (2 ** bits);
    // 		return num >>> 0;
    // 	}
    //
    /// Battle assigns: this.trunc = this.dex.trunc; (battle.ts:201)
    /// Rust uses num.trunc() as i32 which is functionally equivalent for positive values
    #[inline]
    pub fn trunc(&self, num: f64) -> i32 {
        num.trunc() as i32
    }
}
