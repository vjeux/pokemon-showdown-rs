use crate::*;
use crate::battle::SpreadMoveHitResult;

impl Battle {

    /// Spread move hit - handles individual target hit processing
    /// Equivalent to spreadMoveHit() in battle-actions.ts:1043
    ///
    /// Returns (damages, targets) where damages[i] corresponds to targets[i]
    pub fn spread_move_hit(
        &mut self,
        targets: &[Option<(usize, usize)>],
        source_pos: (usize, usize),
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) -> SpreadMoveHitResult {
        let mut damages: Vec<Option<i32>> = vec![Some(0); targets.len()];
        let mut final_targets = targets.to_vec();

        // Get move data
        let move_data = match self.dex.get_move(move_id.as_str()) {
            Some(m) => m.clone(),
            None => return (damages, final_targets),
        };

        // Step 1: TryHit event
        if !is_secondary && !is_self {
            for (i, &target) in targets.iter().enumerate() {
                if let Some(target_pos) = target {
                    // JavaScript: hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
                    let hit_result = self.single_event(
                        "TryHit",
                        move_id,
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                    );

                    // If TryHit returns false, the move fails
                    if matches!(hit_result, crate::event::EventResult::Boolean(false)) {
                        damages[i] = None;
                        final_targets[i] = None;
                    }
                }
            }
        }

        // Step 1.5: Accuracy check
        // JavaScript: accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
        if !is_secondary && !is_self {
            for (i, &target) in targets.iter().enumerate() {
                if let Some(target_pos) = target {
                    // Skip if already failed TryHit
                    if damages[i].is_none() {
                        continue;
                    }

                    // Get base accuracy from move
                    let base_accuracy = match self.dex.get_move(move_id.as_str()) {
                        Some(m) => {
                            match m.accuracy {
                            crate::dex::Accuracy::Percent(p) => p,
                            crate::dex::Accuracy::AlwaysHits => {
                                // Always hits, skip accuracy check
                                continue;
                            }
                        }},
                        None => {
                            continue;
                        }
                    };

                    // Trigger Accuracy event to allow abilities/items to modify accuracy
                    // JavaScript: accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
                    let mut accuracy = base_accuracy;
                    if let Some(modified_acc) = self.run_event(
                        "Accuracy",
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                        Some(accuracy),
                    ) {
                        accuracy = modified_acc;
                    }

                    // Check if move hits based on accuracy
                    // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
                    // Always call randomChance to consume PRNG value, even if accuracy is 100
                    if !self.random_chance(accuracy, 100) {
                        // Move missed
                        damages[i] = None;
                        final_targets[i] = None;
                        // TODO: Add miss message: this.battle.add('-miss', pokemon, target);
                    }
                }
            }
        }

        // Step 2: Get damage for each target
        // JavaScript: damage = this.getSpreadDamage(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
        // IMPORTANT: Pass final_targets (which has None for misses), not targets
        damages = self.get_spread_damage(
            &damages,
            &final_targets,
            source_pos,
            move_id,
            is_secondary,
            is_self,
        );

        // Step 3: Apply damage using spread_damage
        let damage_vals: Vec<Option<i32>> = damages.clone();
        let applied_damages = self.spread_damage(
            &damage_vals,
            &final_targets,
            Some(source_pos),
            Some(move_id),
            false,
        );

        for (i, &applied) in applied_damages.iter().enumerate() {
            damages[i] = applied;
            if applied.is_none() || applied == Some(0) {
                // Don't clear target on 0 damage - that's still a hit
                // Only clear on None (failed)
                if applied.is_none() {
                    final_targets[i] = None;
                }
            }
        }

        // Step 3.5: Trigger Hit events for successful hits
        // JavaScript: this.battle.runEvent('Hit', target, pokemon, move)
        for (i, &target) in final_targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // Only trigger Hit if we actually dealt damage or the move succeeded
                if damages[i].is_some() {
                    self.run_event(
                        "Hit",
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                        None,
                    );
                }
            }
        }

        // JS: this.battle.eachEvent('Update'); (line 886 - after hit loop)
        // Call eachEvent("Update") after all hits processed
        if !is_secondary && !is_self {
            self.each_event("Update", None);
        }

        // Step 4: Run move effects (boosts, status, healing, etc.)
        damages = self.run_move_effects(
            &damages,
            &final_targets,
            source_pos,
            &move_data,
            is_secondary,
            is_self,
        );

        (damages, final_targets)
    }
}
