//! Raging Bull Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon) {
///     // will shatter screens through sub, before you hit
///     pokemon.side.removeSideCondition('reflect');
///     pokemon.side.removeSideCondition('lightscreen');
///     pokemon.side.removeSideCondition('auroraveil');
/// }
/// NOTE: dispatch_on_try_hit passes (target_pos, source_pos) per JS convention
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // pokemon.side.removeSideCondition('reflect');
    // pokemon.side.removeSideCondition('lightscreen');
    // pokemon.side.removeSideCondition('auroraveil');
    let target_side = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.side_index
    };

    battle.sides[target_side].remove_side_condition(&ID::from("reflect"));
    battle.sides[target_side].remove_side_condition(&ID::from("lightscreen"));
    battle.sides[target_side].remove_side_condition(&ID::from("auroraveil"));

    EventResult::Continue
}

/// onModifyType(move, pokemon) {
///     switch (pokemon.species_id.as_str()) {
///     case 'Tauros-Paldea-Combat':
///         move.type = 'Fighting';
///         break;
///     case 'Tauros-Paldea-Blaze':
///         move.type = 'Fire';
///         break;
///     case 'Tauros-Paldea-Aqua':
///         move.type = 'Water';
///         break;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // switch (pokemon.species_id.as_str()) {
    let species_id = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.species_id.to_string()
    };

    if let Some(ref mut active_move) = battle.active_move {
        match species_id.as_str() {
            // case 'Tauros-Paldea-Combat':
            //     move.type = 'Fighting';
            "taurospaldea" | "taurospaldeacombat" => {
                active_move.move_type = "Fighting".to_string();
            }
            // case 'Tauros-Paldea-Blaze':
            //     move.type = 'Fire';
            "taurospaldeablaze" => {
                active_move.move_type = "Fire".to_string();
            }
            // case 'Tauros-Paldea-Aqua':
            //     move.type = 'Water';
            "taurospaldeaaqua" => {
                active_move.move_type = "Water".to_string();
            }
            _ => {}
        }
    }

    EventResult::Continue
}
