//! BattleActions::hitStepBreakProtect - Break through protection moves
//!
//! 1:1 port of hitStepBreakProtect from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;

/// Break through protection moves like Protect, King's Shield, etc.
/// Equivalent to battle-actions.ts hitStepBreakProtect()
///
/// hitStepBreakProtect(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     if (move.breaksProtect) {
///         for (const target of targets) {
///             let broke = false;
///             for (const effectid of [
///                 'banefulbunker', 'burningbulwark', 'kingsshield', 'obstruct', 'protect', 'silktrap', 'spikyshield',
///             ]) {
///                 if (target.removeVolatile(effectid)) broke = true;
///             }
///             if (this.battle.gen >= 6 || !target.isAlly(pokemon)) {
///                 for (const effectid of ['craftyshield', 'matblock', 'quickguard', 'wideguard']) {
///                     if (target.side.removeSideCondition(effectid)) broke = true;
///                 }
///             }
///             if (broke) {
///                 if (move.id === 'feint') {
///                     this.battle.add('-activate', target, 'move: Feint');
///                 } else {
///                     this.battle.add('-activate', target, `move: ${move.name}`, '[broken]');
///                 }
///                 if (this.battle.gen >= 6) delete target.volatiles['stall'];
///             }
///         }
///     }
///     return undefined;
/// }
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn hit_step_break_protect(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    attacker_pos: (usize, usize),
    active_move: &ActiveMove,
) {
    // if (move.breaksProtect) {
    if !active_move.breaks_protect {
        return;
    }

    //     for (const target of targets) {
    for &target_pos in targets {
        //         let broke = false;
        let mut broke = false;

        //         for (const effectid of [
        //             'banefulbunker', 'burningbulwark', 'kingsshield', 'obstruct', 'protect', 'silktrap', 'spikyshield',
        //         ]) {
        //             if (target.removeVolatile(effectid)) broke = true;
        //         }
        let volatile_effects = [
            "banefulbunker",
            "burningbulwark",
            "kingsshield",
            "obstruct",
            "protect",
            "silktrap",
            "spikyshield",
        ];

        for &effectid in &volatile_effects {
            if Pokemon::remove_volatile(battle, target_pos, &ID::from(effectid)) {
                broke = true;
            }
        }

        //         if (this.battle.gen >= 6 || !target.isAlly(pokemon)) {
        //             for (const effectid of ['craftyshield', 'matblock', 'quickguard', 'wideguard']) {
        //                 if (target.side.removeSideCondition(effectid)) broke = true;
        //             }
        //         }
        let is_ally = {
            if let (Some(_target_pokemon), Some(_attacker_pokemon)) = (
                battle.pokemon_at(target_pos.0, target_pos.1),
                battle.pokemon_at(attacker_pos.0, attacker_pos.1),
            ) {
                target_pos.0 == attacker_pos.0 // Same side = ally
            } else {
                false
            }
        };

        if battle.gen >= 6 || !is_ally {
            let side_effects = ["craftyshield", "matblock", "quickguard", "wideguard"];
            for &effectid in &side_effects {
                if battle.sides[target_pos.0].remove_side_condition(&ID::from(effectid)) {
                    broke = true;
                }
            }
        }

        //         if (broke) {
        if broke {
            //             if (move.id === 'feint') {
            //                 this.battle.add('-activate', target, 'move: Feint');
            //             } else {
            //                 this.battle.add('-activate', target, `move: ${move.name}`, '[broken]');
            //             }
            if active_move.id.as_str() == "feint" {
                // Get target pokemon for add message
                if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                    let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                    battle.add("-activate", &[
                        crate::battle::Arg::String(target_ident),
                        crate::battle::Arg::Str("move: Feint"),
                    ]);
                }
            } else {
                if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                    let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                    let move_msg = format!("move: {}", active_move.name);
                    battle.add("-activate", &[
                        crate::battle::Arg::String(target_ident),
                        crate::battle::Arg::String(move_msg),
                        crate::battle::Arg::Str("[broken]"),
                    ]);
                }
            }

            //             if (this.battle.gen >= 6) delete target.volatiles['stall'];
            if battle.gen >= 6 {
                Pokemon::remove_volatile(battle, target_pos, &ID::from("stall"));
            }
        }
    }

    // return undefined;
}
