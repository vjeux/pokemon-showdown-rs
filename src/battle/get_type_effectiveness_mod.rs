use crate::*;

impl Battle {

    /// Calculate type effectiveness modifier as an integer
    /// Equivalent to runEffectiveness() in pokemon.js:1600 and getEffectiveness() in dex.js:219
    ///
    /// JavaScript (pokemon.js:1600-1621):
    ///   runEffectiveness(move) {
    ///     let totalTypeMod = 0;
    ///     if (this.terastallized && move.type === "Stellar") {
    ///       totalTypeMod = 1;
    ///     } else {
    ///       for (const type of this.getTypes()) {
    ///         let typeMod = this.battle.dex.getEffectiveness(move, type);
    ///         typeMod = this.battle.singleEvent("Effectiveness", move, null, this, type, move, typeMod);
    ///         totalTypeMod += this.battle.runEvent("Effectiveness", this, type, move, typeMod);
    ///       }
    ///     }
    ///     ...
    ///     return totalTypeMod;
    ///   }
    ///
    /// JavaScript (dex.js:219-242):
    ///   getEffectiveness(source, target) {
    ///     const sourceType = typeof source !== "string" ? source.type : source;
    ///     const targetTyping = target.getTypes?.() || target.types || target;
    ///     let totalTypeMod = 0;
    ///     if (Array.isArray(targetTyping)) {
    ///       for (const type of targetTyping) {
    ///         totalTypeMod += this.getEffectiveness(sourceType, type);
    ///       }
    ///       return totalTypeMod;
    ///     }
    ///     const typeData = this.types.get(targetTyping);
    ///     if (!typeData) return 0;
    ///     switch (typeData.damageTaken[sourceType]) {
    ///       case 1:
    ///         return 1;  // super-effective
    ///       case 2:
    ///         return -1; // resist
    ///       default:
    ///         return 0;
    ///     }
    ///   }
    pub fn get_type_effectiveness_mod(&self, attack_type: &str, defend_types: &[String]) -> i32 {
        let mut total_type_mod = 0;
        eprintln!("DEBUG get_type_effectiveness_mod: attack_type={}, defend_types={:?}", attack_type, defend_types);
        for defend_type in defend_types {
            let effectiveness = crate::data::typechart::get_effectiveness(attack_type, defend_type);
            eprintln!("DEBUG get_type_effectiveness_mod: {} vs {} = {}", attack_type, defend_type, effectiveness);
            // Convert float effectiveness to integer mod matching JavaScript:
            // 2.0 or higher = +1 (super-effective)
            // 0.5 or lower (but not 0) = -1 (not very effective)
            // 0.0 = 0 (immune, but handled as modifier)
            // 1.0 = 0 (neutral)
            if effectiveness >= 2.0 {
                total_type_mod += 1;
            } else if effectiveness > 0.0 && effectiveness <= 0.5 {
                total_type_mod -= 1;
            }
            // For immunity (0.0), we don't add anything (acts as 0)
        }
        eprintln!("DEBUG get_type_effectiveness_mod: total_type_mod={}", total_type_mod);
        total_type_mod
    }
}
