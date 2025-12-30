use crate::*;

impl Battle {

    /// Truncate a number to an integer (equivalent to Math.trunc or this.trunc in JS)
    /// 1:1 port of Dex.trunc from dex.ts:
    //
    // 	trunc(this: void, num: number, bits = 0) {
    // 		if (bits) return (num >>> 0) % (2 ** bits);
    // 		return num >>> 0;
    // 	}
    //
    /// Battle assigns: this.trunc = this.dex.trunc; (battle.ts:201)
    /// JavaScript's >>> 0 converts to unsigned 32-bit integer
    /// Rust: num as u32 is equivalent to >>> 0
    #[inline]
    pub fn trunc(&self, num: f64, bits: Option<usize>) -> u32 {
        let truncated = num as u32;  // Equivalent to >>> 0
        if let Some(b) = bits {
            if b > 0 {
                return truncated % (1u32 << b);  // Equivalent to % (2 ** bits)
            }
        }
        truncated
    }
}
