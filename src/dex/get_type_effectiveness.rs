use crate::*;

impl Dex {

    /// Get total type effectiveness against a Pokemon's types
    pub fn get_type_effectiveness(&self, attack_type: &str, defend_types: &[String]) -> f64 {
        let mut mult = 1.0;
        for defend_type in defend_types {
            mult *= self.get_effectiveness(attack_type, defend_type);
        }
        mult
    }
}
