//! Zen Mode Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Darmanitan' || pokemon.transformed) {
///         return;
///     }
///     if (pokemon.hp <= pokemon.maxhp / 2 && !['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
///         pokemon.addVolatile('zenmode');
///     } else if (pokemon.hp > pokemon.maxhp / 2 && ['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
///         pokemon.addVolatile('zenmode'); // in case of base Darmanitan-Zen
///         pokemon.removeVolatile('zenmode');
///     }
/// }
pub fn on_residual(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (!pokemon.volatiles['zenmode'] || !pokemon.hp) return;
///     pokemon.transformed = false;
///     delete pokemon.volatiles['zenmode'];
///     if (pokemon.species.baseSpecies === 'Darmanitan' && pokemon.species.battleOnly) {
///         pokemon.formeChange(pokemon.species.battleOnly as string, this.effect, false, '0', '[silent]');
///     }
/// }
pub fn on_end(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (!pokemon.species.name.includes('Galar')) {
    ///         if (pokemon.species.id !== 'darmanitanzen') pokemon.formeChange('Darmanitan-Zen');
    ///     } else {
    ///         if (pokemon.species.id !== 'darmanitangalarzen') pokemon.formeChange('Darmanitan-Galar-Zen');
    ///     }
    /// }
    pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     if (['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
    ///         pokemon.formeChange(pokemon.species.battleOnly as string);
    ///     }
    /// }
    pub fn on_end(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
