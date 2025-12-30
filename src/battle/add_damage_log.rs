use crate::*;

impl Battle {

    /// Helper to add damage log messages
    /// Matches JavaScript battle.ts:2088-2112
    /// Rust helper method - JavaScript has this logic inline in spreadDamage()
    /// This method extracts the damage logging logic for code organization
    /// Handles special cases: partiallytrapped, powder, confused, and default damage logs
    pub fn add_damage_log(
        &mut self,
        target: (usize, usize),
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
    ) {
        let (side_idx, poke_idx) = target;

        // Get target health string
        let health_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}/{}", pokemon.hp, pokemon.maxhp)
            } else {
                return;
            }
        } else {
            return;
        };

        let target_str = format!("p{}a", side_idx + 1);
        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Special case handling
        match effect_id {
            "partiallytrapped" => {
                // Get source effect name from volatiles
                // JS: '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname
                let source_effect_name = if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        let trap_id = ID::new("partiallytrapped");
                        if let Some(trap_state) = pokemon.volatiles.get(&trap_id) {
                            // Extract sourceEffect.fullname from data HashMap
                            if let Some(source_effect) = trap_state.data.get("sourceEffect") {
                                source_effect
                                    .get("fullname")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("partiallytrapped")
                            } else {
                                "partiallytrapped"
                            }
                        } else {
                            "partiallytrapped"
                        }
                    } else {
                        "partiallytrapped"
                    }
                } else {
                    "partiallytrapped"
                };

                let from_str = format!("[from] {}", source_effect_name);
                self.add_log(
                    "-damage",
                    &[&target_str, &health_str, &from_str, "[partiallytrapped]"],
                );
            }
            "powder" => {
                self.add_log("-damage", &[&target_str, &health_str, "[silent]"]);
            }
            "confused" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] confusion"]);
            }
            _ => {
                // Default damage log
                if effect.is_none() {
                    self.add_log("-damage", &[&target_str, &health_str]);
                } else if let Some(src) = source {
                    let src_str = format!("p{}a", src.0 + 1);
                    let from_str = format!("[from] {}", effect_id);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-damage", &[&target_str, &health_str, &from_str, &of_str]);
                } else {
                    let from_str = format!("[from] {}", effect_id);
                    self.add_log("-damage", &[&target_str, &health_str, &from_str]);
                }
            }
        }
    }
}
