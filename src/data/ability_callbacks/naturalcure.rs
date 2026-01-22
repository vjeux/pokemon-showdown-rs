//! Natural Cure Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_check_show(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.side.active.length === 1) return;
    let active_count = {
        if pokemon_pos.0 >= battle.sides.len() {
            return EventResult::Continue;
        }
        battle.sides[pokemon_pos.0].active.len()
    };

    if active_count == 1 {
        return EventResult::Continue;
    }

    // if (pokemon.showCure === true || pokemon.showCure === false) return;
    let show_cure = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.show_cure
    };

    if show_cure == Some(true) || show_cure == Some(false) {
        return EventResult::Continue;
    }

    // Collect cure list and no cure count
    let mut cure_list: Vec<(usize, usize)> = Vec::new();
    let mut no_cure_count = 0;

    // Get active pokemon positions for this side
    let active_positions: Vec<(usize, usize)> = {
        if pokemon_pos.0 >= battle.sides.len() {
            return EventResult::Continue;
        }

        battle.sides[pokemon_pos.0]
            .active
            .iter()
            .enumerate()
            .filter_map(|(idx, active)| {
                if let Some(slot) = active {
                    if *slot != 0 {
                        Some((pokemon_pos.0, idx))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    for cur_poke_pos in &active_positions {
        // if (!curPoke?.status) continue;
        let (has_status, cur_show_cure, species_id) = {
            let cur_poke = match battle.pokemon_at(cur_poke_pos.0, cur_poke_pos.1) {
                Some(p) => p,
                None => continue,
            };
            (!cur_poke.status.is_empty(), cur_poke.show_cure, cur_poke.species_id.clone())
        };

        if !has_status {
            continue;
        }

        // if (curPoke.showCure) continue;
        if cur_show_cure == Some(true) {
            continue;
        }

        // const species = curPoke.species;
        // if (!Object.values(species.abilities).includes('Natural Cure'))
        let species = match battle.dex.species().get(species_id.as_str()) {
            Some(s) => s,
            None => continue,
        };

        // Check if any ability slot has Natural Cure
        let has_natural_cure = [
            species.abilities.slot0.as_ref(),
            species.abilities.slot1.as_ref(),
            species.abilities.hidden.as_ref(),
            species.abilities.special.as_ref(),
        ]
        .iter()
        .filter_map(|s| *s)
        .any(|ability| ability == "Natural Cure");

        if !has_natural_cure {
            continue;
        }

        // if (!species.abilities['1'] && !species.abilities['H'])
        let has_multiple_abilities = species.abilities.slot1.is_some() || species.abilities.hidden.is_some();
        if !has_multiple_abilities {
            continue;
        }

        // if (curPoke !== pokemon && !this.queue.willSwitch(curPoke))
        if cur_poke_pos != &pokemon_pos {
            let will_switch = battle.queue.will_switch(cur_poke_pos.0, cur_poke_pos.1).is_some();
            if !will_switch {
                continue;
            }
        }

        // if (curPoke.hasAbility('naturalcure'))
        let has_ability = {
            let cur_poke = match battle.pokemon_at(cur_poke_pos.0, cur_poke_pos.1) {
                Some(p) => p,
                None => continue,
            };
            cur_poke.has_ability(battle, &["naturalcure"])
        };

        if has_ability {
            cure_list.push(*cur_poke_pos);
        } else {
            no_cure_count += 1;
        }
    }

    // if (!cureList.length || !noCureCount)
    if cure_list.is_empty() || no_cure_count == 0 {
        // It's possible to know what pokemon were cured
        // for (const pkmn of cureList) { pkmn.showCure = true; }
        for poke_pos in &cure_list {
            if let Some(pkmn) = battle.pokemon_at_mut(poke_pos.0, poke_pos.1) {
                pkmn.show_cure = Some(true);
            }
        }
    } else {
        // It's not possible to know what pokemon were cured
        // this.add('-message', `(${cureList.length} of ${pokemon.side.name}'s pokemon ${cureList.length === 1 ? "was" : "were"} cured by Natural Cure.)`);
        let side_name = {
            if pokemon_pos.0 >= battle.sides.len() {
                return EventResult::Continue;
            }
            battle.sides[pokemon_pos.0].name.clone()
        };

        let cure_count = cure_list.len();
        let verb = if cure_count == 1 { "was" } else { "were" };
        let message = format!("({} of {}'s pokemon {} cured by Natural Cure.)", cure_count, side_name, verb);

        battle.add("-message", &[message.into()]);

        // for (const pkmn of cureList) { pkmn.showCure = false; }
        for poke_pos in &cure_list {
            if let Some(pkmn) = battle.pokemon_at_mut(poke_pos.0, poke_pos.1) {
                pkmn.show_cure = Some(false);
            }
        }
    }

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
pub fn on_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!pokemon.status) return;
    let (has_status, show_cure, status_id) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (!pokemon.status.is_empty(), pokemon.show_cure, pokemon.status.clone())
    };

    if !has_status {
        return EventResult::Continue;
    }

    // if (pokemon.showCure === undefined) pokemon.showCure = true;
    let final_show_cure = if show_cure.is_none() {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.show_cure = Some(true);
        }
        Some(true)
    } else {
        show_cure
    };

    // if (pokemon.showCure) this.add('-curestatus', pokemon, pokemon.status, '[from] ability: Natural Cure');
    if final_show_cure == Some(true) && !status_id.is_empty() {
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-curestatus", &[
            pokemon_slot.into(),
            status_id.as_str().into(),
            "[from] ability: Natural Cure".into(),
        ]);
    }

    // pokemon.clearStatus();
    use crate::pokemon::Pokemon;
    Pokemon::clear_status(battle, pokemon_pos);

    // only reset .showCure if it's false
    // (once you know a Pokemon has Natural Cure, its cures are always known)
    // if (!pokemon.showCure) pokemon.showCure = undefined;
    if final_show_cure == Some(false) {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.show_cure = None;
        }
    }

    EventResult::Continue
}

