use crate::*;

impl Dex {

    /// Get type effectiveness for a defensive type against an attacking type
    /// Equivalent to this.dex.types.get(typeName).damageTaken[attackType] in conversion2.ts
    /// Returns: 0 = normal, 1 = super effective, 2 = not very effective, 3 = immune
    pub fn get_type_damage_taken(&self, defending_type: &str, attacking_type: &str) -> u8 {
        self.types
            .get(defending_type)
            .and_then(|type_data| type_data.damage_taken.get(attacking_type))
            .copied()
            .unwrap_or(0) // Default to normal effectiveness
    }
}
