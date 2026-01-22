//! Par Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::Arg;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     if (sourceEffect && sourceEffect.effectType === 'Ability') {
///         this.add('-status', target, 'par', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-status', target, 'par');
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // Get target ident
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // Check if sourceEffect is an ability
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // this.add('-status', target, 'par', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let source_ident = battle.active_pokemon
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-status",
            &[
                Arg::String(target_ident),
                Arg::Str("par"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-status', target, 'par');
        battle.add("-status", &[Arg::String(target_ident), Arg::Str("par")]);
    }

    EventResult::Continue
}

/// onModifySpe
/// JavaScript source (data/conditions.ts):
/// ```js
/// onModifySpePriority: -101,
/// onModifySpe(spe, pokemon) {
///     // Paralysis occurs after all other Speed modifiers, so evaluate all modifiers up to this point first
///     spe = this.finalModify(spe);
///     if (!pokemon.hasAbility('quickfeet')) {
///         spe = Math.floor(spe * 50 / 100);
///     }
///     return spe;
/// }
/// ```
pub fn on_modify_spe(
    battle: &mut Battle,
    spe: i32,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // spe = this.finalModify(spe);
    let spe = battle.final_modify(spe);

    // if (!pokemon.hasAbility('quickfeet'))
    let has_quickfeet = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["quickfeet"])
    };

    if !has_quickfeet {
        // spe = Math.floor(spe * 50 / 100);
        let modified_spe = (spe * 50 / 100) as i32;
        return EventResult::Number(modified_spe);
    }

    // return spe;
    EventResult::Number(spe)
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeMovePriority: 1,
/// onBeforeMove(pokemon) {
///     if (this.randomChance(1, 4)) {
///         this.add('cant', pokemon, 'par');
///         return false;
///     }
/// }
/// ```
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // if (this.randomChance(1, 4))
    let is_paralyzed = battle.random_chance(1.0, 4);

    if is_paralyzed {
        // this.add('cant', pokemon, 'par');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("cant", &[Arg::String(pokemon_ident), Arg::Str("par")]);

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

