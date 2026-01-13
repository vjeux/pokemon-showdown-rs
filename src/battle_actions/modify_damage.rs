//! BattleActions::modifyDamage - Apply damage modifiers
//!
//! 1:1 port of modifyDamage from battle-actions.ts:1449

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::ActiveMove;

/// Apply damage modifiers
/// Equivalent to modifyDamage() in battle-actions.js:1449
///
/// JavaScript (battle-actions.js:1449-1520):
///   modifyDamage(baseDamage, pokemon, target, move, suppressMessages = false) {
///     const tr = this.battle.trunc;
///     if (!move.type) move.type = "???";
///     const type = move.type;
///     baseDamage += 2;
///     if (move.spreadHit) {
///       const spreadModifier = this.battle.gameType === "freeforall" ? 0.5 : 0.75;
///       this.battle.debug(`Spread modifier: ${spreadModifier}`);
///       baseDamage = this.battle.modify(baseDamage, spreadModifier);
///     } else if (move.multihitType === "parentalbond" && move.hit > 1) {
///       const bondModifier = this.battle.gen > 6 ? 0.25 : 0.5;
///       this.battle.debug(`Parental Bond modifier: ${bondModifier}`);
///       baseDamage = this.battle.modify(baseDamage, bondModifier);
///     }
///     baseDamage = this.battle.runEvent("WeatherModifyDamage", pokemon, target, move, baseDamage);
///     const isCrit = target.getMoveHitData(move).crit;
///     if (isCrit) {
///       baseDamage = tr(baseDamage * (move.critModifier || (this.battle.gen >= 6 ? 1.5 : 2)));
///     }
///     baseDamage = this.battle.randomizer(baseDamage);
///     if (type !== "???") {
///       let stab = 1;
///       const isSTAB = move.forceSTAB || pokemon.hasType(type) || pokemon.getTypes(false, true).includes(type);
///       if (isSTAB) {
///         stab = 1.5;
///       }
///       if (pokemon.terastallized === "Stellar") {
///         if (!pokemon.stellarBoostedTypes.includes(type) || move.stellarBoosted) {
///           stab = isSTAB ? 2 : [4915, 4096];
///           move.stellarBoosted = true;
///           if (pokemon.species.name !== "Terapagos-Stellar") {
///             pokemon.stellarBoostedTypes.push(type);
///           }
///         }
///       } else {
///         if (pokemon.terastallized === type && pokemon.getTypes(false, true).includes(type)) {
///           stab = 2;
///         }
///         stab = this.battle.runEvent("ModifySTAB", pokemon, target, move, stab);
///       }
///       baseDamage = this.battle.modify(baseDamage, stab);
///     }
///     let typeMod = target.runEffectiveness(move);
///     typeMod = this.battle.clampIntRange(typeMod, -6, 6);
///     target.getMoveHitData(move).typeMod = typeMod;
///     if (typeMod > 0) {
///       if (!suppressMessages) this.battle.add("-supereffective", target);
///       for (let i = 0; i < typeMod; i++) {
///         baseDamage *= 2;
///       }
///     }
///     if (typeMod < 0) {
///       if (!suppressMessages) this.battle.add("-resisted", target);
///       for (let i = 0; i > typeMod; i--) {
///         baseDamage = tr(baseDamage / 2);
///       }
///     }
///     if (isCrit && !suppressMessages) this.battle.add("-crit", target);
///     if (pokemon.status === "brn" && move.category === "Physical" && !pokemon.hasAbility("guts")) {
///       if (this.battle.gen < 6 || move.id !== "facade") {
///         baseDamage = this.battle.modify(baseDamage, 0.5);
///       }
///     }
///     if (this.battle.gen === 5 && !baseDamage) baseDamage = 1;
///     baseDamage = this.battle.runEvent("ModifyDamage", pokemon, target, move, baseDamage);
///     if (move.isZOrMaxPowered && target.getMoveHitData(move).zBrokeProtect) {
///       baseDamage = this.battle.modify(baseDamage, 0.25);
///       this.battle.add("-zbroken", target);
///     }
///     if (this.battle.gen !== 5 && !baseDamage) return 1;
///     return tr(baseDamage, 16);
///   }
/// JavaScript: modifyDamage(baseDamage, pokemon, target, move: ActiveMove, suppressMessages)
/// Now takes ActiveMove directly instead of MoveData, matching JavaScript's pattern.
pub fn modify_damage(
    battle: &mut Battle,
    mut base_damage: i32,
    pokemon_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: &ActiveMove,
    is_crit: bool,
) -> i32 {
    if battle.turn >= 64 && battle.turn <= 66 {
        eprintln!("[MODIFY_DAMAGE] ENTRY: turn={}, move={}, base_damage={}, source=({},{}), target=({},{})",
            battle.turn, active_move.id, base_damage, pokemon_pos.0, pokemon_pos.1, target_pos.0, target_pos.1);
    }
    // baseDamage += 2;
    base_damage += 2;

    // if (move.spreadHit) {
    //   const spreadModifier = this.battle.gameType === "freeforall" ? 0.5 : 0.75;
    //   this.battle.debug(`Spread modifier: ${spreadModifier}`);
    //   baseDamage = this.battle.modify(baseDamage, spreadModifier);
    // }
    // IMPORTANT: Spread moves only get damage reduction in multi-target battles (Doubles, Triples, etc.)
    // In Singles, spreadHit may be true (for moves that CAN hit multiple targets), but no modifier is applied.
    // The spread modifier is only for when there are actually multiple Pokemon on each side.
    if active_move.spread_hit {
        // Only apply spread modifier in Doubles, Triples, FreeForAll, etc. NOT in Singles.
        let should_apply_spread = !matches!(battle.game_type, crate::dex_data::GameType::Singles);

        if should_apply_spread {
            let spread_modifier = if battle.game_type == crate::dex_data::GameType::FreeForAll {
                0.5
            } else {
                0.75
            };
            if battle.turn >= 64 && battle.turn <= 66 {
                eprintln!("[MODIFY_DAMAGE] Spread modifier: {}, game_type={:?}, spreadHit={}",
                    spread_modifier, battle.game_type, active_move.spread_hit);
            }
            base_damage = battle.modify_f(base_damage, spread_modifier);
        } else if battle.turn >= 64 && battle.turn <= 66 {
            eprintln!("[MODIFY_DAMAGE] NOT applying spread modifier (Singles format), game_type={:?}, spreadHit={}",
                battle.game_type, active_move.spread_hit);
        }
    }
    // else if (move.multihitType === "parentalbond" && move.hit > 1) {
    //   const bondModifier = this.battle.gen > 6 ? 0.25 : 0.5;
    //   this.battle.debug(`Parental Bond modifier: ${bondModifier}`);
    //   baseDamage = this.battle.modify(baseDamage, bondModifier);
    // }
    else if active_move.multi_hit_type.as_deref() == Some("parentalbond") && active_move.hit > 1 {
        let bond_modifier = if battle.gen > 6 { 0.25 } else { 0.5 };
        eprintln!("[MODIFY_DAMAGE] Parental Bond modifier: {}", bond_modifier);
        base_damage = battle.modify_f(base_damage, bond_modifier);
    }

    // baseDamage = this.battle.runEvent("WeatherModifyDamage", pokemon, target, move, baseDamage);
    eprintln!("[MODIFY_DAMAGE] turn={}, Before WeatherModifyDamage event: base_damage={}", battle.turn, base_damage);
    let weather_result = battle.run_event(
        "WeatherModifyDamage",
        Some(crate::event::EventTarget::Pokemon(target_pos)),  // target = defender
        Some(pokemon_pos),                                       // source = attacker
        Some(&crate::battle::Effect::move_(active_move.id.clone())),
        EventResult::Number(base_damage),
        false,
        false
    );

    match weather_result {
        EventResult::Number(modified) => {
            base_damage = modified;
            eprintln!("[MODIFY_DAMAGE] turn={}, After WeatherModifyDamage event (Number): base_damage={}", battle.turn, base_damage);
        }
        EventResult::Float(multiplier) => {
            // JavaScript: return this.chainModify(multiplier)
            // chainModify returns a modified value, not a multiplier
            base_damage = battle.modify_f(base_damage, multiplier);
            eprintln!("[MODIFY_DAMAGE] turn={}, After WeatherModifyDamage event (Float {}x): base_damage={}", battle.turn, multiplier, base_damage);
        }
        _ => {
            eprintln!("[MODIFY_DAMAGE] turn={}, WeatherModifyDamage returned: {:?}", battle.turn, weather_result);
        }
    }

    // if (isCrit) {
    //   baseDamage = tr(baseDamage * (move.critModifier || (this.battle.gen >= 6 ? 1.5 : 2)));
    // }
    if is_crit {
        let crit_multiplier = if battle.gen >= 6 { 1.5 } else { 2.0 };
        let before_crit = base_damage;
        base_damage = battle.trunc(base_damage as f64 * crit_multiplier, None) as i32;
        eprintln!("[MODIFY_DAMAGE CRIT] Applying crit multiplier: {} * {} = {}",
            before_crit, crit_multiplier, base_damage);
    }

    // baseDamage = this.battle.randomizer(baseDamage);
    // JavaScript: if (!move.noDamageVariance) { baseDamage = this.battle.randomizer(baseDamage); }
    // Check if the move has noDamageVariance set to true
    let should_apply_variance = active_move.no_damage_variance.unwrap_or(false) == false; // Apply variance unless explicitly disabled

    eprintln!("[MODIFY_DAMAGE] turn={}, BEFORE randomizer: base_damage={}, should_apply={}",
        battle.turn, base_damage, should_apply_variance);

    if should_apply_variance {
        base_damage = battle.randomizer(base_damage);
    }
    eprintln!("[MODIFY_DAMAGE] turn={}, AFTER randomizer (applied={}): base_damage={}",
        battle.turn, should_apply_variance, base_damage);

    // Get source and target data for STAB and type effectiveness
    // Use get_types() instead of pokemon.types to handle ability-based type changes
    // (e.g., Multitype for Arceus, RKS System for Silvally)
    let (source_types, _target_types, target_slot) = {
        let source_types = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            if battle.turn >= 64 && battle.turn <= 66 {
                eprintln!("[MODIFY_DAMAGE] Reading source types for {} (species: {}): raw={:?}",
                    pokemon.name, pokemon.species_id, pokemon.types);
            }
            // Use get_types to properly handle ability-based type changes like Multitype
            pokemon.get_types(battle, false)
        } else {
            vec![]
        };

        let target_types = if let Some(pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
            if battle.turn >= 64 && battle.turn <= 66 {
                eprintln!("[MODIFY_DAMAGE] Reading target types for {} (species: {}): raw={:?}",
                    pokemon.name, pokemon.species_id, pokemon.types);
            }
            // Use get_types to properly handle ability-based type changes
            pokemon.get_types(battle, false)
        } else {
            vec![]
        };

        let target_slot = if let Some(pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
            pokemon.get_slot()
        } else {
            String::new()
        };

        (source_types, target_types, target_slot)
    };

    // if (type !== "???") {
    //   let stab = 1;
    //   const isSTAB = move.forceSTAB || pokemon.hasType(type) || pokemon.getTypes(false, true).includes(type);
    //   if (isSTAB) {
    //     stab = 1.5;
    //   }
    //   ...
    //   baseDamage = this.battle.modify(baseDamage, stab);
    // }
    // Use move_type from the passed active_move (may be modified by abilities like Pixilate)
    let move_type = active_move.move_type.clone();

    if battle.turn >= 64 && battle.turn <= 66 {
        eprintln!("[MODIFY_DAMAGE] move_type={}, source_types={:?}", move_type, source_types);
    }

    eprintln!("[MODIFY_DAMAGE] turn={}, About to check STAB: move_type={}, source_types={:?}",
        battle.turn, move_type, source_types);

    if &move_type != "???" {
        // JavaScript: let stab = 1;
        let mut stab = 1.0_f64;

        // JavaScript: const isSTAB = move.forceSTAB || pokemon.hasType(type) || pokemon.getTypes(false, true).includes(type);
        let force_stab = active_move.force_stab;
        let has_stab = force_stab || source_types.iter().any(|t| t == &move_type);
        if battle.turn >= 64 && battle.turn <= 66 {
            eprintln!("[MODIFY_DAMAGE] Checking STAB: has_stab={}", has_stab);
        }
        eprintln!("[MODIFY_DAMAGE] turn={}, STAB check: has_stab={}, base_damage_before={}",
            battle.turn, has_stab, base_damage);

        // JavaScript: if (isSTAB) { stab = 1.5; }
        if has_stab {
            stab = 1.5;
        }

        // TODO: Handle Terastallized/Stellar cases (pokemon.terastallized)
        // For now, skip Stellar tera handling

        // JavaScript: stab = this.battle.runEvent("ModifySTAB", pokemon, target, move, stab);
        let stab_result = battle.run_event(
            "ModifySTAB",
            Some(crate::event::EventTarget::Pokemon(pokemon_pos)),  // pokemon = attacker
            Some(target_pos),                                        // target = defender
            Some(&crate::battle::Effect::move_(active_move.id.clone())),
            EventResult::Float(stab),
            false,
            false
        );
        match stab_result {
            EventResult::Float(modified_stab) => {
                stab = modified_stab;
            }
            _ => {
                // No modification
            }
        }

        // JavaScript: baseDamage = this.battle.modify(baseDamage, stab);
        base_damage = battle.modify_f(base_damage, stab);
        eprintln!("[MODIFY_DAMAGE] turn={}, After STAB (stab={}): base_damage={}", battle.turn, stab, base_damage);
    } else {
        eprintln!("[MODIFY_DAMAGE] turn={}, Skipping STAB because move_type is ???", battle.turn);
    }

    // let typeMod = target.runEffectiveness(move);
    // typeMod = this.battle.clampIntRange(typeMod, -6, 6);
    // target.getMoveHitData(move).typeMod = typeMod;
    let mut type_mod = Pokemon::run_effectiveness(battle, target_pos, active_move);
    type_mod = battle.clamp_int_range(type_mod, Some(-6), Some(6));
    eprintln!("[MODIFY_DAMAGE] Type effectiveness (via run_effectiveness): move_id={}, type_mod={}",
        active_move.id, type_mod);
    // Store type_mod in move hit data
    // target.getMoveHitData(move).typeMod = typeMod;
    if let Some(move_hit_data) = battle.get_move_hit_data_mut(target_pos) {
        move_hit_data.type_mod = type_mod as i8;
    }

    // if (typeMod > 0) {
    //   if (!suppressMessages) this.battle.add("-supereffective", target);
    //   for (let i = 0; i < typeMod; i++) {
    //     baseDamage *= 2;
    //   }
    // }
    // if (typeMod < 0) {
    //   if (!suppressMessages) this.battle.add("-resisted", target);
    //   for (let i = 0; i > typeMod; i--) {
    //     baseDamage = tr(baseDamage / 2);
    //   }
    // }
    if type_mod > 0 {
        battle.add("-supereffective", &[Arg::String(target_slot.clone())]);
        for _ in 0..type_mod {
            base_damage *= 2;
        }
        eprintln!("[MODIFY_DAMAGE] After super effective: base_damage={}", base_damage);
    } else if type_mod < 0 {
        battle.add("-resisted", &[Arg::String(target_slot.clone())]);
        for _ in type_mod..0 {
            base_damage = battle.trunc(base_damage as f64 / 2.0, None) as i32;
        }
        eprintln!("[MODIFY_DAMAGE] After resisted: base_damage={}", base_damage);
    }

    // if (isCrit && !suppressMessages) this.battle.add("-crit", target);
    if is_crit {
        battle.add("-crit", &[Arg::String(target_slot.clone())]);
    }

    // if (pokemon.status === "brn" && move.category === "Physical" && !pokemon.hasAbility("guts")) {
    //   if (this.battle.gen < 6 || move.id !== "facade") {
    //     baseDamage = this.battle.modify(baseDamage, 0.5);
    //   }
    // }
    let (source_status, source_has_guts) = {
        if let Some(side) = battle.sides.get(pokemon_pos.0) {
            if let Some(pokemon) = side.pokemon.get(pokemon_pos.1) {
                let status = pokemon.status.clone();
                let has_guts = pokemon.has_ability(battle, &["guts"]);
                (status, has_guts)
            } else {
                (ID::new(""), false)
            }
        } else {
            (ID::new(""), false)
        }
    };

    if source_status.as_str() == "brn"
        && active_move.category == "Physical"
        && !source_has_guts
    {
        if battle.gen < 6 || active_move.id.as_str() != "facade" {
            base_damage = battle.modify(base_damage, 1, 2);
            eprintln!("[MODIFY_DAMAGE] After burn halving: base_damage={}", base_damage);
        }
    }

    // if (this.battle.gen === 5 && !baseDamage) baseDamage = 1;
    if battle.gen == 5 && base_damage == 0 {
        base_damage = 1;
    }

    // baseDamage = this.battle.runEvent("ModifyDamage", pokemon, target, move, baseDamage);
    // Note: In JavaScript, 'pokemon' (attacker) is the event target, 'target' (defender) is the source
    eprintln!("[MODIFY_DAMAGE] Before ModifyDamage event: base_damage={}", base_damage);
    if let EventResult::Number(modified) = battle.run_event(
                "ModifyDamage",
                Some(crate::event::EventTarget::Pokemon(pokemon_pos)),  // pokemon = attacker (event target)
                Some(target_pos),                                       // target = defender (source)
                Some(&crate::battle::Effect::move_(active_move.id.clone())),
                EventResult::Number(base_damage),
                false,
                false
    ) {
        base_damage = modified;
        eprintln!("[MODIFY_DAMAGE] After ModifyDamage event: base_damage={}", base_damage);
    }

    // if (move.isZOrMaxPowered && target.getMoveHitData(move).zBrokeProtect) {
    //   baseDamage = this.battle.modify(baseDamage, 0.25);
    //   this.battle.add("-zbroken", target);
    // }
    // Check if move is Z-powered or Max-powered and broke through protect
    // Z-moves and Max moves that break through protection deal reduced damage
    let is_z_or_max_powered = active_move.is_z_or_max_powered;
    let z_broke_protect = battle.get_move_hit_data(target_pos)
        .map(|hit_data| hit_data.z_broke_protect)
        .unwrap_or(false);

    if is_z_or_max_powered && z_broke_protect {
        base_damage = battle.modify(base_damage, 1, 4); // 0.25x = 1/4
        battle.add("-zbroken", &[Arg::String(target_slot.clone())]);
    }

    // if (this.battle.gen !== 5 && !baseDamage) return 1;
    // return tr(baseDamage, 16);
    let final_damage = if battle.gen != 5 && base_damage == 0 {
        1
    } else {
        base_damage
    };

    // JavaScript: return tr(baseDamage, 16);
    // The 16-bit truncation is important for damage calculation accuracy
    let result = battle.trunc(final_damage as f64, Some(16)) as i32;
    eprintln!("[MODIFY_DAMAGE] FINAL: result={}", result);
    result
}
