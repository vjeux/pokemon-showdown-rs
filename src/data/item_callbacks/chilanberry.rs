//! Chilan Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle_actions::ActiveMove;
use crate::event::EventResult;
use crate::Pokemon;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (
///         move.type === 'Normal' &&
///         (!target.volatiles['substitute'] || move.flags['bypasssub'] || (move.infiltrates && this.gen >= 6))
///     ) {
///         if (target.eatItem()) {
///             this.debug('-50% reduction');
///             this.add('-enditem', target, this.effect, '[weaken]');
///             return this.chainModify(0.5);
///         }
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), active_move: &ActiveMove) -> EventResult {
    use crate::dex_data::ID;

    if active_move.move_type != "Normal" {
        return EventResult::Continue;
    }

    // Check if NOT hitting substitute OR bypassing substitute
    let should_activate = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !target.volatiles.contains_key(&ID::from("substitute"))
            || active_move.flags.bypasssub
            || (active_move.infiltrates && battle.gen >= 6)
    };

    if !should_activate {
        return EventResult::Continue;
    }

    let item_eaten = {
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::eat_item(battle, target_pos, false, None, None)
    };

    if item_eaten.is_some() {
        battle.debug("-50% reduction");
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };
        battle.add("-enditem", &[target_ident.as_str().into(), "Chilan Berry".into(), "[weaken]".into()]);
        battle.chain_modify(0.5);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 200,
///     gen: 4,
/// }
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    EventResult::Continue
}
