use crate::*;

impl Battle {

    /// Helper to add heal log messages
    /// Rust helper method - JavaScript has this logic inline in heal() method (battle.ts:2246-2268)
    /// Extracted for borrow checker compatibility
    pub fn add_heal_log(
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
            "leechseed" | "rest" => {
                self.add_log("-heal", &[&target_str, &health_str, "[silent]"]);
            }
            "drain" => {
                if let Some(src) = source {
                    let src_str = format!("p{}a", src.0 + 1);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log(
                        "-heal",
                        &[&target_str, &health_str, "[from] drain", &of_str],
                    );
                } else {
                    self.add_log("-heal", &[&target_str, &health_str, "[from] drain"]);
                }
            }
            "wish" => {
                // Don't add any log for wish
            }
            "zpower" => {
                self.add_log("-heal", &[&target_str, &health_str, "[zeffect]"]);
            }
            "" => {
                // No effect - no log
            }
            _ => {
                // Default heal log
                // JS: if (effect.effectType === 'Move') { this.add('-heal', target, target.getHealth); }
                // JS: else if (source && source !== target) { this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`, `[of] ${source}`); }
                // JS: else { this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`); }

                // Check if effect type is Move
                let is_move = effect.is_some_and(|e| self.get_effect_type(e) == "Move");

                if is_move {
                    // Move effects: just show heal without [from] tag
                    self.add_log("-heal", &[&target_str, &health_str]);
                } else if let Some(src) = source {
                    // Non-move effects with source: show [from] effect [of] source
                    let src_str = format!("p{}a", src.0 + 1);
                    let from_str = format!("[from] {}", effect_id);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-heal", &[&target_str, &health_str, &from_str, &of_str]);
                } else {
                    // Non-move effects without source: show [from] effect
                    let from_str = format!("[from] {}", effect_id);
                    self.add_log("-heal", &[&target_str, &health_str, &from_str]);
                }
            }
        }
    }
}
