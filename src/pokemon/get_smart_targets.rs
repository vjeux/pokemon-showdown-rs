use crate::*;

impl Pokemon {
    /// Get smart targets for move (Dragon Darts)
    ///
    /// JavaScript source:
    /// ```js
    /// /** Get targets for Dragon Darts */
    /// getSmartTargets(target: Pokemon, move: ActiveMove) {
    ///     const target2 = target.adjacentAllies()[0];
    ///     if (!target2 || target2 === this || !target2.hp) {
    ///         move.smartTarget = false;
    ///         return [target];
    ///     }
    ///     if (!target.hp) {
    ///         move.smartTarget = false;
    ///         return [target2];
    ///     }
    ///     return [target, target2];
    /// }
    /// ```
    ///
    /// In Rust, this is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on Pokemon within that Battle.
    /// Call as: Pokemon::get_smart_targets(battle, source_pos, target_pos)
    pub fn get_smart_targets(
        battle: &Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
    ) -> Vec<(usize, usize)> {
        // JS: const target2 = target.adjacentAllies()[0];
        // ✅ NOW IMPLEMENTED (Session 24 Part 45): Get adjacent ally of target
        let adjacent_allies = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return vec![target_pos],
            };
            target_pokemon.adjacent_allies(battle)
        };

        let target2_pos = adjacent_allies.first().copied();

        // JS: if (!target2 || target2 === this || !target2.hp) {
        // JS:     move.smartTarget = false;
        // JS:     return [target];
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 45): Check if adjacent ally is valid
        let target2_valid = if let Some(t2_pos) = target2_pos {
            // Check if target2 is not the source Pokemon
            if t2_pos == source_pos {
                false
            } else {
                // Check if target2 has HP
                let t2_pokemon = match battle.pokemon_at(t2_pos.0, t2_pos.1) {
                    Some(p) => p,
                    None => return vec![target_pos],
                };
                t2_pokemon.hp > 0
            }
        } else {
            false
        };

        if !target2_valid {
            // move.smartTarget = false (caller should handle)
            return vec![target_pos];
        }

        let target2_pos = target2_pos.unwrap(); // Safe because we checked target2_valid

        // JS: if (!target.hp) {
        // JS:     move.smartTarget = false;
        // JS:     return [target2];
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 45): If original target fainted, return ally only
        let target_hp = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return vec![target2_pos],
            };
            target_pokemon.hp
        };

        if target_hp == 0 {
            // move.smartTarget = false (caller should handle)
            return vec![target2_pos];
        }

        // JS: return [target, target2];
        // ✅ NOW IMPLEMENTED (Session 24 Part 45): Both targets valid, return both for Dragon Darts
        vec![target_pos, target2_pos]
    }
}
