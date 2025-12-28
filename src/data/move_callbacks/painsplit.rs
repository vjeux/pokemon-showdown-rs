//! Pain Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, pokemon) {
///     const targetHP = target.getUndynamaxedHP();
///     const averagehp = Math.floor((targetHP + pokemon.hp) / 2) || 1;
///     const targetChange = targetHP - averagehp;
///     target.sethp(target.hp - targetChange);
///     this.add('-sethp', target, target.getHealth, '[from] move: Pain Split', '[silent]');
///     pokemon.sethp(averagehp);
///     this.add('-sethp', pokemon, pokemon.getHealth, '[from] move: Pain Split');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetHP = target.getUndynamaxedHP();
    let target_hp = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.max_hp_undynamaxed
    };

    // const averagehp = Math.floor((targetHP + pokemon.hp) / 2) || 1;
    let average_hp = {
        let pokemon_hp = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.hp
        };

        let avg = (target_hp + pokemon_hp) / 2;
        if avg == 0 { 1 } else { avg }
    };

    // const targetChange = targetHP - averagehp;
    let target_change = target_hp - average_hp;

    // target.sethp(target.hp - targetChange);
    {
        let new_hp = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.hp - target_change
        };
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.set_hp(new_hp);
    }

    // this.add('-sethp', target, target.getHealth, '[from] move: Pain Split', '[silent]');
    {
        let (target_arg, target_health) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.get_slot(), target_pokemon.get_health())
        };

        battle.add("-sethp", &[
            target_arg,
            target_health.into(),
            "[from] move: Pain Split".into(),
            "[silent]".into(),
        ]);
    }

    // pokemon.sethp(averagehp);
    {
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.set_hp(average_hp);
    }

    // this.add('-sethp', pokemon, pokemon.getHealth, '[from] move: Pain Split');
    {
        let (pokemon_arg, pokemon_health) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (pokemon_pokemon.get_slot(), pokemon_pokemon.get_health())
        };

        battle.add("-sethp", &[
            pokemon_arg,
            pokemon_health.into(),
            "[from] move: Pain Split".into(),
        ]);
    }

    EventResult::Continue
}

