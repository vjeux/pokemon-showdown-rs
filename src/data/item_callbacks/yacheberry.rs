//! Yache Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle_actions::ActiveMove;
use crate::event::EventResult;
use crate::Pokemon;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.type === 'Ice' && target.getMoveHitData(move).typeMod > 0) {
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
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), active_move: &ActiveMove) -> EventResult {
    use crate::dex_data::ID;

    if active_move.move_type != "Ice" {
        return EventResult::Continue;
    }

    let type_effectiveness = {
        let _target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::run_effectiveness(battle, target_pos, active_move)
    };

    if type_effectiveness <= 0 {
        return EventResult::Continue;
    }

    let hit_sub = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.volatiles.contains_key(&ID::from("substitute"))
            && !active_move.flags.bypasssub
            && !(active_move.infiltrates && battle.gen >= 6)
    };

    if hit_sub {
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
        battle.add("-enditem", &[target_ident.as_str().into(), "Yache Berry".into(), "[weaken]".into()]);
        battle.chain_modify(0.5);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 188,
///     gen: 4,
/// }
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    EventResult::Continue
}
