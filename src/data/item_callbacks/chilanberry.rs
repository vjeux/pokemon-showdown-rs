//! Chilan Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // if (
    //     move.type === 'Normal' &&
    //     (!target.volatiles['substitute'] || move.flags['bypasssub'] || (move.infiltrates && this.gen >= 6))
    // ) {
    //     if (target.eatItem()) {
    //         this.debug('-50% reduction');
    //         this.add('-enditem', target, this.effect, '[weaken]');
    //         return this.chainModify(0.5);
    //     }
    // }

    use crate::dex_data::ID;

    // Check if move.type === 'Normal'
    let (is_normal_type, bypass_sub, infiltrates) = match &battle.active_move {
        Some(active_move) => (
            active_move.move_type == "Normal",
            active_move.flags.bypasssub,
            active_move.infiltrates,
        ),
        None => return EventResult::Continue,
    };

    if !is_normal_type {
        return EventResult::Continue;
    }

    // Check if NOT hitting substitute OR bypassing substitute
    // (!target.volatiles['substitute'] || move.flags['bypasssub'] || (move.infiltrates && this.gen >= 6))
    let should_activate = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !target.volatiles.contains_key(&ID::from("substitute"))
            || bypass_sub
            || (infiltrates && battle.gen >= 6)
    };

    if !should_activate {
        return EventResult::Continue;
    }

    // if (target.eatItem())
    let item_eaten = {
        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_mut.eat_item(false)
    };

    if item_eaten.is_some() {
        // this.debug('-50% reduction');
        battle.debug("-50% reduction");

        // this.add('-enditem', target, this.effect, '[weaken]');
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };
        battle.add("-enditem", &[target_ident.as_str().into(), "Chilan Berry".into(), "[weaken]".into()]);

        // return this.chainModify(0.5);
        battle.chain_modify(0.5);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 200,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // onEat callback has no implementation - just metadata
    EventResult::Continue
}
