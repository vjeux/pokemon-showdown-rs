//! Battle Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex::Multihit;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (source.bondTriggered) return;
///     if (effect?.effectType !== 'Move') return;
///     if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft()) {
///         this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
///         this.add('-activate', source, 'ability: Battle Bond');
///         source.bondTriggered = true;
///     }
/// }
pub fn on_source_after_faint(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.bondTriggered) return;
    let bond_triggered = {
        let source_pokemon = match _battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.bond_triggered
    };

    if bond_triggered {
        return EventResult::Continue;
    }

    // if (effect?.effectType !== 'Move') return;
    let is_move = if let Some(eff_id) = effect_id {
        _battle.dex.moves().get_by_id(&crate::dex_data::ID::from(eff_id)).is_some()
    } else {
        false
    };

    if !is_move {
        return EventResult::Continue;
    }

    // if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft())
    let (is_greninja_bond, has_hp, not_transformed) = {
        let source_pokemon = match _battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            source_pokemon.species_id.as_str() == "greninjabond",
            source_pokemon.hp > 0,
            !source_pokemon.transformed,
        )
    };

    if !is_greninja_bond || !has_hp || !not_transformed {
        return EventResult::Continue;
    }

    // Check if foes have Pokemon left
    let foe_pokemon_left = {
        if source.0 >= _battle.sides.len() {
            return EventResult::Continue;
        }
        _battle.sides[source.0].foe_pokemon_left() > 0
    };

    if !foe_pokemon_left {
        return EventResult::Continue;
    }

    // this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
    _battle.boost(
        &[("atk", 1), ("spa", 1), ("spe", 1)],
        source,
        Some(source),
        None,
        false,
        false,
    );

    // this.add('-activate', source, 'ability: Battle Bond');
    let source_slot = {
        let source_pokemon = match _battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };

    _battle.add("-activate", &[
        source_slot.into(),
        "ability: Battle Bond".into(),
    ]);

    // source.bondTriggered = true;
    if let Some(source_pokemon) = _battle.pokemon_at_mut(source.0, source.1) {
        source_pokemon.bond_triggered = true;
    }

    EventResult::Continue
}

/// onModifyMove(move, attacker) {
///     if (move.id === 'watershuriken' && attacker.species.name === 'Greninja-Ash' &&
///         !attacker.transformed) {
///         move.multihit = 3;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Check if move is watershuriken
    let is_water_shuriken = if let Some(ref active_move) = battle.active_move {
        active_move.id.as_str() == "watershuriken"
    } else {
        return EventResult::Continue;
    };

    if !is_water_shuriken {
        return EventResult::Continue;
    }

    // if (attacker.species.name === 'Greninja-Ash' && !attacker.transformed)
    let (species_name, transformed) = {
        let attacker = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_data = match battle.dex.species().get(attacker.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (species_data.name.clone(), attacker.transformed)
    };

    if species_name == "Greninja-Ash" && !transformed {
        // move.multihit = 3;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.multi_hit = Some(Multihit::Fixed(3));
        }
    }

    EventResult::Continue
}

