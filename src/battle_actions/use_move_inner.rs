use crate::*;
use crate::event::EventResult;
use crate::dex::MoveData;

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
    move_data: &MoveData,
    pokemon_pos: (usize, usize),
    mut target_pos: Option<(usize, usize)>,
    source_effect_param: Option<&Effect>,
    z_move_param: Option<&str>,
    max_move_param: Option<&str>,
) -> bool {
    debug_elog!("[USE_MOVE_INNER] ENTRY: move={}, pokemon=({}, {}), target={:?}, PRNG={}",
        move_data.id.as_str(), pokemon_pos.0, pokemon_pos.1, target_pos, battle.prng.call_count);

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
    // Since we have MoveData, convert it directly to ActiveMove
    let mut active_move = match battle.dex.get_active_move(move_data.id.as_str()) {
        Some(m) => m,
        None => {
            debug_elog!("[USE_MOVE_INNER] Move not found: {}", move_data.id.as_str());
            return false;
        }
    };

    // Create move effect once for reuse in all run_event/single_event calls
    let move_effect = battle.make_move_effect(&active_move.id);

    // pokemon.lastMoveUsed = move;
    // Note: We set this after ModifyType runs so we capture the runtime type

    // if (move.id === 'weatherball' && zMove) {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     if (move.type !== 'Normal') sourceEffect = move;
    // }
    if active_move.id.as_str() == "weatherball" && z_move.is_some() {
        // We need to run ModifyType event to potentially change the move type
        // Store the move temporarily in battle.active_move for the event
        battle.active_move = Some(crate::battle_actions::SharedActiveMove::new(active_move.clone()));

        let modify_type_result = battle.single_event(
            "ModifyType",
            &move_effect,
            None,
            Some(pokemon_pos),
            target_pos,
            Some(&move_effect),
            None,
        );
        // Apply the returned type to active_move if the callback returned a String
        if let EventResult::String(new_type) = modify_type_result {
            active_move.move_type = new_type.clone();
            if let Some(ref am) = battle.active_move {
                am.borrow_mut().move_type = new_type;
            }
        }

        // Get the potentially modified move back
        if let Some(ref modified_move) = battle.active_move {
            if modified_move.borrow().move_type != "Normal" {
                source_effect = Some(move_effect.clone());
            }
            active_move = modified_move.borrow().clone();
        }
    }

    // if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
    //     move = this.getActiveZMove(move, pokemon);
    // }
    let should_convert_to_z = z_move.is_some() ||
        (active_move.category != "Status" &&
         source_effect.as_ref().map_or(false, |se| {
             // Check if the source effect is a Z-move by checking battle.active_move
             battle.active_move.as_ref().map_or(false, |am| am.borrow().is_z.is_some() && am.borrow().id == se.id)
         }));

    if should_convert_to_z {
        // Look up the move data from the dex to pass to get_active_z_move
        if let Some(base_move_data) = battle.dex.moves().get(active_move.id.as_str()) {
            active_move = crate::battle_actions::get_active_z_move(
                battle,
                pokemon_pos.0,
                pokemon_pos.1,
                base_move_data,
            );
        }
    }

    // if (maxMove && move.category !== 'Status') {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // }
    if max_move.is_some() && active_move.category != "Status" {
        battle.active_move = Some(crate::battle_actions::SharedActiveMove::new(active_move.clone()));

        let modify_type_result = battle.single_event(
            "ModifyType",
            &move_effect,
            None,
            Some(pokemon_pos),
            target_pos,
            Some(&move_effect),
            None,
        );
        // Apply the returned type to active_move if the callback returned a String
        if let EventResult::String(new_type) = modify_type_result {
            active_move.move_type = new_type.clone();
            if let Some(ref am) = battle.active_move {
                am.borrow_mut().move_type = new_type;
            }
        }

        battle.run_event("ModifyType", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&move_effect), EventResult::Continue, false, false);

        if let Some(ref modified_move) = battle.active_move {
            active_move = modified_move.borrow().clone();
        }
    }

    // if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
    //     move = this.getActiveMaxMove(move, pokemon);
    // }
    let should_convert_to_max = max_move.is_some() ||
        (active_move.category != "Status" &&
         source_effect.as_ref().map_or(false, |se| {
             battle.active_move.as_ref().map_or(false, |am| am.borrow().is_max.is_some() && am.borrow().id == se.id)
         }));

    if should_convert_to_max {
        // Look up the move data from the dex to pass to get_active_max_move
        if let Some(base_move_data) = battle.dex.moves().get(active_move.id.as_str()) {
            active_move = crate::battle_actions::get_active_max_move(
                battle,
                pokemon_pos.0,
                pokemon_pos.1,
                base_move_data,
            );
        }
    }

    // if (this.battle.activeMove) {
    //     move.priority = this.battle.activeMove.priority;
    //     if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
    // }
    if let Some(ref existing_active_move) = battle.active_move {
        let borrowed = existing_active_move.borrow();
        active_move.priority = borrowed.priority;
        if !active_move.has_bounced {
            active_move.prankster_boosted = borrowed.prankster_boosted;
        }
    }

    // const baseTarget = move.target;
    let base_target = active_move.target.clone();

    // let targetRelayVar = { target };
    // targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
    // if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
    let modify_target_result = battle.run_event("ModifyTarget", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&move_effect), EventResult::Continue, false, false);

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
        debug_elog!("[USE_MOVE_INNER] Setting target to self for move {}, target={:?}", active_move.id, active_move.target);
        target_pos = Some(pokemon_pos);
    }

    // if (sourceEffect) {
    //     move.sourceEffect = sourceEffect.id;
    //     move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
    // }
    if let Some(ref se) = source_effect {
        active_move.source_effect_name = Some(se.to_string());
        // Also store the full source_effect for callbacks that check it (e.g., twoturnmove.onStart)
        active_move.source_effect = Some(se.clone());
        // Check if source effect is an active move and has ignoreAbility
        if let Some(ref existing_active) = battle.active_move {
            let borrowed = existing_active.borrow();
            if borrowed.id == se.id {
                active_move.ignore_ability = borrowed.ignore_ability;
            }
        }
    }

    // let moveResult = false;
    let move_result;

    // Preserve has_bounced flag from old active_move if it was set
    // This is needed for Magic Bounce/Magic Coat - they set has_bounced = true on the
    // current active_move before calling use_move, and we need to preserve that flag
    // on the new active_move to prevent infinite recursion
    let old_has_bounced = battle.active_move.as_ref().map_or(false, |m| m.borrow().has_bounced);
    if old_has_bounced {
        active_move.has_bounced = true;
    }

    // this.battle.setActiveMove(move, pokemon, target);
    battle.set_active_move(Some(active_move.id.clone()), Some(pokemon_pos), target_pos);
    battle.active_move = Some(crate::battle_actions::SharedActiveMove::new(active_move.clone()));

    // Run ModifyPriority to allow abilities like Prankster to boost priority
    // This is needed because the active_move was just created from dex data with base priority
    // Abilities like Dazzling check move.priority, which needs to include these boosts
    let priority_result = battle.run_event(
        "ModifyPriority",
        Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
        target_pos,
        Some(&move_effect),
        EventResult::Number(active_move.priority as i32),
        false,
        false,
    );
    // Apply the modified priority
    if let EventResult::Number(new_priority) = priority_result {
        if let Some(ref am) = battle.active_move {
            am.borrow_mut().priority = new_priority as i8;
        }
    }
    // Re-clone the active_move to get any modifications
    if let Some(ref modified) = battle.active_move {
        active_move = modified.borrow().clone();
    }

    // this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    // In JavaScript, the callback modifies move.type directly.
    // In Rust, the callback returns EventResult::String(type_name) which we apply to active_move.move_type.
    let modify_type_result = battle.single_event(
        "ModifyType",
        &move_effect,
        None,
        Some(pokemon_pos),
        target_pos,
        Some(&move_effect),
        None,
    );
    // Apply the returned type to active_move if the callback returned a String
    if let EventResult::String(new_type) = modify_type_result {
        active_move.move_type = new_type.clone();
        // Also update battle.active_move so the change persists
        if let Some(ref am) = battle.active_move {
            am.borrow_mut().move_type = new_type;
        }
    }

    // this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
    battle.single_event(
        "ModifyMove",
        &move_effect,
        None,
        Some(pokemon_pos),
        target_pos,
        Some(&move_effect),
        None,
    );

    // Get potentially modified move
    if let Some(ref modified) = battle.active_move {
        active_move = modified.borrow().clone();
    }

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    debug_elog!("[USE_MOVE_INNER] After ModifyMove: base_target={}, active_move.target={}, target_pos={:?}", base_target, active_move.target, target_pos);
    if base_target != active_move.target {
        debug_elog!("[USE_MOVE_INNER] Target changed! Getting new random target");
        target_pos = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
        debug_elog!("[USE_MOVE_INNER] New target_pos={:?}", target_pos);
    }

    // move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
    battle.run_event("ModifyType", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&move_effect), EventResult::Continue, false, false);

    // move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
    let modify_move_result = battle.run_event("ModifyMove", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&move_effect), EventResult::Continue, false, false);

    // Get potentially modified move again
    if let Some(ref modified) = battle.active_move {
        active_move = modified.borrow().clone();
    }

    // pokemon.lastMoveUsed = move; (after ModifyType to capture runtime modifications)
    battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].last_move_used = Some(Box::new(active_move.clone()));

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    debug_elog!("[USE_MOVE_INNER] After ModifyMove: base_target={}, active_move.target={}, target_pos={:?}", base_target, active_move.target, target_pos);
    if base_target != active_move.target {
        debug_elog!("[USE_MOVE_INNER] Target changed! Getting new random target");
        target_pos = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
        debug_elog!("[USE_MOVE_INNER] New target_pos={:?}", target_pos);
    }

    // if (!move || pokemon.fainted) {
    //     return false;
    // }
    // JavaScript: if (!move || pokemon.fainted) - "!move" means if move became falsy (e.g., false/null from onModifyMove)
    // We check if modify_move_result is falsy (Boolean(false), Null, etc.)
    let move_blocked = match modify_move_result {
        EventResult::Boolean(false) | EventResult::Null => true,
        _ => false,
    };
    if move_blocked {
        debug_elog!("[USE_MOVE_INNER] Move blocked by ModifyMove (result: {:?}), returning false", modify_move_result);
        return false;
    }
    let pokemon_fainted = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].is_fainted();
    if pokemon_fainted {
        debug_elog!("[USE_MOVE_INNER] Pokemon fainted, returning false");
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
    if z_move.is_some() && active_move.is_z.is_some() {
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
            let am_borrowed = am.borrow();
            if am_borrowed.category == "Status" {
                // Get Z-power effect ID
                let zpower_id = ID::from("zpower");
                let zpower_effect = battle.make_move_effect(&zpower_id);

                // Check if there's a boost
                if let Some(ref z_move_data) = am_borrowed.z_move {
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

                        // Need to drop borrow before calling battle methods
                        drop(am_borrowed);
                        // Apply boost
                        battle.boost(&boosts_array, pokemon_pos, Some(pokemon_pos), Some(zpower_id.as_str()), false, false);
                    } else if let Some(ref effect) = z_move_data.effect {
                        // Apply effect based on type
                        let effect_str = effect.clone();
                        drop(am_borrowed);
                        match effect_str.as_str() {
                            "heal" => {
                                let maxhp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;
                                battle.heal(maxhp, Some(pokemon_pos), Some(pokemon_pos), Some(&zpower_effect));
                            }
                            "healreplacement" => {
                                // pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
                                let pokemon_position = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].position;
                                let healreplacement_id = ID::from("healreplacement");
                                battle.add_slot_condition(
                                    (pokemon_pos.0, pokemon_position),
                                    healreplacement_id,
                                    Some(pokemon_pos),
                                    Some(&zpower_effect),
                                    None,
                                );
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
                                Pokemon::add_volatile(battle, pokemon_pos, followme_id, Some(pokemon_pos), Some(&zpower_effect), None, None);
                            }
                            "crit2" => {
                                // pokemon.addVolatile('focusenergy', pokemon, zPower);
                                let focusenergy_id = ID::from("focusenergy");
                                Pokemon::add_volatile(battle, pokemon_pos, focusenergy_id, Some(pokemon_pos), Some(&zpower_effect), None, None);
                            }
                            "curse" => {
                                let has_ghost = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1]
                                    .has_type(battle, "Ghost");
                                if has_ghost {
                                    let maxhp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;
                                    battle.heal(maxhp, Some(pokemon_pos), Some(pokemon_pos), Some(&zpower_effect));
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
                drop(am_borrowed);
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
        &active_move,
        Some(target),
    );
    let targets = get_move_targets_result.targets;
    let pressure_targets = get_move_targets_result.pressure_targets;

    // Handle smartTarget clearing (JS: move.smartTarget = false in getSmartTargets)
    // This happens when there's no valid adjacent ally for Dragon Darts
    if get_move_targets_result.should_clear_smart_target {
        active_move.smart_target = Some(false);
    }

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
    // JavaScript: const callerMoveForPressure = sourceEffect && (sourceEffect as ActiveMove).pp ? sourceEffect as ActiveMove : null;
    // Check if source_effect is a move (moves have PP). In JS, this checks if sourceEffect.pp exists and is > 0.
    // In Rust, we check if source_effect is a Move effect type, which implies it has PP.
    let caller_move_for_pressure = source_effect.as_ref().and_then(|se| {
        if se.effect_type == crate::battle::EffectType::Move {
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
            // JavaScript: const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
            //             if (ppDrop !== true) { extraPP += ppDrop || 0; }
            // Pressure returns 1 (extra PP to deduct)
            let pp_drop_result = battle.run_event(
                "DeductPP",
                Some(crate::event::EventTarget::Pokemon(pressure_target_pos)),
                Some(pokemon_pos),
                Some(&move_effect),
                EventResult::Boolean(true), // Default is true (no extra PP)
                false,
                false,
            );

            // JavaScript: if (ppDrop !== true) { extraPP += ppDrop || 0; }
            // If a handler returned a Number (like Pressure returning 1), add it
            match pp_drop_result {
                EventResult::Number(pp_drop) => {
                    extra_pp += pp_drop as u8;
                }
                EventResult::Boolean(true) => {
                    // No modification, don't add
                }
                _ => {
                    // Other cases (Continue treated as no modification)
                }
            }
        }

        if extra_pp > 0 {
            // Create ActiveMove from move ID for deduct_pp
            let move_id_to_deduct = caller_move_for_pressure.as_ref().map(|e| &e.id).unwrap_or(&move_data.id);
            if let Some(active_move_for_pp) = battle.dex.get_active_move(move_id_to_deduct.as_str()) {
                battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].deduct_pp(battle.gen, &active_move_for_pp, Some(extra_pp));
            }
        }
    }

    // if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
    //     !this.battle.runEvent('TryMove', pokemon, target, move)) {
    //     move.mindBlownRecoil = false;
    //     return false;
    // }
    // IMPORTANT: JavaScript uses short-circuit evaluation here!
    // If singleEvent returns a falsy value (null, false), runEvent is NEVER called.
    // This is critical for two-turn moves like Ice Burn where the move's onTryMove
    // returns null on the charge turn to prevent the move from executing - in that case,
    // we should NOT run other onTryMove callbacks (like Metronome's condition callback).
    let try_move_single = battle.single_event(
        "TryMove",
        &move_effect,
        None,
        Some(pokemon_pos),
        Some(final_target),
        Some(&move_effect),
        None,
    );

    // Check if singleEvent succeeded (is truthy)
    let try_move_single_success = !matches!(try_move_single, crate::event::EventResult::Boolean(false) |
                                                              crate::event::EventResult::Null |
                                                              crate::event::EventResult::Stop);

    // Only call runEvent if singleEvent succeeded - matching JavaScript short-circuit behavior
    let try_move_run = if try_move_single_success {
        battle.run_event(
            "TryMove",
            Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
            Some(final_target),
            Some(&move_effect),
            crate::event::EventResult::Number(1),
            false,
            false,
        ).is_truthy()
    } else {
        // singleEvent failed, so the overall result will be false anyway
        // We skip runEvent to match JavaScript's short-circuit behavior
        true
    };

    let try_move_success = try_move_single_success && try_move_run;

    if !try_move_success {
        if let Some(ref am) = battle.active_move {
            am.borrow_mut().mindblown_recoil = false;
        }
        debug_elog!("[USE_MOVE_INNER] TryMove failed, returning false");
        return false;
    }

    // this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
    battle.single_event(
        "UseMoveMessage",
        &move_effect,
        None,
        Some(pokemon_pos),
        Some(final_target),
        Some(&move_effect),
        None,
    );

    // if (move.ignoreImmunity === undefined) {
    //     move.ignoreImmunity = (move.category === 'Status');
    // }
    if let Some(ref am) = battle.active_move {
        let mut am_mut = am.borrow_mut();
        if am_mut.ignore_immunity.is_none() {
            if am_mut.category == "Status" {
                am_mut.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::All);
            }
        }
    }

    // if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.gen != 4 {
        if let Some(ref am) = battle.active_move {
            let am_borrowed = am.borrow();
            debug_elog!("[SELFDESTRUCT] Checking selfdestruct for move={}, self_destruct={:?}",
                am_borrowed.id, am_borrowed.self_destruct);
            if am_borrowed.self_destruct.as_deref() == Some("always") {
                debug_elog!("[SELFDESTRUCT] Calling battle.faint() for pokemon_pos={:?}", pokemon_pos);
                drop(am_borrowed);
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
        debug_elog!("[USE_MOVE_INNER] turn={}, move={}, taking tryMoveHit branch (target={})",
            battle.turn, active_move.id, active_move.target);
        // damage = this.tryMoveHit(targets, pokemon, move);
        let hit_result = crate::battle_actions::try_move_hit(
            battle,
            &targets,
            pokemon_pos,
            &active_move,
        );

        // if (damage === this.battle.NOT_FAIL) pokemon.moveThisTurnResult = null;
        // In JavaScript, NOT_FAIL is an empty string ""
        // Handle NOT_FAIL case - if move returned NOT_FAIL, clear moveThisTurnResult
        if hit_result.is_not_fail() {
            // Set pokemon.moveThisTurnResult to null
            if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon.move_this_turn_result = crate::battle_actions::MoveResult::Null;
            }
        }

        // if (damage || damage === 0 || damage === undefined) moveResult = true;
        // In JavaScript: damage (truthy), damage === 0, or damage === undefined all result in moveResult = true
        // Only damage === false or damage === '' (NOT_FAIL) result in moveResult staying false
        // DamageResult::Undefined corresponds to JS undefined, which makes moveResult = true
        // DamageResult::Damage(0) corresponds to JS 0, which makes moveResult = true
        // DamageResult::Success/Damage(>0) are truthy, moveResult = true
        // DamageResult::Failed corresponds to JS false, moveResult stays false
        // DamageResult::NotFail corresponds to JS '' (NOT_FAIL), moveResult stays false
        move_result = match hit_result {
            DamageResult::Undefined => DamageResult::Success,  // undefined in JS → moveResult = true
            DamageResult::Damage(0) => DamageResult::Success,  // 0 in JS → moveResult = true
            other => other,  // truthy values stay truthy, false/NOT_FAIL stay falsy
        };
    } else {
        debug_elog!("[USE_MOVE_INNER] turn={}, move={}, taking trySpreadMoveHit branch (target={})",
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
        // NOTE: Use local active_move for consistency with JS which uses local 'move' variable
        if battle.gen == 4 {
            if active_move.self_destruct.as_deref() == Some("always") {
                battle.faint(pokemon_pos, Some(pokemon_pos), Some(active_move.id.as_str()));
            }
        }

        // moveResult = this.trySpreadMoveHit(targets, pokemon, move);
        move_result = crate::battle_actions::try_spread_move_hit::try_spread_move_hit(
            battle,
            &targets,
            pokemon_pos,
            &mut active_move,
            false,
        );
    }

    // if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
    // NOTE: We must use the LOCAL active_move variable here, not battle.active_move!
    // In JS, "move" is a local variable that doesn't change even if useMove is called recursively
    // (e.g., via metronome's onHit calling useMove for the selected move).
    // If we read from battle.active_move, we'd get the wrong move's selfBoost.
    let self_boost = active_move.self_boost.clone();

    if let Some(ref boosts) = self_boost {
        if move_result.is_success() {
            // Apply self-boost using moveHit
            // moveHit(pokemon, pokemon, move, move.selfBoost, false, true)
            // Parameters: targets, pokemon, move, moveData, isSecondary, isSelf
            // Create a MoveSecondary containing just the boosts
            let self_boost_effect = crate::dex::MoveSecondary {
                boosts: Some(boosts.clone()),
                ..Default::default()
            };
            crate::battle_actions::move_hit(
                battle,
                &[Some(pokemon_pos)], // targets = [pokemon] (self-targeting)
                pokemon_pos,          // pokemon (user)
                &active_move,         // move - use local active_move, not battle.active_move
                Some(crate::battle_actions::HitEffect::Secondary(&self_boost_effect)),
                false,                // isSecondary
                true,                 // isSelf
            );
        }
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
            &move_effect,
            None,
            Some(final_target),
            Some(pokemon_pos),
            Some(&move_effect),
            None,
        );
        debug_elog!("[USE_MOVE_INNER] Move failed, returning false");
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
    // NOTE: Use local active_move, not battle.active_move (same reason as selfBoost)
    let has_futuremove = active_move.flags.future_move;

    if !(active_move.has_sheer_force && has_sheer_force) && !has_futuremove {
        let original_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].hp;

        battle.single_event(
            "AfterMoveSecondarySelf",
            &move_effect,
            None,
            Some(pokemon_pos),
            Some(final_target),
            Some(&move_effect),
            None,
        );

        battle.run_event("AfterMoveSecondarySelf", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(final_target), Some(&move_effect), EventResult::Continue, false, false);

        let current_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].hp;
        let max_hp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;

        if pokemon_pos != final_target && active_move.category != "Status" {
            if current_hp <= max_hp / 2 && original_hp > max_hp / 2 {
                battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(pokemon_pos), None, EventResult::Continue, false, false);
            }
        }
    }

    debug_elog!("[USE_MOVE_INNER] SUCCESS, returning true, PRNG={}", battle.prng.call_count);
    // return true;
    true
}

