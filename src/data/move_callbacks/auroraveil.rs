//! Aurora Veil Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onTry() {
///     return this.field.isWeather(['hail', 'snowscape']);
/// }
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // return this.field.isWeather(['hail', 'snowscape']);
    let weather_id = &battle.field.weather;
    let is_weather_valid = weather_id == &ID::from("hail") || weather_id == &ID::from("snowscape");

    EventResult::Boolean(is_weather_valid)
}

pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (source?.hasItem('lightclay')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // if (source?.hasItem('lightclay')) {
        //     return 8;
        // }
        if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };

            if source_pokemon.has_item(&["lightclay"]) {
                return EventResult::Number(8);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onAnyModifyDamage(damage, source, target, move) {
    ///     if (target !== source && this.effectState.target.hasAlly(target)) {
    ///         if ((target.side.getSideCondition('reflect') && this.getCategory(move) === 'Physical') ||
    ///             (target.side.getSideCondition('lightscreen') && this.getCategory(move) === 'Special')) {
    ///             return;
    ///         }
    ///         if (!target.getMoveHitData(move).crit && !move.infiltrates) {
    ///             this.debug('Aurora Veil weaken');
    ///             if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
    ///             return this.chainModify(0.5);
    ///         }
    ///     }
    /// }
    pub fn on_any_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        source_pos: Option<(usize, usize)>,
        target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // Get target and source positions
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target !== source && this.effectState.target.hasAlly(target)) {
        if target == source {
            return EventResult::Continue;
        }

        // Check if target's side has aurora veil
        let target_side_idx = target.0;
        let auroraveil_id = ID::from("auroraveil");
        if !battle.sides[target_side_idx].has_side_condition(&auroraveil_id) {
            return EventResult::Continue;
        }

        // if ((target.side.getSideCondition('reflect') && this.getCategory(move) === 'Physical') ||
        //     (target.side.getSideCondition('lightscreen') && this.getCategory(move) === 'Special')) {
        //     return;
        // }
        let move_id_obj = ID::from(move_id);
        let category = battle.get_category(&move_id_obj);

        let reflect_id = ID::from("reflect");
        let lightscreen_id = ID::from("lightscreen");

        if (battle.sides[target_side_idx].has_side_condition(&reflect_id) && category == "Physical")
            || (battle.sides[target_side_idx].has_side_condition(&lightscreen_id)
                && category == "Special")
        {
            return EventResult::Continue;
        }

        // if (!target.getMoveHitData(move).crit && !move.infiltrates) {
        // Note: getMoveHitData and infiltrates checks not yet implemented
        // For now, just apply the damage modification
        // TODO: Add crit and infiltrates checks when available

        // this.debug('Aurora Veil weaken');
        battle.debug("Aurora Veil weaken");

        // if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
        // return this.chainModify(0.5);
        if battle.active_per_half > 1 {
            battle.chain_modify_fraction(2732, 4096);
        } else {
            battle.chain_modify(0.5);
        }

        EventResult::Continue
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Aurora Veil');
    /// }
    pub fn on_side_start(_battle: &mut Battle) -> EventResult {
        // TODO: Side parameter missing from signature - should be (battle: &mut Battle, side_idx: usize)
        // For now, the event system should handle adding the side context
        // this.add('-sidestart', side, 'move: Aurora Veil');
        // Note: Cannot implement without side parameter
        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Aurora Veil');
    /// }
    pub fn on_side_end(_battle: &mut Battle) -> EventResult {
        // TODO: Side parameter missing from signature - should be (battle: &mut Battle, side_idx: usize)
        // For now, the event system should handle adding the side context
        // this.add('-sideend', side, 'move: Aurora Veil');
        // Note: Cannot implement without side parameter
        EventResult::Continue
    }
}
