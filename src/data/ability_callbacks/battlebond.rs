//! Battle Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::EffectType;
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
pub fn on_source_after_faint(_battle: &mut Battle, _length: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
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
    let is_move_effect = effect
        .map(|e| e.effect_type == EffectType::Move)
        .unwrap_or(false);

    if !is_move_effect {
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
    // Note: Side.foe_pokemon_left() is a stub that always returns 0,
    // so we calculate it directly here using Battle context
    let foe_pokemon_left = {
        if source.0 >= _battle.sides.len() {
            return EventResult::Continue;
        }
        // For standard 2-player battles, foe is the other side
        let foe_idx = source.0 ^ 1;  // XOR with 1 to flip between 0 and 1
        if foe_idx < _battle.sides.len() {
            _battle.sides[foe_idx].pokemon_left
        } else {
            0
        }
    };

    if foe_pokemon_left == 0 {
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
pub fn on_modify_move(battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Use the passed active_move parameter (battle.active_move is None during dispatch)
    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if move is watershuriken
    if active_move.id.as_str() != "watershuriken" {
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
        active_move.multi_hit = Some(Multihit::Fixed(3));
    }

    EventResult::Continue
}
