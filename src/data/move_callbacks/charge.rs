//! Charge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
    ///         this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
    ///     } else {
    ///         this.add('-start', pokemon, 'Charge');
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
        let is_special_ability = if let Some(eid) = effect_id {
            eid == "Electromorphosis" || eid == "Wind Power"
        } else {
            false
        };

        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        if is_special_ability {
            // this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
            let active_move_name = match &battle.active_move {
                Some(active_move) => active_move.name.clone(),
                None => "".to_string(),
            };

            let from_str = format!("[from] ability: {}", effect_id.unwrap_or(""));
            battle.add(
                "-start",
                &[
                    pokemon_ident.as_str().into(),
                    "Charge".into(),
                    active_move_name.into(),
                    from_str.into(),
                ],
            );
        } else {
            // this.add('-start', pokemon, 'Charge');
            battle.add("-start", &[pokemon_ident.as_str().into(), "Charge".into()]);
        }

        EventResult::Continue
    }

    /// onRestart(pokemon, source, effect) {
    ///     if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
    ///         this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
    ///     } else {
    ///         this.add('-start', pokemon, 'Charge');
    ///     }
    /// }
    pub fn on_restart(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // Same logic as onStart
        // if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
        let is_special_ability = if let Some(eid) = effect_id {
            eid == "Electromorphosis" || eid == "Wind Power"
        } else {
            false
        };

        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        if is_special_ability {
            // this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
            let active_move_name = match &battle.active_move {
                Some(active_move) => active_move.name.clone(),
                None => "".to_string(),
            };

            let from_str = format!("[from] ability: {}", effect_id.unwrap_or(""));
            battle.add(
                "-start",
                &[
                    pokemon_ident.as_str().into(),
                    "Charge".into(),
                    active_move_name.into(),
                    from_str.into(),
                ],
            );
        } else {
            // this.add('-start', pokemon, 'Charge');
            battle.add("-start", &[pokemon_ident.as_str().into(), "Charge".into()]);
        }

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Electric') {
    ///         this.debug('charge boost');
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.get_move_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric') {
        if move_data.move_type == "Electric" {
            // this.debug('charge boost');
            battle.debug("charge boost");

            // return this.chainModify(2);
            let result = battle.chain_modify(2.0);
            return EventResult::Number(result);
        }

        EventResult::Continue
    }

    /// onMoveAborted(pokemon, target, move) {
    ///     if (move.type === 'Electric' && move.id !== 'charge') {
    ///         pokemon.removeVolatile('charge');
    ///     }
    /// }
    pub fn on_move_aborted(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // Get the move data
        let move_data = match battle.dex.get_move_by_id(&ID::from(move_id)) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric' && move.id !== 'charge') {
        if move_data.move_type == "Electric" && move_id != "charge" {
            // pokemon.removeVolatile('charge');
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.remove_volatile(&ID::from("charge"));
        }

        EventResult::Continue
    }

    /// onAfterMove(pokemon, target, move) {
    ///     if (move.type === 'Electric' && move.id !== 'charge') {
    ///         pokemon.removeVolatile('charge');
    ///     }
    /// }
    pub fn on_after_move(
        battle: &mut Battle,
        source_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.get_move_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric' && move.id !== 'charge') {
        if move_data.move_type == "Electric" && move_id.as_str() != "charge" {
            // pokemon.removeVolatile('charge');
            let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.remove_volatile(&ID::from("charge"));
        }

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Charge', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-end', pokemon, 'Charge', '[silent]');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                pokemon_ident.as_str().into(),
                "Charge".into(),
                "[silent]".into(),
            ],
        );

        EventResult::Continue
    }
}
