//! Instruct Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.lastMove || target.volatiles['dynamax']) return false;
///     const lastMove = target.lastMove;
///     const moveSlot = target.getMoveData(lastMove.id);
///     if (
///         lastMove.flags['failinstruct'] || lastMove.isZ || lastMove.isMax ||
///         lastMove.flags['charge'] || lastMove.flags['recharge'] ||
///         target.volatiles['beakblast'] || target.volatiles['focuspunch'] || target.volatiles['shelltrap'] ||
///         (moveSlot && moveSlot.pp <= 0)
///     ) {
///         return false;
///     }
///     this.add('-singleturn', target, 'move: Instruct', `[of] ${source}`);
///     this.queue.prioritizeAction(this.queue.resolveAction({
///         choice: 'move',
///         pokemon: target,
///         moveid: target.lastMove.id,
///         targetLoc: target.lastMoveTargetLoc!,
///     })[0] as MoveAction);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!target.lastMove || target.volatiles['dynamax']) return false;
    let (has_last_move, last_move_id, has_dynamax) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target_pokemon.last_move.is_some(),
            target_pokemon.last_move.clone(),
            target_pokemon.volatiles.contains_key(&ID::from("dynamax"))
        )
    };

    if !has_last_move || has_dynamax {
        return EventResult::Boolean(false);
    }

    let last_move_id = last_move_id.unwrap();

    // const lastMove = target.lastMove;
    // const moveSlot = target.getMoveData(lastMove.id);
    let (has_failinstruct, is_z, is_max, has_charge, has_recharge, move_slot_pp) = {
        let last_move = battle.dex.get_move_by_id(&last_move_id);
        let (has_failinstruct, is_z, is_max, has_charge, has_recharge) = match last_move {
            Some(m) => (
                m.flags.fail_instruct,
                m.is_z,
                m.is_max,
                m.flags.charge,
                m.flags.recharge
            ),
            None => (false, false, false, false, false),
        };

        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_slot_pp = target_pokemon.get_move_data(&last_move_id)
            .map(|slot| slot.pp);

        (has_failinstruct, is_z, is_max, has_charge, has_recharge, move_slot_pp)
    };

    // if (
    //     lastMove.flags['failinstruct'] || lastMove.isZ || lastMove.isMax ||
    //     lastMove.flags['charge'] || lastMove.flags['recharge'] ||
    //     target.volatiles['beakblast'] || target.volatiles['focuspunch'] || target.volatiles['shelltrap'] ||
    //     (moveSlot && moveSlot.pp <= 0)
    // ) {
    //     return false;
    // }
    let has_beakblast = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("beakblast"))
    };

    let has_focuspunch = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("focuspunch"))
    };

    let has_shelltrap = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("shelltrap"))
    };

    if has_failinstruct || is_z || is_max || has_charge || has_recharge ||
       has_beakblast || has_focuspunch || has_shelltrap ||
       (move_slot_pp.is_some() && move_slot_pp.unwrap() <= 0) {
        return EventResult::Boolean(false);
    }

    // this.add('-singleturn', target, 'move: Instruct', `[of] ${source}`);
    let target_arg = crate::battle::Arg::Pos(target.0, target.1);
    let source_str = format!("[of] p{}a: {}", source.0 + 1, {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        &source_pokemon.name
    });
    battle.add("-singleturn", &[target_arg, "move: Instruct".into(), source_str.into()]);

    // this.queue.prioritizeAction(this.queue.resolveAction({
    //     choice: 'move',
    //     pokemon: target,
    //     moveid: target.lastMove.id,
    //     targetLoc: target.lastMoveTargetLoc!,
    // })[0] as MoveAction);
    let target_loc = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.last_move_target_loc.unwrap_or(0)
    };

    battle.queue.prioritize_move(target, &last_move_id, target_loc);

    EventResult::Continue
}

