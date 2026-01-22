//! Aurora Veil Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
        _effect: Option<&Effect>,
    ) -> EventResult {
        // if (source?.hasItem('lightclay')) {
        //     return 8;
        // }
        if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };

            if source_pokemon.has_item(battle, &["lightclay"]) {
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
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        debug_elog!("[AURORAVEIL] on_any_modify_damage called: source={:?}, target={:?}, move={}", source_pos, target_pos, move_id);

        // Get target and source positions
        let target = match target_pos {
            Some(pos) => pos,
            None => {
                debug_elog!("[AURORAVEIL] No target_pos, returning Continue");
                return EventResult::Continue;
            }
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => {
                debug_elog!("[AURORAVEIL] No source_pos, returning Continue");
                return EventResult::Continue;
            }
        };

        // if (target !== source && this.effectState.target.hasAlly(target)) {
        if target == source {
            debug_elog!("[AURORAVEIL] target == source, returning Continue");
            return EventResult::Continue;
        }

        // Check if this Aurora Veil's effectState.target is an ally of the actual target
        // This ensures that when both sides have Aurora Veil, only the defending side's applies
        let effect_target = match battle.with_effect_state_ref(|state| state.target).flatten() {
            Some(t) => t,
            None => {
                debug_elog!("[AURORAVEIL] No effect_target, returning Continue");
                return EventResult::Continue;
            }
        };

        let has_ally = battle.is_ally(effect_target, target);
        if !has_ally {
            debug_elog!("[AURORAVEIL] effect_target {:?} is not ally of target {:?}, returning Continue", effect_target, target);
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

        let target_side_idx = target.0;
        let has_reflect = battle.sides[target_side_idx].has_side_condition(&reflect_id);
        let has_lightscreen = battle.sides[target_side_idx].has_side_condition(&lightscreen_id);

        debug_elog!("[AURORAVEIL] category={}, has_reflect={}, has_lightscreen={}", category, has_reflect, has_lightscreen);

        if (has_reflect && category == "Physical")
            || (has_lightscreen && category == "Special")
        {
            debug_elog!("[AURORAVEIL] Reflect/Lightscreen already active for this category, returning Continue");
            return EventResult::Continue;
        }

        // if (!target.getMoveHitData(move).crit && !move.infiltrates) {
        let infiltrates = battle
            .active_move
            .as_ref()
            .map(|m| m.infiltrates)
            .unwrap_or(false);

        // Check crit via getMoveHitData
        let crit = battle
            .get_move_hit_data(target)
            .map(|hit_data| hit_data.crit)
            .unwrap_or(false);

        debug_elog!("[AURORAVEIL] crit={}, infiltrates={}", crit, infiltrates);

        if crit || infiltrates {
            debug_elog!("[AURORAVEIL] Crit or infiltrates, returning Continue");
            return EventResult::Continue;
        }

        // this.debug('Aurora Veil weaken');
        battle.debug("Aurora Veil weaken");
        debug_elog!("[AURORAVEIL] Applying damage reduction, active_per_half={}", battle.active_per_half);

        // if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
        // return this.chainModify(0.5);
        if battle.active_per_half > 1 {
            debug_elog!("[AURORAVEIL] Calling chain_modify_fraction(2732, 4096)");
            battle.chain_modify_fraction(2732, 4096);
        } else {
            debug_elog!("[AURORAVEIL] Calling chain_modify(0.5)");
            battle.chain_modify(0.5);
        }

        EventResult::Continue
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Aurora Veil');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Aurora Veil');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Aurora Veil".into()]);
        }

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Aurora Veil');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'move: Aurora Veil');
        // Following the pattern from gmaxcannonade.rs - access side via with_effect_state_ref
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "move: Aurora Veil".into()]);
        }

        EventResult::Continue
    }
}
