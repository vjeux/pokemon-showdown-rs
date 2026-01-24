//! Charge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

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
        effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
        let is_special_ability = if let Some(eff) = effect {
            &*eff.name == "Electromorphosis" || &*eff.name == "Wind Power"
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
                Some(active_move) => active_move.borrow().name.clone(),
                None => "".to_string(),
            };

            let effect_name = effect.map(|e| &*e.name).unwrap_or("");
            let from_str = format!("[from] ability: {}", effect_name);
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
        effect: Option<&Effect>,
    ) -> EventResult {
        let effect_id = effect.map(|e| &*e.name);

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
                Some(active_move) => active_move.borrow().name.clone(),
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
        // JavaScript checks move.type (the active move's type, not the dex type)
        let is_electric = battle.active_move.as_ref()
            .map(|m| m.borrow().move_type == "Electric")
            .unwrap_or(false);

        // if (move.type === 'Electric') {
        if is_electric {
            // this.debug('charge boost');
            battle.debug("charge boost");

            // return this.chainModify(2);
            battle.chain_modify(2.0);
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
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // JavaScript checks move.type (the active move's type, not the dex type)
        let (is_electric, move_id) = match active_move {
            Some(m) => (m.move_type == "Electric", m.id.as_str()),
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric' && move.id !== 'charge') {
        if is_electric && move_id != "charge" {
            // pokemon.removeVolatile('charge');
            Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("charge"));
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
        // JavaScript checks move.type (the active move's type, not the dex type)
        let (is_electric, is_charge) = match &battle.active_move {
            Some(m) => (m.borrow().move_type == "Electric", m.borrow().id.as_str() == "charge"),
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric' && move.id !== 'charge') {
        if is_electric && !is_charge {
            // pokemon.removeVolatile('charge');
            Pokemon::remove_volatile(battle, source_pos, &ID::from("charge"));
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
