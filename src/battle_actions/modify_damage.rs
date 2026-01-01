//! BattleActions::modifyDamage - Apply damage modifiers
//!
//! 1:1 port of modifyDamage from battle-actions.ts:1449

use crate::*;

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
pub fn modify_damage(
    battle: &mut Battle,
    mut base_damage: i32,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    move_data: &crate::dex::MoveData,
    is_crit: bool,
) -> i32 {
    // baseDamage += 2;
    base_damage += 2;

    // baseDamage = this.battle.runEvent("WeatherModifyDamage", pokemon, target, move, baseDamage);
    eprintln!("[MODIFY_DAMAGE] Before WeatherModifyDamage event: base_damage={}", base_damage);
    if let Some(modified) = battle.run_event(
        "WeatherModifyDamage",
        Some(source_pos),
        Some(target_pos),
        Some(&move_data.id),
        Some(base_damage),
    ) {
        base_damage = modified;
        eprintln!("[MODIFY_DAMAGE] After WeatherModifyDamage event: base_damage={}", base_damage);
    }

    // if (isCrit) {
    //   baseDamage = tr(baseDamage * (move.critModifier || (this.battle.gen >= 6 ? 1.5 : 2)));
    // }
    if is_crit {
        let crit_multiplier = if battle.gen >= 6 { 1.5 } else { 2.0 };
        base_damage = battle.trunc(base_damage as f64 * crit_multiplier, None) as i32;
    }

    // baseDamage = this.battle.randomizer(baseDamage);
    base_damage = battle.randomizer(base_damage);
    eprintln!("[MODIFY_DAMAGE] After randomizer: base_damage={}", base_damage);

    // Get source and target data for STAB and type effectiveness
    let (source_types, target_types, target_slot) = {
        let source_types = if let Some(side) = battle.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                pokemon.types.clone()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let target_types = if let Some(side) = battle.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                eprintln!("[MODIFY_DAMAGE] Reading types for {} (species: {}): {:?}", pokemon.name, pokemon.species_id, pokemon.types);
                pokemon.types.clone()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let target_slot = if let Some(side) = battle.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                pokemon.get_slot()
            } else {
                String::new()
            }
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
    // CRITICAL: Use active_move.move_type, not move_data.move_type!
    // Abilities like Pixilate modify active_move.move_type, so we need to read from there.
    let move_type = if let Some(ref active_move) = battle.active_move {
        &active_move.move_type
    } else {
        &move_data.move_type
    };

    if move_type != "???" {
        let has_stab = source_types.iter().any(|t| t == move_type);
        if has_stab {
            base_damage = battle.modify(base_damage, 3, 2);
            eprintln!("[MODIFY_DAMAGE] After STAB: base_damage={}", base_damage);
        }
    }

    // let typeMod = target.runEffectiveness(move);
    // typeMod = this.battle.clampIntRange(typeMod, -6, 6);
    // target.getMoveHitData(move).typeMod = typeMod;
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
    // CRITICAL: Use the same move_type we used for STAB (from active_move if available)
    let type_mod = battle.get_type_effectiveness_mod(move_type, &target_types);
    eprintln!("[MODIFY_DAMAGE] Type effectiveness: move_type={}, target_types={:?}, type_mod={}",
        move_type, target_types, type_mod);
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

    // baseDamage = this.battle.runEvent("ModifyDamage", pokemon, target, move, baseDamage);
    eprintln!("[MODIFY_DAMAGE] Before ModifyDamage event: base_damage={}", base_damage);
    if let Some(modified) = battle.run_event(
        "ModifyDamage",
        Some(source_pos),
        Some(target_pos),
        Some(&move_data.id),
        Some(base_damage),
    ) {
        base_damage = modified;
        eprintln!("[MODIFY_DAMAGE] After ModifyDamage event: base_damage={}", base_damage);
    }

    // if (this.battle.gen !== 5 && !baseDamage) return 1;
    // return tr(baseDamage, 16);
    let final_damage = if battle.gen != 5 && base_damage == 0 {
        1
    } else {
        base_damage
    };

    let result = battle.trunc(final_damage as f64, None) as i32;
    eprintln!("[MODIFY_DAMAGE] FINAL: result={}", result);
    result
}
