use crate::*;

impl Battle {

    /// Check if an ability is being suppressed (Mold Breaker, etc.)
    /// Equivalent to battle.ts suppressingAbility()
    //
    // 	suppressingAbility(target?: Pokemon) {
    // 		return this.activePokemon && this.activePokemon.isActive && (this.activePokemon !== target || this.gen < 8) &&
    // 			this.activeMove && this.activeMove.ignoreAbility && !target?.hasItem('Ability Shield');
    // 	}
    //
    pub fn suppressing_ability(&self, target: Option<(usize, usize)>) -> bool {
        // JS: this.activePokemon && this.activePokemon.isActive
        if let Some((active_side, active_idx)) = self.active_pokemon {
            if let Some(side) = self.sides.get(active_side) {
                if let Some(pokemon) = side.pokemon.get(active_idx) {
                    if !pokemon.is_active {
                        return false;
                    }

                    // JS: (this.activePokemon !== target || this.gen < 8)
                    // In Gen 8+, can't suppress your own ability
                    if self.gen >= 8 {
                        if let Some(t) = target {
                            if t == (active_side, active_idx) {
                                return false;
                            }
                        }
                    }

                    // JS: this.activeMove && this.activeMove.ignoreAbility
                    if let Some(ref active_move) = self.active_move {
                        if !active_move.borrow().ignore_ability {
                            return false;
                        }

                        // JS: !target?.hasItem('Ability Shield')
                        if let Some((target_side, target_idx)) = target {
                            if let Some(tside) = self.sides.get(target_side) {
                                if let Some(tpoke) = tside.pokemon.get(target_idx) {
                                    if tpoke.item.as_str() == "abilityshield" {
                                        return false;
                                    }
                                }
                            }
                        }

                        return true;
                    }
                }
            }
        }
        false
    }
}
