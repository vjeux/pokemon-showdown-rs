use crate::*;

impl Battle {

    /// Apply a modifier to a value
    /// Equivalent to battle.ts modify(value, numerator, denominator)
    ///
    //
    // 	modify(value: number, numerator: number | number[], denominator = 1) {
    // 		// You can also use:
    // 		// modify(value, [numerator, denominator])
    // 		// modify(value, fraction) - assuming you trust JavaScript's floating-point handler
    // 		if (Array.isArray(numerator)) {
    // 			denominator = numerator[1];
    // 			numerator = numerator[0];
    // 		}
    // 		const tr = this.trunc;
    // 		const modifier = tr(numerator * 4096 / denominator);
    // 		return tr((tr(value * modifier) + 2048 - 1) / 4096);
    // 	}
    //
    pub fn modify(&self, value: i32, numerator: i32, denominator: i32) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc((numerator * 4096) as f64 / denominator as f64);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }
}
