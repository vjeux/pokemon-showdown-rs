//! Hadron Engine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (!this.field.setTerrain('electricterrain') && this.field.isTerrain('electricterrain')) {
///         this.add('-activate', pokemon, 'ability: Hadron Engine');
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.field.setTerrain('electricterrain') && this.field.isTerrain('electricterrain')) {
    //     this.add('-activate', pokemon, 'ability: Hadron Engine');
    // }

    // Try to set electric terrain (returns false if already set)
    let terrain_set = battle.field.set_terrain(crate::dex_data::ID::new("electricterrain"), None);

    // If terrain was already electric terrain (setTerrain returned false) and is still electric terrain
    if !terrain_set && battle.field.is_terrain("electricterrain") {
        // Get Pokemon identifier string before the mutable borrow
        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Format Pokemon ID like Display trait: "p1a: Pikachu"
            let side_id = format!("p{}", pokemon.side_index + 1);
            if pokemon.is_active {
                let pos_letter = (b'a' + pokemon.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, pokemon.name)
            } else {
                format!("{}: {}", side_id, pokemon.name)
            }
        };

        battle.add("-activate", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Hadron Engine"),
        ]);
    }

    EventResult::Continue
}

/// onModifySpA(atk, attacker, defender, move) {
///     if (this.field.isTerrain('electricterrain')) {
///         this.debug('Hadron Engine boost');
///         return this.chainModify([5461, 4096]);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (this.field.isTerrain('electricterrain')) {
    //     this.debug('Hadron Engine boost');
    //     return this.chainModify([5461, 4096]);
    // }

    if battle.field.is_terrain("electricterrain") {
        eprintln!("Hadron Engine boost");
        return EventResult::Number(battle.chain_modify_fraction(5461, 4096));
    }

    EventResult::Continue
}

