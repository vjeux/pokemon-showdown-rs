use crate::*;
use crate::event::EventResult;

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
    mut target_pos: Option<(usize, usize)>,
    source_effect_param: Option<&ID>,
    z_move_param: Option<&str>,
    max_move_param: Option<&str>,
) -> bool {
    eprintln!("[USE_MOVE_INNER] ENTRY: move={}, pokemon=({}, {}), target={:?}, PRNG={}",
        move_or_move_name.as_str(), pokemon_pos.0, pokemon_pos.1, target_pos, battle.prng.call_count);

    // let target = options?.target;
    // let sourceEffect = options?.sourceEffect;
    // const zMove = options?.zMove;
    // const maxMove = options?.maxMove;
    let mut source_effect = source_effect_param.cloned();
    let z_move = z_move_param;
    let max_move = max_move_param;

    // if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
    if source_effect.is_none() && battle.effect.as_ref().map_or(false, |e| !e.as_str().is_empty()) {
        source_effect = battle.effect.as_ref().cloned();
    }

    // if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
    if let Some(ref se) = source_effect {
        if se.as_str() == "instruct" || se.as_str() == "custapberry" {
            source_effect = None;
        }
    }

    // let move = this.dex.getActiveMove(moveOrMoveName);
    let mut active_move = match battle.dex.get_active_move(move_or_move_name.as_str()) {
        Some(m) => m,
        None => {
            eprintln!("[USE_MOVE_INNER] Move not found: {}", move_or_move_name.as_str());
            return false;
        }
    };

    // pokemon.lastMoveUsed = move;
    battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].last_move_used = Some(active_move.id.clone());

    // if (move.id === 'weatherball' && zMove) {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     if (move.type !== 'Normal') sourceEffect = move;
    // }
    if active_move.id.as_str() == "weatherball" && z_move.is_some() {
        // We need to run ModifyType event to potentially change the move type
        // Store the move temporarily in battle.active_move for the event
        battle.active_move = Some(active_move.clone());

        battle.single_event(
            "ModifyType",
            &active_move.id,
            Some(pokemon_pos),
            target_pos,
            Some(&active_move.id),
            None,
        );

        // Get the potentially modified move back
        if let Some(ref modified_move) = battle.active_move {
            if modified_move.move_type != "Normal" {
                source_effect = Some(active_move.id.clone());
            }
            active_move = modified_move.clone();
        }
    }

    // if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
    //     move = this.getActiveZMove(move, pokemon);
    // }
    let should_convert_to_z = z_move.is_some() ||
        (active_move.category != "Status" &&
         source_effect.as_ref().map_or(false, |se| {
             // Check if the source effect is a Z-move by checking battle.active_move
             battle.active_move.as_ref().map_or(false, |am| am.is_z && am.id == *se)
         }));

    if should_convert_to_z {
        active_move = crate::battle_actions::get_active_z_move(
            battle,
            pokemon_pos.0,
            pokemon_pos.1,
            &active_move.id.to_string(),
        );
    }

    // if (maxMove && move.category !== 'Status') {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // }
    if max_move.is_some() && active_move.category != "Status" {
        battle.active_move = Some(active_move.clone());

        battle.single_event(
            "ModifyType",
            &active_move.id,
            Some(pokemon_pos),
            target_pos,
            Some(&active_move.id),
            None,
        );

        battle.run_event("ModifyType", Some(pokemon_pos), target_pos, Some(&active_move.id), EventResult::Continue, false, false);

        if let Some(ref modified_move) = battle.active_move {
            active_move = modified_move.clone();
        }
    }

    // if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
    //     move = this.getActiveMaxMove(move, pokemon);
    // }
    let should_convert_to_max = max_move.is_some() ||
        (active_move.category != "Status" &&
         source_effect.as_ref().map_or(false, |se| {
             battle.active_move.as_ref().map_or(false, |am| am.is_max && am.id == *se)
         }));

    if should_convert_to_max {
        active_move = crate::battle_actions::get_active_max_move(
            battle,
            pokemon_pos.0,
            pokemon_pos.1,
            &active_move.id.to_string(),
        );
    }

    // if (this.battle.activeMove) {
    //     move.priority = this.battle.activeMove.priority;
    //     if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
    // }
    if let Some(ref existing_active_move) = battle.active_move {
        active_move.priority = existing_active_move.priority;
        if !active_move.has_bounced {
            active_move.prankster_boosted = existing_active_move.prankster_boosted;
        }
    }

    // const baseTarget = move.target;
    let base_target = active_move.target.clone();

    // let targetRelayVar = { target };
    // targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
    // if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
    let modify_target_result = battle.run_event("ModifyTarget", Some(pokemon_pos), target_pos, Some(&active_move.id), EventResult::Continue, false, false);

    // Extract the new target if ModifyTarget returned a position
    if let EventResult::Position(new_target) = modify_target_result {
        target_pos = Some(new_target);
    }

    // if (target === undefined) target = this.battle.getRandomTarget(pokemon, move);
    if target_pos.is_none() {
        target_pos = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
    }

    // if (move.target === 'self' || move.target === 'allies') {
    //     target = pokemon;
    // }
    if active_move.target == "self" || active_move.target == "allies" {
        eprintln!("[USE_MOVE_INNER] Setting target to self for move {}, target={:?}", active_move.id, active_move.target);
        target_pos = Some(pokemon_pos);
    }

    // if (sourceEffect) {
    //     move.sourceEffect = sourceEffect.id;
    //     move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
    // }
    if let Some(ref se) = source_effect {
        active_move.source_effect_name = Some(se.to_string());
        // Check if source effect is an active move and has ignoreAbility
        if let Some(ref existing_active) = battle.active_move {
            if existing_active.id == *se {
                active_move.ignore_ability = existing_active.ignore_ability;
            }
        }
    }

    // let moveResult = false;
    let move_result;

    // this.battle.setActiveMove(move, pokemon, target);
    battle.set_active_move(Some(active_move.id.clone()), Some(pokemon_pos), target_pos);
    battle.active_move = Some(active_move.clone());

    // this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    battle.single_event(
        "ModifyType",
        &active_move.id,
        Some(pokemon_pos),
        target_pos,
        Some(&active_move.id),
        None,
    );

    // this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
    battle.single_event(
        "ModifyMove",
        &active_move.id,
        Some(pokemon_pos),
        target_pos,
        Some(&active_move.id),
        None,
    );

    // Get potentially modified move
    if let Some(ref modified) = battle.active_move {
        active_move = modified.clone();
    }

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    eprintln!("[USE_MOVE_INNER] After ModifyMove: base_target={}, active_move.target={}, target_pos={:?}", base_target, active_move.target, target_pos);
    if base_target != active_move.target {
        eprintln!("[USE_MOVE_INNER] Target changed! Getting new random target");
        target_pos = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
        eprintln!("[USE_MOVE_INNER] New target_pos={:?}", target_pos);
    }

    // move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
    battle.run_event("ModifyType", Some(pokemon_pos), target_pos, Some(&active_move.id), EventResult::Continue, false, false);

    // move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
    battle.run_event("ModifyMove", Some(pokemon_pos), target_pos, Some(&active_move.id), EventResult::Continue, false, false);

    // Get potentially modified move again
    if let Some(ref modified) = battle.active_move {
        active_move = modified.clone();
    }

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    eprintln!("[USE_MOVE_INNER] After ModifyMove: base_target={}, active_move.target={}, target_pos={:?}", base_target, active_move.target, target_pos);
    if base_target != active_move.target {
        eprintln!("[USE_MOVE_INNER] Target changed! Getting new random target");
        target_pos = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
        eprintln!("[USE_MOVE_INNER] New target_pos={:?}", target_pos);
    }

    // if (!move || pokemon.fainted) {
    //     return false;
    // }
    let pokemon_fainted = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].is_fainted();
    if pokemon_fainted {
        eprintln!("[USE_MOVE_INNER] Pokemon fainted, returning false");
        return false;
    }

    // let attrs = '';
    let mut attrs = String::new();

    // let movename = move.name;
    // if (move.id === 'hiddenpower') movename = 'Hidden Power';
    let mut movename = active_move.name.clone();
    if active_move.id.as_str() == "hiddenpower" {
        movename = "Hidden Power".to_string();
    }

    // if (sourceEffect) attrs += `|[from] ${sourceEffect.fullname}`;
    if let Some(ref se) = source_effect {
        // Get the fullname of the source effect
        if let Some(effect_move) = battle.dex.moves().get(se.as_str()) {
            attrs.push_str(&format!("|[from] {}", effect_move.name));
        } else if let Some(effect_item) = battle.dex.items().get(se.as_str()) {
            attrs.push_str(&format!("|[from] item: {}", effect_item.name));
        } else if let Some(effect_ability) = battle.dex.abilities().get(se.as_str()) {
            attrs.push_str(&format!("|[from] ability: {}", effect_ability.name));
        }
    }

    // if (zMove && move.isZ === true) {
    //     attrs = `|[anim]${movename}${attrs}`;
    //     movename = `Z-${movename}`;
    // }
    if z_move.is_some() && active_move.is_z {
        attrs = format!("|[anim]{}{}", movename, attrs);
        movename = format!("Z-{}", movename);
    }

    // this.battle.addMove('move', pokemon, movename, `${target}${attrs}`);
    let target_str = if let Some(tp) = target_pos {
        format!("{}a", tp.0 + 1) // Simple target format, might need adjustment
    } else {
        String::new()
    };
    battle.add_move(&["move", &format!("{}a", pokemon_pos.0 + 1), &movename, &format!("{}{}", target_str, attrs)]);

    // if (zMove) this.runZPower(move, pokemon);
    if z_move.is_some() {
        // Apply Z-Power effect for status Z-moves
        if let Some(ref am) = battle.active_move {
            if am.category == "Status" {
                // Get Z-power effect ID
                let zpower_id = ID::from("zpower");

                // Check if there's a boost
                if let Some(ref z_move_data) = am.z_move {
                    if let Some(ref boost_table) = z_move_data.boost {
                        // Convert BoostsTable to array format for boost method
                        let mut boosts_array: Vec<(&str, i8)> = Vec::new();
                        if boost_table.atk != 0 { boosts_array.push(("atk", boost_table.atk)); }
                        if boost_table.def != 0 { boosts_array.push(("def", boost_table.def)); }
                        if boost_table.spa != 0 { boosts_array.push(("spa", boost_table.spa)); }
                        if boost_table.spd != 0 { boosts_array.push(("spd", boost_table.spd)); }
                        if boost_table.spe != 0 { boosts_array.push(("spe", boost_table.spe)); }
                        if boost_table.accuracy != 0 { boosts_array.push(("accuracy", boost_table.accuracy)); }
                        if boost_table.evasion != 0 { boosts_array.push(("evasion", boost_table.evasion)); }

                        // Apply boost
                        battle.boost(&boosts_array, pokemon_pos, Some(pokemon_pos), Some(zpower_id.as_str()), false, false);
                    } else if let Some(ref effect) = z_move_data.effect {
                        // Apply effect based on type
                        match effect.as_str() {
                            "heal" => {
                                let maxhp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;
                                battle.heal(maxhp, Some(pokemon_pos), Some(pokemon_pos), Some(&zpower_id));
                            }
                            "healreplacement" => {
                                // pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
                                let pokemon_position = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].position;
                                let healreplacement_id = ID::from("healreplacement");
                                battle.sides[pokemon_pos.0].add_slot_condition(pokemon_position, healreplacement_id, None);
                            }
                            "clearnegativeboost" => {
                                // Clear negative boosts
                                let mut boosts_array: Vec<(&str, i8)> = Vec::new();
                                let current_boosts = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].boosts.clone();
                                if current_boosts.atk < 0 { boosts_array.push(("atk", -current_boosts.atk)); }
                                if current_boosts.def < 0 { boosts_array.push(("def", -current_boosts.def)); }
                                if current_boosts.spa < 0 { boosts_array.push(("spa", -current_boosts.spa)); }
                                if current_boosts.spd < 0 { boosts_array.push(("spd", -current_boosts.spd)); }
                                if current_boosts.spe < 0 { boosts_array.push(("spe", -current_boosts.spe)); }
                                if current_boosts.accuracy < 0 { boosts_array.push(("accuracy", -current_boosts.accuracy)); }
                                if current_boosts.evasion < 0 { boosts_array.push(("evasion", -current_boosts.evasion)); }

                                if !boosts_array.is_empty() {
                                    battle.boost(&boosts_array, pokemon_pos, Some(pokemon_pos), Some(zpower_id.as_str()), false, false);
                                }
                                let pokemon_id = format!("p{}a", pokemon_pos.0 + 1);
                                battle.add("-clearnegativeboost", &[Arg::String(pokemon_id), Arg::Str("[zeffect]")]);
                            }
                            "redirect" => {
                                // pokemon.addVolatile('followme', pokemon, zPower);
                                let followme_id = ID::from("followme");
                                Pokemon::add_volatile(battle, pokemon_pos, followme_id, Some(pokemon_pos), Some(&zpower_id), None, None);
                            }
                            "crit2" => {
                                // pokemon.addVolatile('focusenergy', pokemon, zPower);
                                let focusenergy_id = ID::from("focusenergy");
                                Pokemon::add_volatile(battle, pokemon_pos, focusenergy_id, Some(pokemon_pos), Some(&zpower_id), None, None);
                            }
                            "curse" => {
                                let has_ghost = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1]
                                    .has_type(battle, "Ghost");
                                if has_ghost {
                                    let maxhp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;
                                    battle.heal(maxhp, Some(pokemon_pos), Some(pokemon_pos), Some(&zpower_id));
                                } else {
                                    let boosts_array = [("atk", 1i8)];
                                    battle.boost(&boosts_array, pokemon_pos, Some(pokemon_pos), Some(zpower_id.as_str()), false, false);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                // Damage Z-moves just add [zeffect] attribute
                battle.attr_last_move(&["[zeffect]"]);
            }
        }
    }

    // if (!target) {
    //     this.battle.attrLastMove('[notarget]');
    //     this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
    //     return false;
    // }
    if target_pos.is_none() {
        battle.attr_last_move(&["[notarget]"]);
        let fail_msg = if battle.gen >= 5 { "-fail" } else { "-notarget" };
        let pokemon_id = format!("p{}a", pokemon_pos.0 + 1);
        battle.add(fail_msg, &[Arg::String(pokemon_id)]);
        return false;
    }

    let target = target_pos.unwrap(); // Safe because we just checked

    // const { targets, pressureTargets } = pokemon.getMoveTargets(move, target);
    let get_move_targets_result = crate::pokemon::Pokemon::get_move_targets(
        battle,
        pokemon_pos,
        &active_move.id,
        Some(target),
    );
    let targets = get_move_targets_result.targets;
    let pressure_targets = get_move_targets_result.pressure_targets;

    // if (targets.length) {
    //     target = targets[targets.length - 1];
    // }
    let final_target = if !targets.is_empty() {
        targets[targets.len() - 1]
    } else {
        target
    };

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
    let caller_move_for_pressure = source_effect.as_ref().and_then(|se| {
        if battle.active_move.as_ref().map_or(false, |am| am.id == *se && am.pp > 0) {
            source_effect.clone()
        } else {
            None
        }
    });

    if source_effect.is_none() || caller_move_for_pressure.is_some() ||
       source_effect.as_ref().map_or(false, |se| se.as_str() == "pursuit") {
        let mut extra_pp = 0;

        for &pressure_target_pos in &pressure_targets {
            // Run DeductPP event to check for Pressure ability
            let pp_drop_result = battle.run_event(
                "DeductPP",
                Some(pressure_target_pos),
                Some(pokemon_pos),
                Some(&active_move.id),
                EventResult::Number(1), // Default PP drop is 1
                false,
                false,
            );

            // If the event returns a number > 1, it means Pressure is active
            if let EventResult::Number(pp_drop) = pp_drop_result {
                if pp_drop > 1 {
                    extra_pp += (pp_drop - 1) as u8; // Add extra PP (beyond the base 1)
                }
            }
        }

        if extra_pp > 0 {
            let move_to_deduct = caller_move_for_pressure.as_ref().unwrap_or(move_or_move_name);
            battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].deduct_pp(battle.gen, move_to_deduct, Some(extra_pp));
        }
    }

    // if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
    //     !this.battle.runEvent('TryMove', pokemon, target, move)) {
    //     move.mindBlownRecoil = false;
    //     return false;
    // }
    let try_move_single = battle.single_event(
        "TryMove",
        &active_move.id,
        Some(pokemon_pos),
        Some(final_target),
        Some(&active_move.id),
        None,
    );

    let try_move_run = battle.run_event_bool(
        "TryMove",
        Some(pokemon_pos),
        Some(final_target),
        Some(&active_move.id),
    );

    let try_move_success = !matches!(try_move_single, crate::event::EventResult::Boolean(false) |
                                                       crate::event::EventResult::Null |
                                                       crate::event::EventResult::Stop) && try_move_run;

    if !try_move_success {
        if let Some(ref mut am) = battle.active_move {
            am.mindblown_recoil = false;
        }
        eprintln!("[USE_MOVE_INNER] TryMove failed, returning false");
        return false;
    }

    // this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
    battle.single_event(
        "UseMoveMessage",
        &active_move.id,
        Some(pokemon_pos),
        Some(final_target),
        Some(&active_move.id),
        None,
    );

    // if (move.ignoreImmunity === undefined) {
    //     move.ignoreImmunity = (move.category === 'Status');
    // }
    if let Some(ref mut am) = battle.active_move {
        if am.ignore_immunity.is_none() {
            if am.category == "Status" {
                am.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::All);
            }
        }
    }

    // if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.gen != 4 {
        if let Some(ref am) = battle.active_move {
            if am.self_destruct.as_deref() == Some("always") {
                battle.faint(pokemon_pos, Some(pokemon_pos), Some(active_move.id.as_str()));
            }
        }
    }

    // let damage: number | false | undefined | '' = false;
    // if (move.target === 'all' || move.target === 'foeSide' || move.target === 'allySide' || move.target === 'allyTeam') {
    //     damage = this.tryMoveHit(targets, pokemon, move);
    //     if (damage === this.battle.NOT_FAIL) pokemon.moveThisTurnResult = null;
    //     if (damage || damage === 0 || damage === undefined) moveResult = true;
    // } else {
    //     ...
    // }
    if active_move.target == "all" || active_move.target == "foeSide" ||
       active_move.target == "allySide" || active_move.target == "allyTeam" {
        eprintln!("[USE_MOVE_INNER] turn={}, move={}, taking tryMoveHit branch (target={})",
            battle.turn, active_move.id, active_move.target);
        // damage = this.tryMoveHit(targets, pokemon, move);
        let hit_result = crate::battle_actions::try_move_hit(
            battle,
            &targets,
            pokemon_pos,
            &active_move.id,
        );

        // if (damage === this.battle.NOT_FAIL) pokemon.moveThisTurnResult = null;
        // In JavaScript, NOT_FAIL is an empty string ""
        // Handle NOT_FAIL case - if move returned NOT_FAIL, clear moveThisTurnResult
        if hit_result.is_not_fail() {
            // Set pokemon.moveThisTurnResult to null
            if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon.move_this_turn_result = None;
            }
        }

        // if (damage || damage === 0 || damage === undefined) moveResult = true;
        // Move succeeds if it dealt damage, or if it's NOT_FAIL or undefined
        move_result = hit_result;
    } else {
        eprintln!("[USE_MOVE_INNER] turn={}, move={}, taking trySpreadMoveHit branch (target={})",
            battle.turn, active_move.id, active_move.target);
        // if (!targets.length) {
        //     this.battle.attrLastMove('[notarget]');
        //     this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
        //     return false;
        // }
        if targets.is_empty() {
            battle.attr_last_move(&["[notarget]"]);
            let fail_msg = if battle.gen >= 5 { "-fail" } else { "-notarget" };
            let pokemon_id = format!("p{}a", pokemon_pos.0 + 1);
            battle.add(fail_msg, &[Arg::String(pokemon_id)]);
            return false;
        }

        // if (this.battle.gen === 4 && move.selfdestruct === 'always') {
        //     this.battle.faint(pokemon, pokemon, move);
        // }
        if battle.gen == 4 {
            if let Some(ref am) = battle.active_move {
                if am.self_destruct.as_deref() == Some("always") {
                    battle.faint(pokemon_pos, Some(pokemon_pos), Some(active_move.id.as_str()));
                }
            }
        }

        // moveResult = this.trySpreadMoveHit(targets, pokemon, move);
        move_result = crate::battle_actions::try_spread_move_hit::try_spread_move_hit(
            battle,
            &targets,
            pokemon_pos,
            &active_move.id,
            false,
        );
    }

    // if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
    let (has_self_boost, move_id_for_boost) = if let Some(ref am) = battle.active_move {
        (am.self_boost.is_some(), am.id.clone())
    } else {
        (false, ID::from(""))
    };

    if has_self_boost && move_result.is_success() {
        // Apply self-boost using moveHit
        // moveHit(pokemon, pokemon, move, move.selfBoost, false, true)
        // Parameters: targets, pokemon, move, moveData, isSecondary, isSelf
        crate::battle_actions::move_hit(
            battle,
            &[Some(pokemon_pos)], // targets = [pokemon] (self-targeting)
            pokemon_pos,          // pokemon (user)
            &move_id_for_boost,   // move
            Some(&move_id_for_boost), // moveData (use move.selfBoost data - passed via active_move)
            false,                // isSecondary
            true,                 // isSelf
        );
    }

    // if (!pokemon.hp) {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    let pokemon_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].hp;
    if pokemon_hp == 0 {
        battle.faint(pokemon_pos, Some(pokemon_pos), Some(active_move.id.as_str()));
    }

    // if (!moveResult) {
    //     this.battle.singleEvent('MoveFail', move, null, target, pokemon, move);
    //     return false;
    // }
    if !move_result.is_success() {
        battle.single_event(
            "MoveFail",
            &active_move.id,
            Some(final_target),
            Some(pokemon_pos),
            Some(&active_move.id),
            None,
        );
        eprintln!("[USE_MOVE_INNER] Move failed, returning false");
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
    let has_sheer_force = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1]
        .has_ability(battle, &["sheerforce"]);
    let has_futuremove = battle.active_move.as_ref()
        .map_or(false, |am| am.flags.future_move);

    if !(active_move.has_sheer_force && has_sheer_force) && !has_futuremove {
        let original_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].hp;

        battle.single_event(
            "AfterMoveSecondarySelf",
            &active_move.id,
            Some(pokemon_pos),
            Some(final_target),
            Some(&active_move.id),
            None,
        );

        battle.run_event("AfterMoveSecondarySelf", Some(pokemon_pos), Some(final_target), Some(&active_move.id), EventResult::Continue, false, false);

        let current_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].hp;
        let max_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;

        if pokemon_pos != final_target && active_move.category != "Status" {
            if current_hp <= max_hp / 2 && original_hp > max_hp / 2 {
                battle.run_event("EmergencyExit", Some(pokemon_pos), Some(pokemon_pos), None, EventResult::Continue, false, false);
            }
        }
    }

    eprintln!("[USE_MOVE_INNER] SUCCESS, returning true, PRNG={}", battle.prng.call_count);
    // return true;
    true
}

