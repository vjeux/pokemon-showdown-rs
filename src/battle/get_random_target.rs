use crate::*;

impl Battle {

    /// Get random target for a move
    /// Equivalent to battle.ts getRandomTarget()
    ///
    //
    // 	getRandomTarget(pokemon: Pokemon, move: string | Move) {
    // 		// A move was used without a chosen target
    //
    // 		// For instance: Metronome chooses Ice Beam. Since the user didn't
    // 		// choose a target when choosing Metronome, Ice Beam's target must
    // 		// be chosen randomly.
    //
    // 		// The target is chosen randomly from possible targets, EXCEPT that
    // 		// moves that can target either allies or foes will only target foes
    // 		// when used without an explicit target.
    //
    // 		move = this.dex.moves.get(move);
    // 		if (['self', 'all', 'allySide', 'allyTeam', 'adjacentAllyOrSelf'].includes(move.target)) {
    // 			return pokemon;
    // 		} else if (move.target === 'adjacentAlly') {
    // 			if (this.gameType === 'singles') return null;
    // 			const adjacentAllies = pokemon.adjacentAllies();
    // 			return adjacentAllies.length ? this.sample(adjacentAllies) : null;
    // 		}
    // 		if (this.gameType === 'singles') return pokemon.side.foe.active[0];
    //
    // 		if (this.activePerHalf > 2) {
    // 			if (move.target === 'adjacentFoe' || move.target === 'normal' || move.target === 'randomNormal') {
    // 				// even if a move can target an ally, auto-resolution will never make it target an ally
    // 				// i.e. if both your opponents faint before you use Flamethrower, it will fail instead of targeting your ally
    // 				const adjacentFoes = pokemon.adjacentFoes();
    // 				if (adjacentFoes.length) return this.sample(adjacentFoes);
    // 				// no valid target at all, return slot directly across for any possible redirection
    // 				return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
    // 			}
    // 		}
    // 		return pokemon.side.randomFoe() || pokemon.side.foe.active[0];
    // 	}
    //
    pub fn get_random_target(
        &mut self,
        user_side: usize,
        user_idx: usize,
        move_target: &str,
    ) -> Option<(usize, usize)> {
        // JS: if (['self', 'all', 'allySide', 'allyTeam', 'adjacentAllyOrSelf'].includes(move.target)) return pokemon;
        if move_target == "Self"
            || move_target == "All"
            || move_target == "AllySide"
            || move_target == "AllyTeam"
            || move_target == "AdjacentAllyOrSelf"
        {
            return Some((user_side, user_idx));
        }

        // JS: else if (move.target === 'adjacentAlly') {
        // JS:     if (this.gameType === 'singles') return null;
        // JS:     const adjacentAllies = pokemon.adjacentAllies();
        // JS:     return adjacentAllies.length ? this.sample(adjacentAllies) : null;
        // JS: }
        if move_target == "AdjacentAlly" {
            if self.game_type == GameType::Singles {
                return None;
            }
            let adjacent_allies = if let Some(pokemon) = self.sides.get(user_side)
                .and_then(|s| s.pokemon.get(user_idx))
            {
                pokemon.adjacent_allies(self)
            } else {
                Vec::new()
            };
            if !adjacent_allies.is_empty() {
                return self.sample(&adjacent_allies).copied();
            }
            return None;
        }

        // JS: if (this.gameType === 'singles') return pokemon.side.foe.active[0];
        if self.game_type == GameType::Singles {
            let foe_side = if user_side == 0 { 1 } else { 0 };
            if foe_side < self.sides.len() {
                if let Some(side) = self.sides.get(foe_side) {
                    if let Some(Some(poke_idx)) = side.active.first() {
                        return Some((foe_side, *poke_idx));
                    }
                }
            }
            return None;
        }

        // JS: if (this.activePerHalf > 2) {
        if self.active_per_half > 2 {
            // JS: if (move.target === 'adjacentFoe' || move.target === 'normal' || move.target === 'randomNormal') {
            if move_target == "AdjacentFoe"
                || move_target == "Normal"
                || move_target == "RandomNormal"
            {
                // JS: const adjacentFoes = pokemon.adjacentFoes();
                // JS: if (adjacentFoes.length) return this.sample(adjacentFoes);
                let adjacent_foes = if let Some(pokemon) = self.sides.get(user_side)
                    .and_then(|s| s.pokemon.get(user_idx))
                {
                    pokemon.adjacent_foes(self)
                } else {
                    Vec::new()
                };
                if !adjacent_foes.is_empty() {
                    return self.sample(&adjacent_foes).copied();
                }

                // JS: return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
                // No valid adjacent target, return slot directly across for any possible redirection
                let foe_side = if user_side == 0 { 1 } else { 0 };
                if foe_side < self.sides.len() {
                    if let Some(side) = self.sides.get(foe_side) {
                        let position = if let Some(user_side_ref) = self.sides.get(user_side) {
                            if let Some(pokemon) = user_side_ref.pokemon.get(user_idx) {
                                pokemon.position
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        let target_slot =
                            side.active.len().saturating_sub(1).saturating_sub(position);
                        if let Some(Some(poke_idx)) = side.active.get(target_slot) {
                            return Some((foe_side, *poke_idx));
                        }
                    }
                }
            }
        }

        // JS: return pokemon.side.randomFoe() || pokemon.side.foe.active[0];
        let foe_side = if user_side == 0 { 1 } else { 0 };
        if foe_side < self.sides.len() {
            // Try to get a random active foe
            let valid_targets: Vec<usize> = self.sides[foe_side]
                .pokemon
                .iter()
                .enumerate()
                .filter(|(_, p)| p.is_active && !p.is_fainted())
                .map(|(idx, _)| idx)
                .collect();

            if !valid_targets.is_empty() {
                let random_idx = self.random(valid_targets.len() as i32) as usize;
                return Some((foe_side, valid_targets[random_idx]));
            }

            // Fallback: return first active
            if let Some(Some(poke_idx)) = self.sides[foe_side].active.first() {
                return Some((foe_side, *poke_idx));
            }
        }

        None
    }
}
