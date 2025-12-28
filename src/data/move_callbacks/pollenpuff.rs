//! Pollen Puff Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (source.isAlly(target)) {
///         move.basePower = 0;
///         move.infiltrates = true;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let source = source_pos;
    let target = target_pos;

    // if (source.isAlly(target)) {
    let is_ally = battle.is_ally(source, target);

    if is_ally {
        // move.basePower = 0;
        // move.infiltrates = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.base_power = 0;
            active_move.infiltrates = true;
        }
    }

    EventResult::Continue
}

/// onTryMove(source, target, move) {
///     if (source.isAlly(target) && source.volatiles['healblock']) {
///         this.attrLastMove('[still]');
///         this.add('cant', source, 'move: Heal Block', move);
///         return false;
///     }
/// }
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.isAlly(target) && source.volatiles['healblock']) {
    let is_ally = battle.is_ally(source, target);

    if !is_ally {
        return EventResult::Continue;
    }

    let has_healblock = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon
            .volatiles
            .contains_key(&ID::from("healblock"))
    };

    if has_healblock {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.add('cant', source, 'move: Heal Block', move);
        let (source_arg, move_name) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_arg = source_pokemon.get_slot();

            let move_id = match &battle.active_move {
                Some(active_move) => active_move.id.clone(),
                None => return EventResult::Continue,
            };

            let move_data = battle.dex.get_move_by_id(&move_id);
            let move_name = move_data
                .map(|m| m.name.clone())
                .unwrap_or_else(|| move_id.to_string());

            (source_arg, move_name)
        };

        battle.add(
            "cant",
            &[
                source_arg.into(),
                "move: Heal Block".into(),
                move_name.into(),
            ],
        );

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     if (source.isAlly(target)) {
///         if (!this.heal(Math.floor(target.baseMaxhp * 0.5))) {
///             return this.NOT_FAIL;
///         }
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.isAlly(target)) {
    let is_ally = battle.is_ally(source, target);

    if is_ally {
        // if (!this.heal(Math.floor(target.baseMaxhp * 0.5))) {
        let heal_amount = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.base_maxhp / 2
        };

        let healed = battle.heal(heal_amount, Some(target), None, None);

        if healed.unwrap_or(0) == 0 {
            // return this.NOT_FAIL;
            return EventResult::NotFail;
        }
    }

    EventResult::Continue
}
