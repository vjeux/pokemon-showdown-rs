//! Charti Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.type === 'Rock' && target.getMoveHitData(move).typeMod > 0) {
///         const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///         if (hitSub) return;
/// 
///         if (target.eatItem()) {
///             this.debug('-50% reduction');
///             this.add('-enditem', target, this.effect, '[weaken]');
///             return this.chainModify(0.5);
///         }
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // if (move.type === 'Rock' && target.getMoveHitData(move).typeMod > 0) {
    //     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    //     if (hitSub) return;
    //
    //     if (target.eatItem()) {
    //         this.debug('-50% reduction');
    //         this.add('-enditem', target, this.effect, '[weaken]');
    //         return this.chainModify(0.5);
    //     }
    // }

    use crate::dex_data::ID;

    // Check if move.type === 'Rock'
    let (is_rock_type, bypass_sub, infiltrates) = match &battle.active_move {
        Some(active_move) => (
            active_move.move_type == "Rock",
            active_move.flags.bypasssub,
            active_move.infiltrates,
        ),
        None => return EventResult::Continue,
    };

    if !is_rock_type {
        return EventResult::Continue;
    }

    // Check type effectiveness against target (typeMod > 0 means super effective)
    let type_effectiveness = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.run_effectiveness("Rock")
    };

    if type_effectiveness <= 1.0 {
        return EventResult::Continue;
    }

    // const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    let hit_sub = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.volatiles.contains_key(&ID::from("substitute"))
            && !bypass_sub
            && !(infiltrates && battle.gen >= 6)
    };

    if hit_sub {
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
        battle.add("-enditem", &[target_ident.as_str().into(), "Charti Berry".into(), "[weaken]".into()]);

        // return this.chainModify(0.5);
        battle.chain_modify(0.5);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 195,
///     gen: 4,
/// }
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // onEat callback has no implementation - just metadata
    EventResult::Continue
}
