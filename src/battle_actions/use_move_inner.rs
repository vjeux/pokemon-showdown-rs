use crate::*;

use crate::battle_actions::ZPowerResult;

/// Inner implementation of useMove - handles the actual move execution
/// Equivalent to battle-actions.ts useMoveInner() (lines 380-543)
// 	useMoveInner(
// 		moveOrMoveName: Move | string, pokemon: Pokemon, options?: {
// 			target?: Pokemon | null, sourceEffect?: Effect | null,
// 			zMove?: string, maxMove?: string,
// 		},
// 	) {
// 		let target = options?.target;
// 		let sourceEffect = options?.sourceEffect;
// 		const zMove = options?.zMove;
// 		const maxMove = options?.maxMove;
// 		if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
// 		if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
//
// 		let move = this.dex.getActiveMove(moveOrMoveName);
// 		pokemon.lastMoveUsed = move;
// 		if (move.id === 'weatherball' && zMove) {
// 			// Z-Weather Ball only changes types if it's used directly,
// 			// not if it's called by Z-Sleep Talk or something.
// 			this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 			if (move.type !== 'Normal') sourceEffect = move;
// 		}
// 		if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
// 			move = this.getActiveZMove(move, pokemon);
// 		}
// 		if (maxMove && move.category !== 'Status') {
// 			// Max move outcome is dependent on the move type after type modifications from ability and the move itself
// 			this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 			this.battle.runEvent('ModifyType', pokemon, target, move, move);
// 		}
// 		if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
// 			move = this.getActiveMaxMove(move, pokemon);
// 		}
//
// 		if (this.battle.activeMove) {
// 			move.priority = this.battle.activeMove.priority;
// 			if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
// 		}
// 		const baseTarget = move.target;
// 		let targetRelayVar = { target };
// 		targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
// 		if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
// 		if (target === undefined) target = this.battle.getRandomTarget(pokemon, move);
// 		if (move.target === 'self' || move.target === 'allies') {
// 			target = pokemon;
// 		}
// 		if (sourceEffect) {
// 			move.sourceEffect = sourceEffect.id;
// 			move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
// 		}
// 		let moveResult = false;
//
// 		this.battle.setActiveMove(move, pokemon, target);
//
// 		this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 		this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
// 		if (baseTarget !== move.target) {
// 			// Target changed in ModifyMove, so we must adjust it here
// 			// Adjust before the next event so the correct target is passed to the
// 			// event
// 			target = this.battle.getRandomTarget(pokemon, move);
// 		}
// 		move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
// 		move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
// 		if (baseTarget !== move.target) {
// 			// Adjust again
// 			target = this.battle.getRandomTarget(pokemon, move);
// 		}
// 		if (!move || pokemon.fainted) {
// 			return false;
// 		}
//
// 		let attrs = '';
//
// 		let movename = move.name;
// 		if (move.id === 'hiddenpower') movename = 'Hidden Power';
// 		if (sourceEffect) attrs += `|[from] ${sourceEffect.fullname}`;
// 		if (zMove && move.isZ === true) {
// 			attrs = `|[anim]${movename}${attrs}`;
// 			movename = `Z-${movename}`;
// 		}
// 		this.battle.addMove('move', pokemon, movename, `${target}${attrs}`);
//
// 		if (zMove) this.runZPower(move, pokemon);
//
// 		if (!target) {
// 			this.battle.attrLastMove('[notarget]');
// 			this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
// 			return false;
// 		}
//
// 		const { targets, pressureTargets } = pokemon.getMoveTargets(move, target);
// 		if (targets.length) {
// 			target = targets[targets.length - 1]; // in case of redirection
// 		}
//
// 		const callerMoveForPressure = sourceEffect && (sourceEffect as ActiveMove).pp ? sourceEffect as ActiveMove : null;
// 		if (!sourceEffect || callerMoveForPressure || sourceEffect.id === 'pursuit') {
// 			let extraPP = 0;
// 			for (const source of pressureTargets) {
// 				const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
// 				if (ppDrop !== true) {
// 					extraPP += ppDrop || 0;
// 				}
// 			}
// 			if (extraPP > 0) {
// 				pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
// 			}
// 		}
//
// 		if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
// 			!this.battle.runEvent('TryMove', pokemon, target, move)) {
// 			move.mindBlownRecoil = false;
// 			return false;
// 		}
//
// 		this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
//
// 		if (move.ignoreImmunity === undefined) {
// 			move.ignoreImmunity = (move.category === 'Status');
// 		}
//
// 		if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
// 			this.battle.faint(pokemon, pokemon, move);
// 		}
//
// 		let damage: number | false | undefined | '' = false;
// 		if (move.target === 'all' || move.target === 'foeSide' || move.target === 'allySide' || move.target === 'allyTeam') {
// 			damage = this.tryMoveHit(targets, pokemon, move);
// 			if (damage === this.battle.NOT_FAIL) pokemon.moveThisTurnResult = null;
// 			if (damage || damage === 0 || damage === undefined) moveResult = true;
// 		} else {
// 			if (!targets.length) {
// 				this.battle.attrLastMove('[notarget]');
// 				this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
// 				return false;
// 			}
// 			if (this.battle.gen === 4 && move.selfdestruct === 'always') {
// 				this.battle.faint(pokemon, pokemon, move);
// 			}
// 			moveResult = this.trySpreadMoveHit(targets, pokemon, move);
// 		}
// 		if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
// 		if (!pokemon.hp) {
// 			this.battle.faint(pokemon, pokemon, move);
// 		}
//
// 		if (!moveResult) {
// 			this.battle.singleEvent('MoveFail', move, null, target, pokemon, move);
// 			return false;
// 		}
//
// 		if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce')) && !move.flags['futuremove']) {
// 			const originalHp = pokemon.hp;
// 			this.battle.singleEvent('AfterMoveSecondarySelf', move, null, pokemon, target, move);
// 			this.battle.runEvent('AfterMoveSecondarySelf', pokemon, target, move);
// 			if (pokemon && pokemon !== target && move.category !== 'Status') {
// 				if (pokemon.hp <= pokemon.maxhp / 2 && originalHp > pokemon.maxhp / 2) {
// 					this.battle.runEvent('EmergencyExit', pokemon, pokemon);
// 				}
// 			}
// 		}
//
// 		return true;
// 	}
//
pub fn use_move_inner(
    battle: &mut crate::battle::Battle,
    move_or_move_name: &ID,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_effect_param: Option<&ID>,
    z_move_param: Option<&str>,
    max_move_param: Option<&str>,
) -> bool {
    let prng_call_start = battle.prng.call_count;
    eprintln!("[USE_MOVE_INNER] START move {:?} from {:?} to {:?}, PRNG calls={}",
        move_or_move_name, pokemon_pos, target_pos, prng_call_start);
    // let target = options?.target;
    let mut target = target_pos;

    // let sourceEffect = options?.sourceEffect;
    let mut source_effect = source_effect_param.cloned();

    // const zMove = options?.zMove;
    let z_move = z_move_param;

    // const maxMove = options?.maxMove;
    let max_move = max_move_param;

    // if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
    // if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
    if source_effect.is_none() {
        if let Some(ref current_eff) = battle.current_effect {
            source_effect = Some(current_eff.clone());
        }
    }

    // Exclude instruct and custapberry from source effects
    if let Some(ref eff) = source_effect {
        let eff_id = eff.as_str();
        if eff_id == "instruct" || eff_id == "custapberry" {
            source_effect = None;
        }
    }

    // let move = this.dex.getActiveMove(moveOrMoveName);
    let mut active_move = match battle.dex.get_active_move(move_or_move_name.as_str()) {
        Some(m) => m,
        None => return false,
    };

    // pokemon.lastMoveUsed = move;
    let (side_idx, poke_idx) = pokemon_pos;
    battle.sides[side_idx].pokemon[poke_idx].last_move_used = Some(move_or_move_name.clone());

    // if (move.id === 'weatherball' && zMove) {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     if (move.type !== 'Normal') sourceEffect = move;
    // }
    if move_or_move_name.as_str() == "weatherball" && z_move.is_some() {
        battle.single_event(
            "ModifyType",
            move_or_move_name,
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
        );
        // After ModifyType event, check if move type changed from Normal
        if let Some(updated_move) = battle.dex.moves().get(move_or_move_name.as_str()) {
            if updated_move.move_type != "Normal" {
                source_effect = Some(move_or_move_name.clone());
            }
        }
    }

    // if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
    //     move = this.getActiveZMove(move, pokemon);
    // }
    if z_move.is_some()
        || (active_move.category != "Status"
            && source_effect.as_ref().is_some_and(|eff| {
                // Check if source effect is a Z-move
                battle
                    .dex
                    .get_active_move(eff.as_str())
                    .is_some_and(|m| m.is_z)
            }))
    {
        // TODO: Transform to Z-move - requires proper dex.getActiveMove implementation
        // TypeScript calls this.dex.getActiveMove() to get Z-move from dex
        // active_move = BattleActions::get_active_z_move(...);
    }

    // if (maxMove && move.category !== 'Status') {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // }
    if max_move.is_some() && active_move.category != "Status" {
        battle.single_event(
            "ModifyType",
            move_or_move_name,
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
        );
        battle.run_event(
            "ModifyType",
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
            None,
        );
    }

    // if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
    //     move = this.getActiveMaxMove(move, pokemon);
    // }
    if max_move.is_some()
        || (active_move.category != "Status"
            && source_effect.as_ref().is_some_and(|eff| {
                // Check if source effect is a Max move
                battle
                    .dex
                    .get_active_move(eff.as_str())
                    .is_some_and(|m| m.is_max)
            }))
    {
        // TODO: Transform to Max move - requires proper dex.getActiveMove implementation
        // TypeScript calls this.dex.getActiveMove() to get Max move from dex
        // active_move = BattleActions::get_active_max_move(...);
    }

    // if (this.battle.activeMove) {
    //     move.priority = this.battle.activeMove.priority;
    //     if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
    // }
    if let Some(ref battle_active_move) = battle.active_move {
        active_move.priority = battle_active_move.priority;
        if !active_move.has_bounced {
            active_move.prankster_boosted = battle_active_move.prankster_boosted;
        }
    }

    // const baseTarget = move.target;
    let base_target = active_move.target.clone();

    // let targetRelayVar = { target };
    // targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
    // if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
    // Implement ModifyTarget event using encoded target positions
    // This event allows moves to change their target (e.g., Payback, Metal Burst retargeting)
    // We encode target positions as integers: side_idx * 10 + pokemon_idx
    // Event handlers can return a new encoded position to redirect the move
    if let Some((target_side, target_pos)) = target {
        let encoded_target = (target_side as i32 * 10) + target_pos as i32;
        let modified_target = battle.run_event(
            "ModifyTarget",
            Some(pokemon_pos),
            Some((target_side, target_pos)),
            Some(move_or_move_name),
            Some(encoded_target),
        );
        if let Some(encoded) = modified_target {
            // Decode the modified target position
            let new_side = (encoded / 10) as usize;
            let new_pos = (encoded % 10) as usize;
            target = Some((new_side, new_pos));
        }
    }

    // if (target === undefined) target = this.battle.getRandomTarget(pokemon, move);
    // Call getRandomTarget if target is None
    // Gets a random valid target for a move based on its target type
    // Needed for moves with target="normal" or when original target is invalid
    if target.is_none() {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
    }

    // if (move.target === 'self' || move.target === 'allies') {
    //     target = pokemon;
    // }
    if active_move.target == "self" || active_move.target == "allies" {
        target = Some(pokemon_pos);
    }

    // if (sourceEffect) {
    //     move.sourceEffect = sourceEffect.id;
    //     move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
    // }
    if let Some(ref source_eff_id) = source_effect {
        active_move.source_effect = Some(source_eff_id.clone());

        // If sourceEffect is an ActiveMove, copy its ignoreAbility
        if let Some(source_move) = battle.dex.get_active_move(source_eff_id.as_str()) {
            active_move.ignore_ability = source_move.ignore_ability;
        }
    }

    // let moveResult = false;
    // Note: move_result will be initialized later based on move execution

    // this.battle.setActiveMove(move, pokemon, target);
    battle.set_active_move(Some(move_or_move_name.clone()), Some(pokemon_pos), target);

    // this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    // this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
    eprintln!("[USE_MOVE_INNER] turn={}, About to call singleEvent ModifyType and ModifyMove for move: {}", battle.turn, move_or_move_name.as_str());
    battle.single_event(
        "ModifyType",
        move_or_move_name,
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
    );
    eprintln!("[USE_MOVE_INNER] turn={}, Calling singleEvent ModifyMove for move: {}", battle.turn, move_or_move_name.as_str());
    battle.single_event(
        "ModifyMove",
        move_or_move_name,
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
    );
    eprintln!("[USE_MOVE_INNER] turn={}, Finished singleEvent ModifyMove for move: {}", battle.turn, move_or_move_name.as_str());

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    // Handle target adjustment after ModifyMove
    // If the move's target type changed in ModifyMove, need to get new target
    let current_target = battle
        .dex
        .moves().get(move_or_move_name.as_str())
        .unwrap()
        .target
        .clone();
    if base_target != current_target {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &current_target);
    }

    // move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
    battle.run_event(
        "ModifyType",
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
        None,
    );
    battle.run_event(
        "ModifyMove",
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
        None,
    );

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    // Handle second target adjustment
    // If the move's target type changed after runEvent('ModifyMove'), adjust target again
    let current_target = battle
        .dex
        .moves().get(move_or_move_name.as_str())
        .unwrap()
        .target
        .clone();
    if base_target != current_target {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &current_target);
    }

    // if (!move || pokemon.fainted) {
    //     return false;
    // }
    let (side_idx, poke_idx) = pokemon_pos;
    if battle
        .sides
        .get(side_idx)
        .and_then(|s| s.pokemon.get(poke_idx))
        .map(|p| p.is_fainted())
        .unwrap_or(true)
    {
        return false;
    }

    // let attrs = '';
    // let movename = move.name;
    // if (move.id === 'hiddenpower') movename = 'Hidden Power';
    // if (sourceEffect) attrs += `|[from] ${sourceEffect.fullname}`;
    // if (zMove && move.isZ === true) {
    //     attrs = `|[anim]${movename}${attrs}`;
    //     movename = `Z-${movename}`;
    // }
    // this.battle.addMove('move', pokemon, movename, `${target}${attrs}`);
    let move_name = active_move.name.clone();
    battle.add(
        "move",
        &[
            format!(
                "{}: {}",
                battle.sides[side_idx].id_str(),
                battle.sides[side_idx].pokemon[poke_idx].name
            ).into(),
            move_name.into(),
        ],
    );

    // if (zMove) this.runZPower(move, pokemon);
    // Implement runZPower for status Z-moves
    if z_move.is_some() {
        // Get Pokemon's types for Ghost type check (curse effect)
        let pokemon_has_ghost_type = battle.sides[side_idx].pokemon[poke_idx]
            .types
            .contains(&"Ghost".to_string());

        // Get z_move data from active_move (if it was transformed to Z-Move)
        if let Some(ref z_move_data) = active_move.z_move {
            // Call the run_z_power helper
            let z_power_result = BattleActions::run_z_power(
                &active_move.category,
                z_move_data.boost.as_ref(),
                z_move_data.effect.as_deref(),
                pokemon_has_ghost_type,
            );

            // Apply the Z-Power effect based on result
            match z_power_result {
                ZPowerResult::DamageMove => {
                    // JS: this.battle.attrLastMove('[zeffect]');
                    battle.attr_last_move(&["[zeffect]"]);
                }
                ZPowerResult::Boost(ref boosts) => {
                    // JS: this.battle.boost(move.zMove.boost, pokemon, pokemon, zPower);
                    // Convert BoostsTable to array of (stat_name, value) tuples
                    let mut boost_array = Vec::new();
                    if boosts.atk != 0 {
                        boost_array.push(("atk", boosts.atk));
                    }
                    if boosts.def != 0 {
                        boost_array.push(("def", boosts.def));
                    }
                    if boosts.spa != 0 {
                        boost_array.push(("spa", boosts.spa));
                    }
                    if boosts.spd != 0 {
                        boost_array.push(("spd", boosts.spd));
                    }
                    if boosts.spe != 0 {
                        boost_array.push(("spe", boosts.spe));
                    }
                    if boosts.accuracy != 0 {
                        boost_array.push(("accuracy", boosts.accuracy));
                    }
                    if boosts.evasion != 0 {
                        boost_array.push(("evasion", boosts.evasion));
                    }

                    battle.boost(&boost_array, pokemon_pos, Some(pokemon_pos), Some("zpower"), false, false);
                }
                ZPowerResult::Heal => {
                    // JS: this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
                    let max_hp = battle.sides[side_idx].pokemon[poke_idx].maxhp;
                    let zpower_id = ID::new("zpower");
                    battle.heal(
                        max_hp,
                        Some(pokemon_pos),
                        Some(pokemon_pos),
                        Some(&zpower_id),
                    );
                }
                ZPowerResult::HealReplacement => {
                    // JS: pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
                    let healreplacement_id = ID::new("healreplacement");
                    let slot = battle.sides[side_idx].pokemon[poke_idx].position;
                    battle.sides[side_idx].add_slot_condition(slot, healreplacement_id, None);
                }
                ZPowerResult::ClearNegativeBoost => {
                    // JS: Clear all negative boosts and add '-clearnegativeboost' message
                    let boosts_to_clear = {
                        let pokemon = &battle.sides[side_idx].pokemon[poke_idx];
                        let mut clear_boosts = Vec::new();
                        if pokemon.boosts.atk < 0 {
                            clear_boosts.push(("atk", -pokemon.boosts.atk));
                        }
                        if pokemon.boosts.def < 0 {
                            clear_boosts.push(("def", -pokemon.boosts.def));
                        }
                        if pokemon.boosts.spa < 0 {
                            clear_boosts.push(("spa", -pokemon.boosts.spa));
                        }
                        if pokemon.boosts.spd < 0 {
                            clear_boosts.push(("spd", -pokemon.boosts.spd));
                        }
                        if pokemon.boosts.spe < 0 {
                            clear_boosts.push(("spe", -pokemon.boosts.spe));
                        }
                        if pokemon.boosts.accuracy < 0 {
                            clear_boosts.push(("accuracy", -pokemon.boosts.accuracy));
                        }
                        if pokemon.boosts.evasion < 0 {
                            clear_boosts.push(("evasion", -pokemon.boosts.evasion));
                        }
                        clear_boosts
                    };

                    if !boosts_to_clear.is_empty() {
                        battle.boost(
                            &boosts_to_clear,
                            pokemon_pos,
                            Some(pokemon_pos),
                            Some("zpower"),
                            false,
                            false,
                        );
                        battle.add(
                            "-clearnegativeboost",
                            &[
                                format!(
                                    "{}: {}",
                                    battle.sides[side_idx].id_str(),
                                    battle.sides[side_idx].pokemon[poke_idx].name
                                ).into(),
                                "[zeffect]".into(),
                            ],
                        );
                    }
                }
                ZPowerResult::Redirect => {
                    // JS: pokemon.addVolatile('followme', pokemon, zPower);
                    let followme_id = ID::new("followme");
                    Pokemon::add_volatile(battle, (side_idx, poke_idx), followme_id, None, None);
                }
                ZPowerResult::Crit2 => {
                    // JS: pokemon.addVolatile('focusenergy', pokemon, zPower);
                    let focusenergy_id = ID::new("focusenergy");
                    Pokemon::add_volatile(battle, (side_idx, poke_idx), focusenergy_id, None, None);
                }
                ZPowerResult::None => {
                    // No Z-Power effect to apply
                }
            }
        }
    }

    // if (!target) {
    //     this.battle.attrLastMove('[notarget]');
    //     this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
    //     return false;
    // }
    let mut target_pos = match target {
        Some(t) => t,
        None => {
            battle.add("-notarget", &[]);
            return false;
        }
    };

    // const { targets, pressureTargets } = pokemon.getMoveTargets(move, target);
    // if (targets.length) {
    //     target = targets[targets.length - 1]; // in case of redirection
    // }
    // Implement getMoveTargets for multi-target handling
    // getMoveTargets returns all targets hit by a move (based on target type)
    // and pressure targets (for PP deduction via Pressure ability)
    let result = Pokemon::get_move_targets(battle, pokemon_pos, move_or_move_name, Some(target_pos));
    let (targets, pressure_targets) = (result.targets, result.pressure_targets);
    if !targets.is_empty() {
        // Update target in case of redirection
        target = Some(targets[targets.len() - 1]);
        if let Some(new_target) = target {
            target_pos = new_target;
        }
    }

    // const callerMoveForPressure = sourceEffect && (sourceEffect as ActiveMove).pp ? sourceEffect as ActiveMove : null;
    // if (!sourceEffect || callerMoveForPressure || sourceEffect.id === 'pursuit') {
    //     let extraPP = 0;
    //     for (const source of pressureTargets) {
    //         const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
    //         if (ppDrop !== true) {
    //             extraPP += ppDrop || 0;
    //         }
    //     }
    //     if (extraPP > 0) {
    //         pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
    //     }
    // }
    // Implement PP deduction with Pressure
    // The Pressure ability causes moves to lose 1 extra PP when targeting that Pokemon
    // This loops through pressureTargets and calls DeductPP event for each
    if source_effect.is_none()
        || source_effect
            .map(|e| e.as_str() == "pursuit")
            .unwrap_or(false)
    {
        let mut extra_pp = 0;
        for &pressure_target in &pressure_targets {
            // Call DeductPP event for each pressure target
            // JS: const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
            if let Some(pp_drop) = battle.run_event(
                "DeductPP",
                Some(pressure_target),
                Some(pokemon_pos),
                Some(move_or_move_name),
                Some(1),
            ) {
                // JS: if (ppDrop !== true) extraPP += ppDrop || 0;
                // runEvent returns Some(i32), we add it to extra PP
                extra_pp += pp_drop;
            }
        }

        // JS: if (extraPP > 0) pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
        if extra_pp > 0 {
            let (side_idx, poke_idx) = pokemon_pos;
            battle.sides[side_idx].pokemon[poke_idx].deduct_pp(battle.gen, move_or_move_name, Some(extra_pp as u8));
        }
    }

    // NOTE: BeforeMove event is called in run_move.rs, not here
    // JavaScript calls BeforeMove in runMove(), and useMoveInner() is called from runMove()
    // So we don't call it again here to avoid duplicate PRNG calls

    // if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
    //     !this.battle.runEvent('TryMove', pokemon, target, move)) {
    //     move.mindBlownRecoil = false;
    //     return false;
    // }
    let try_move_result = battle.single_event(
        "TryMove",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    if matches!(try_move_result, crate::event::EventResult::Boolean(false)) {
        // move.mindBlownRecoil = false (this would be set in move state if we tracked it)
        return false;
    }

    // Also check runEvent('TryMove')
    let run_try_move_result = battle.run_event_bool(
        "TryMove",
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    if !run_try_move_result {
        return false;
    }

    // this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
    battle.single_event(
        "UseMoveMessage",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );

    // if (move.ignoreImmunity === undefined) {
    //     move.ignoreImmunity = (move.category === 'Status');
    // }
    // In Rust, the move data is immutable, so this would be set in MoveData
    // Status moves default to ignoring immunity
    // (This is handled in the event system when checking immunity)

    // if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.gen != 4 && active_move.self_destruct == Some("always".to_string()) {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // let damage: number | false | undefined | '' = false;
    // Execute move based on target type
    let move_result = if matches!(
        active_move.target.as_str(),
        "all" | "foeSide" | "allySide" | "allyTeam"
    ) {
        // Field-wide moves - for now, treat like targeted moves with single target
        // Full implementation would use tryMoveHit instead of trySpreadMoveHit
        crate::battle_actions::try_spread_move_hit(battle, &[target_pos], pokemon_pos, move_or_move_name)
    } else {
        // Targeted moves - use trySpreadMoveHit
        // For now, we're using a single target (proper implementation would get all targets)
        crate::battle_actions::try_spread_move_hit(battle, &[target_pos], pokemon_pos, move_or_move_name)
    };

    // JS: this.battle.eachEvent('Update'); (line 969 - inside hit loop, after spreadMoveHit)
    // This is called once per hit for multi-hit moves, or once for single-hit moves
    battle.each_event("Update", None, None);

    // JavaScript recoil handling (battle-actions.ts lines 853-859):
    //   if ((move.recoil || move.id === "chloroblast") && move.totalDamage) {
    //     const hpBeforeRecoil = pokemon.hp;
    //     this.battle.damage(this.calcRecoilDamage(move.totalDamage, move, pokemon), pokemon, pokemon, "recoil");
    //     if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
    //       this.battle.runEvent("EmergencyExit", pokemon, pokemon);
    //     }
    //   }
    //
    // calcRecoilDamage (line 1180):
    //   if (move.id === "chloroblast") return Math.round(pokemon.maxhp / 2);
    //   return this.battle.clampIntRange(Math.round(damageDealt * move.recoil[0] / move.recoil[1]), 1);
    //
    if let Some(ref active_move) = battle.active_move.clone() {
        let has_recoil = active_move.recoil.is_some() || active_move.id.as_str() == "chloroblast";
        let total_damage = active_move.total_damage;

        if has_recoil && total_damage > 0 {
            // Get HP before recoil and maxhp for Emergency Exit check
            let hp_before_recoil = battle.sides[side_idx].pokemon[poke_idx].hp;
            let maxhp = battle.sides[side_idx].pokemon[poke_idx].maxhp;

            // Calculate recoil damage
            let recoil_damage = if active_move.id.as_str() == "chloroblast" {
                // JS: Math.round(pokemon.maxhp / 2)
                (maxhp as f64 / 2.0).round() as i32
            } else if let Some((recoil_num, recoil_denom)) = active_move.recoil {
                // JS: Math.round(damageDealt * move.recoil[0] / move.recoil[1])
                let recoil = (total_damage as f64 * recoil_num as f64 / recoil_denom as f64).round() as i32;
                // JS: this.battle.clampIntRange(recoil, 1)
                recoil.max(1)
            } else {
                0
            };

            // Apply recoil damage
            // JS: this.battle.damage(..., pokemon, pokemon, "recoil")
            if recoil_damage > 0 {
                let recoil_effect_id = ID::new("recoil");
                battle.damage(
                    recoil_damage,
                    Some(pokemon_pos),
                    Some(pokemon_pos),
                    Some(&recoil_effect_id),
                    false,  // instafaint = false
                );

                // Check for Emergency Exit after recoil
                // JS: if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2)
                let current_hp = battle.sides[side_idx].pokemon[poke_idx].hp;
                if current_hp <= maxhp / 2 && hp_before_recoil > maxhp / 2 {
                    battle.run_event(
                        "EmergencyExit",
                        Some(pokemon_pos),
                        Some(pokemon_pos),
                        None,
                        None,
                    );
                }
            }
        }
    }

    // if (moveData.selfdestruct === 'ifHit' && damage[i] !== false) {
    //     this.battle.faint(source, source, move);
    // }
    if active_move.self_destruct == Some("ifHit".to_string()) && move_result {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
    // Self-boost is handled through move data and event system
    // This would call apply_move_secondary or similar with the selfBoost data

    // if (!pokemon.hp) {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.sides[side_idx].pokemon[poke_idx].hp == 0 {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // if (!moveResult) {
    //     this.battle.singleEvent('MoveFail', move, null, target, pokemon, move);
    //     return false;
    // }
    if !move_result {
        battle.single_event(
            "MoveFail",
            move_or_move_name,
            target_pos.into(),
            Some(pokemon_pos),
            Some(move_or_move_name),
        );
        return false;
    }

    // if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce')) && !move.flags['futuremove']) {
    //     const originalHp = pokemon.hp;
    //     this.battle.singleEvent('AfterMoveSecondarySelf', move, null, pokemon, target, move);
    //     this.battle.runEvent('AfterMoveSecondarySelf', pokemon, target, move);
    //     if (pokemon && pokemon !== target && move.category !== 'Status') {
    //         if (pokemon.hp <= pokemon.maxhp / 2 && originalHp > pokemon.maxhp / 2) {
    //             this.battle.runEvent('EmergencyExit', pokemon, pokemon);
    //         }
    //     }
    // }
    // Check if Sheer Force applies (would skip secondary effects)
    // For now, always run the events (Sheer Force check would be in ability callbacks)
    let original_hp = battle.sides[side_idx].pokemon[poke_idx].hp;

    battle.single_event(
        "AfterMoveSecondarySelf",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    battle.run_event(
        "AfterMoveSecondarySelf",
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
        None,
    );

    // Check for Emergency Exit (abilities that trigger when HP drops below 50%)
    if target_pos != pokemon_pos && active_move.category != "Status" {
        let current_hp = battle.sides[side_idx].pokemon[poke_idx].hp;
        let max_hp = battle.sides[side_idx].pokemon[poke_idx].maxhp;
        if current_hp <= max_hp / 2 && original_hp > max_hp / 2 {
            battle.run_event(
                "EmergencyExit",
                Some(pokemon_pos),
                Some(pokemon_pos),
                None,
                None,
            );
        }
    }

    // return true;
    let prng_call_end = battle.prng.call_count;
    eprintln!("[USE_MOVE_INNER] END move {:?}, PRNG calls: {} -> {} (delta={})",
        move_or_move_name, prng_call_start, prng_call_end, prng_call_end - prng_call_start);
    true // Placeholder - will be set by actual move execution
}

