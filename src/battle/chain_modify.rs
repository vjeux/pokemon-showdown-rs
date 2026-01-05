use crate::*;

impl Battle {

    /// Chain modify event modifier
    /// Equivalent to battle.ts chainModify() (battle.ts:2291-2300)
    ///
    // 	chainModify(numerator: number | number[], denominator = 1) {
    // 		const previousMod = this.trunc(this.event.modifier * 4096);
    //
    // 		if (Array.isArray(numerator)) {
    // 			denominator = numerator[1];
    // 			numerator = numerator[0];
    // 		}
    // 		const nextMod = this.trunc(numerator * 4096 / denominator);
    // 		this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
    // 	}
    //
    /// Chain modify the event modifier using a simple multiplier
    /// Most common usage: battle.chain_modify(2.0) to double the value
    pub fn chain_modify(&mut self, multiplier: f32) -> i32 {
        eprintln!("[CHAIN_MODIFY] Input multiplier: {}", multiplier);
        // Convert multiplier to numerator/denominator (e.g., 2.0 -> 2/1, 1.5 -> 3/2)
        let numerator = (multiplier * 4096.0) as i32;
        let denominator = 4096;
        let result = self.chain_modify_fraction(numerator, denominator);
        eprintln!("[CHAIN_MODIFY] Result modifier: {}", result);
        result
    }
}
