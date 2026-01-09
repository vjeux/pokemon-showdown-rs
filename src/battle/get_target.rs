use crate::*;

impl Battle {

    /// Get target for a move
    /// Equivalent to battle.ts getTarget()
    ///
    //
    // 	getTarget(pokemon: Pokemon, move: string | Move, targetLoc: number, originalTarget?: Pokemon) {
    // 		move = this.dex.moves.get(move);
    //
    // 		let tracksTarget = move.tracksTarget;
    // 		// Stalwart sets trackTarget in ModifyMove, but ModifyMove happens after getTarget, so
    // 		// we need to manually check for Stalwart here
    // 		if (pokemon.hasAbility(['stalwart', 'propellertail'])) tracksTarget = true;
    // 		if (tracksTarget && originalTarget?.isActive) {
    // 			// smart-tracking move's original target is on the field: target it
    // 			return originalTarget;
    // 		}
    //
    // 		// banning Dragon Darts from directly targeting itself is done in side.ts, but
    // 		// Dragon Darts can target itself if Ally Switch is used afterwards
    // 		if (move.smartTarget) {
    // 			const curTarget = pokemon.getAtLoc(targetLoc);
    // 			return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
    // 		}
    //
    // 		// Fails if the target is the user and the move can't target its own position
    // 		const selfLoc = pokemon.getLocOf(pokemon);
    // 		if (
    // 			['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
    // 			!pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']
    // 		) {
    // 			return move.flags['futuremove'] ? pokemon : null;
    // 		}
    // 		if (move.target !== 'randomNormal' && this.validTargetLoc(targetLoc, pokemon, move.target)) {
    // 			const target = pokemon.getAtLoc(targetLoc);
    // 			if (target?.fainted) {
    // 				if (this.gameType === 'freeforall') {
    // 					// Target is a fainted opponent in a free-for-all battle; attack shouldn't retarget
    // 					return target;
    // 				}
    // 				if (target.isAlly(pokemon)) {
    // 					if (move.target === 'adjacentAllyOrSelf' && this.gen !== 5) {
    // 						return pokemon;
    // 					}
    // 					// Target is a fainted ally: attack shouldn't retarget
    // 					return target;
    // 				}
    // 			}
    // 			if (target && !target.fainted) {
    // 				// Target is unfainted: use selected target location
    // 				return target;
    // 			}
    //
    // 			// Chosen target not valid,
    // 			// retarget randomly with getRandomTarget
    // 		}
    // 		return this.getRandomTarget(pokemon, move);
    // 	}
    //
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn get_target(
        &mut self,
        user: (usize, usize),
        move_id: &ID,
        target_loc: i8,
        original_target: Option<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        // JS: move = this.dex.moves.get(move);
        let move_def = self.dex.moves().get(move_id.as_str())?;
        let target_type = format!("{:?}", move_def.target);

        // JS: let tracksTarget = move.tracksTarget;
        // JS: if (pokemon.hasAbility(['stalwart', 'propellertail'])) tracksTarget = true;
        let (user_side, user_idx) = user;
        let mut tracks_target = move_def.tracks_target.unwrap_or(false);

        // Check if Pokemon has Stalwart or Propeller Tail abilities
        if let Some(side) = self.sides.get(user_side) {
            if let Some(pokemon) = side.pokemon.get(user_idx) {
                let ability = pokemon.ability.as_str();
                if ability == "stalwart" || ability == "propellertail" {
                    tracks_target = true;
                }
            }
        }

        // JS: if (tracksTarget && originalTarget?.isActive) return originalTarget;
        if tracks_target {
            if let Some((target_side, target_idx)) = original_target {
                if let Some(side) = self.sides.get(target_side) {
                    if let Some(pokemon) = side.pokemon.get(target_idx) {
                        if pokemon.is_active {
                            return Some((target_side, target_idx));
                        }
                    }
                }
            }
        }

        // JS: if (move.smartTarget) {
        // JS:     const curTarget = pokemon.getAtLoc(targetLoc);
        // JS:     return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
        // JS: }
        if move_def.smart_target.unwrap_or(false) {
            // Try to get the Pokemon at target location
            if let Some(cur_target) = self.get_at_loc(user, target_loc) {
                // If target exists and is not fainted, use it
                if !self.is_pokemon_fainted(cur_target) {
                    return Some(cur_target);
                }
            }
            // Otherwise, get a random target
            return self.get_random_target(user.0, user.1, &target_type);
        }

        // JS: const selfLoc = pokemon.getLocOf(pokemon);
        // JS: if (
        // JS:     ['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
        // JS:     !pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']
        // JS: ) {
        // JS:     return move.flags['futuremove'] ? pokemon : null;
        // JS: }
        // Fails if the target is the user and the move can't target its own position
        let self_loc = if let Some(user_pokemon) = self.sides.get(user.0).and_then(|s| s.pokemon.get(user.1)) {
            user_pokemon.get_loc_of(user.0, user.1, self.active_per_half) as i32
        } else {
            0
        };
        if (target_type == "AdjacentAlly" || target_type == "Any" || target_type == "Normal")
            && target_loc as i32 == self_loc
        {
            // Check if Pokemon has volatiles that allow self-targeting
            let has_self_targeting_volatile = if let Some(side) = self.sides.get(user_side) {
                if let Some(pokemon) = side.pokemon.get(user_idx) {
                    pokemon.volatiles.contains_key(&ID::new("twoturnmove"))
                        || pokemon.volatiles.contains_key(&ID::new("iceball"))
                        || pokemon.volatiles.contains_key(&ID::new("rollout"))
                } else {
                    false
                }
            } else {
                false
            };

            if !has_self_targeting_volatile {
                // If move has futuremove flag, return user (self), otherwise return None
                let has_futuremove = move_def
                    .flags
                    .get("futuremove")
                    .unwrap_or(false);
                return if has_futuremove { Some(user) } else { None };
            }
        }

        // JS: if (move.target !== 'randomNormal' && this.validTargetLoc(targetLoc, pokemon, move.target)) {
        if target_type != "RandomNormal"
            && self.valid_target_loc(target_loc as i32, user, &target_type)
        {
            // JS: const target = pokemon.getAtLoc(targetLoc);
            if let Some(target) = self.get_at_loc(user, target_loc) {
                let (target_side, target_idx) = target;
                if let Some(side) = self.sides.get(target_side) {
                    if let Some(pokemon) = side.pokemon.get(target_idx) {
                        // JS: if (target?.fainted) {
                        if pokemon.fainted {
                            // JS: if (this.gameType === 'freeforall') return target;
                            if self.game_type == GameType::FreeForAll {
                                return Some(target);
                            }
                            // JS: if (target.isAlly(pokemon)) {
                            // JS:     if (move.target === 'adjacentAllyOrSelf' && this.gen !== 5) return pokemon;
                            // JS:     return target;
                            // JS: }
                            if self.is_ally(target, user) {
                                if target_type == "AdjacentAllyOrSelf" && self.gen != 5 {
                                    return Some(user);
                                }
                                // Target is a fainted ally: attack shouldn't retarget
                                return Some(target);
                            }
                        }
                        // JS: if (target && !target.fainted) return target;
                        if !pokemon.fainted {
                            return Some(target);
                        }
                    }
                }
            }
        }

        // JS: return this.getRandomTarget(pokemon, move);
        self.get_random_target(user_side, user_idx, &target_type)
    }
}
