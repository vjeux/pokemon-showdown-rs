//! Sticky Hold Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, pokemon, source) {
///     if (!this.activeMove) throw new Error("Battle.activeMove is null");
///     if (!pokemon.hp || pokemon.item === 'stickybarb') return;
///     if ((source && source !== pokemon) || this.activeMove.id === 'knockoff') {
///         this.add('-activate', pokemon, 'ability: Sticky Hold');
///         return false;
///     }
/// }
pub fn on_take_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    use crate::battle::Arg;

    // if (!this.activeMove) throw new Error("Battle.activeMove is null");
    let active_move_id = match &battle.active_move {
        Some(m) => m.id.as_str(),
        None => return EventResult::Continue,
    };

    // if (!pokemon.hp || pokemon.item === 'stickybarb') return;
    let (has_hp, has_stickybarb) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let has_sticky = pokemon.item.as_str() == "stickybarb";
        (pokemon.hp > 0, has_sticky)
    };

    if !has_hp || has_stickybarb {
        return EventResult::Continue;
    }

    // if ((source && source !== pokemon) || this.activeMove.id === 'knockoff')
    let should_prevent = if let Some(src_pos) = source_pos {
        src_pos != pokemon_pos
    } else {
        false
    } || active_move_id == "knockoff";

    if should_prevent {
        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let side_id = format!("p{}", pokemon.side_index + 1);
            if pokemon.is_active {
                let pos_letter = (b'a' + pokemon.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, pokemon.name)
            } else {
                format!("{}: {}", side_id, pokemon.name)
            }
        };

        // this.add('-activate', pokemon, 'ability: Sticky Hold');
        battle.add("-activate", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Sticky Hold"),
        ]);

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
