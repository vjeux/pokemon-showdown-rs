//! Fickle Beam Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (this.randomChance(3, 10)) {
///         this.attrLastMove('[anim] Fickle Beam All Out');
///         this.add('-activate', pokemon, 'move: Fickle Beam');
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (this.randomChance(3, 10)) {
    if battle.random_chance(3, 10) {
        // this.attrLastMove('[anim] Fickle Beam All Out');
        battle.attr_last_move(&["[anim] Fickle Beam All Out"]);

        // this.add('-activate', pokemon, 'move: Fickle Beam');
        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[pokemon_ident.as_str().into(), "move: Fickle Beam".into()],
        );

        // return this.chainModify(2);
        return EventResult::Number(battle.chain_modify(2.0_f32));
    }

    EventResult::Continue
}
