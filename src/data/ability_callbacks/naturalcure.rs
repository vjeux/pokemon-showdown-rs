//! Natural Cure Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onCheckShow(pokemon) {
///     // This is complicated
///     // For the most part, in-game, it's obvious whether or not Natural Cure activated,
///     // since you can see how many of your opponent's pokemon are statused.
///     // The only ambiguous situation happens in Doubles/Triples, where multiple pokemon
///     // that could have Natural Cure switch out, but only some of them get cured.
///     if (pokemon.side.active.length === 1) return;
///     if (pokemon.showCure === true || pokemon.showCure === false) return;
/// 
///     const cureList = [];
///     let noCureCount = 0;
///     for (const curPoke of pokemon.side.active) {
///         // pokemon not statused
///         if (!curPoke?.status) {
///             // this.add('-message', "" + curPoke + " skipped: not statused or doesn't exist");
///             continue;
///         }
///         if (curPoke.showCure) {
///             // this.add('-message', "" + curPoke + " skipped: Natural Cure already known");
///             continue;
///         }
///         const species = curPoke.species;
///         // pokemon can't get Natural Cure
///         if (!Object.values(species.abilities).includes('Natural Cure')) {
///             // this.add('-message', "" + curPoke + " skipped: no Natural Cure");
///             continue;
///         }
///         // pokemon's ability is known to be Natural Cure
///         if (!species.abilities['1'] && !species.abilities['H']) {
///             // this.add('-message', "" + curPoke + " skipped: only one ability");
///             continue;
///         }
///         // pokemon isn't switching this turn
///         if (curPoke !== pokemon && !this.queue.willSwitch(curPoke)) {
///             // this.add('-message', "" + curPoke + " skipped: not switching");
///             continue;
///         }
/// 
///         if (curPoke.hasAbility('naturalcure')) {
///             // this.add('-message', "" + curPoke + " confirmed: could be Natural Cure (and is)");
///             cureList.push(curPoke);
///         } else {
///             // this.add('-message', "" + curPoke + " confirmed: could be Natural Cure (but isn't)");
///             noCureCount++;
///         }
///     }
/// 
///     if (!cureList.length || !noCureCount) {
///         // It's possible to know what pokemon were cured
///         for (const pkmn of cureList) {
///             pkmn.showCure = true;
///         }
///     } else {
///         // It's not possible to know what pokemon were cured
/// 
///         // Unlike a -hint, this is real information that battlers need, so we use a -message
///         this.add('-message', `(${cureList.length} of ${pokemon.side.name}'s pokemon ${cureList.length === 1 ? "was" : "were"} cured by Natural Cure.)`);
/// 
///         for (const pkmn of cureList) {
///             pkmn.showCure = false;
///         }
///     }
/// }
pub fn on_check_show(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSwitchOut(pokemon) {
///     if (!pokemon.status) return;
/// 
///     // if pokemon.showCure is undefined, it was skipped because its ability
///     // is known
///     if (pokemon.showCure === undefined) pokemon.showCure = true;
/// 
///     if (pokemon.showCure) this.add('-curestatus', pokemon, pokemon.status, '[from] ability: Natural Cure');
///     pokemon.clearStatus();
/// 
///     // only reset .showCure if it's false
///     // (once you know a Pokemon has Natural Cure, its cures are always known)
///     if (!pokemon.showCure) pokemon.showCure = undefined;
/// }
pub fn on_switch_out(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

