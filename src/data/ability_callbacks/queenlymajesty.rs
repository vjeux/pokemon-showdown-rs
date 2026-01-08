//! Queenly Majesty Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFoeTryMove(target, source, move) {
///     const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
///     if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
///         return;
///     }
///
///     const dazzlingHolder = this.effectState.target;
///     if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1) {
///         this.attrLastMove('[still]');
///         this.add('cant', dazzlingHolder, 'ability: Queenly Majesty', move, `[of] ${target}`);
///         return false;
///     }
/// }
pub fn on_foe_try_move(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::battle::Arg;

    // const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
    let target_all_exceptions = ["perishsong", "flowershield", "rototiller"];

    // if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id)))
    let (move_target, move_id, move_priority): (String, String, i8) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        (active_move.target.clone(), active_move.id.to_string(), active_move.priority)
    };

    if move_target == "foeSide" || (move_target == "all" && !target_all_exceptions.contains(&move_id.as_str())) {
        return EventResult::Continue;
    }

    // const dazzlingHolder = this.effectState.target;
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1)
    let source_is_ally = source_pos.map_or(false, |src_pos| battle.is_ally(src_pos, ability_holder));

    if (source_is_ally || move_target == "all") && move_priority as f32 > 0.1 {
        // this.attrLastMove('[still]');
        // Skip: attrLastMove is for animation and not critical for battle logic

        // this.add('cant', dazzlingHolder, 'ability: Queenly Majesty', move, `[of] ${target}`);
        let ability_slot = {
            let pokemon = match battle.pokemon_at(ability_holder.0, ability_holder.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            pokemon.get_slot()
        };

        let target_slot = if let Some(tgt_pos) = target_pos {
            let pokemon = match battle.pokemon_at(tgt_pos.0, tgt_pos.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            pokemon.get_slot()
        } else {
            String::new()
        };

        battle.add("cant", &[
            Arg::String(ability_slot),
            Arg::Str("ability: Queenly Majesty"),
            Arg::String(move_id.to_string()),
            Arg::String(format!("[of] {}", target_slot)),
        ]);

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

