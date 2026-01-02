//! Sticky Web Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Sticky Web');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Sticky Web');
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Sticky Web".into()]);
        }

        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (!pokemon.isGrounded() || pokemon.hasItem('heavydutyboots')) return;
    ///     this.add('-activate', pokemon, 'move: Sticky Web');
    ///     this.boost({ spe: -1 }, pokemon, pokemon.side.foe.active[0], this.dex.getActiveMove('stickyweb'));
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {

        let pokemon = pokemon_pos;

        // if (!pokemon.isGrounded() || pokemon.hasItem('heavydutyboots')) return;
        let (is_grounded, has_heavy_duty_boots) = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (
                pokemon_ref.is_grounded(battle),
                pokemon_ref.has_item(battle, &["heavydutyboots"]),
            )
        };

        if !is_grounded || has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // this.add('-activate', pokemon, 'move: Sticky Web');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-activate",
            &[pokemon_arg.into(), "move: Sticky Web".into()],
        );

        // this.boost({ spe: -1 }, pokemon, pokemon.side.foe.active[0], this.dex.getActiveMove('stickyweb'));
        battle.boost(&[("spe", -1)], pokemon, None, Some("stickyweb"), false, false);

        EventResult::Continue
    }
}
