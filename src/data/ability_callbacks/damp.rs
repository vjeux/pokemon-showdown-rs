//! Damp Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyTryMove(target, source, effect) {
///     if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id)) {
///         this.attrLastMove('[still]');
///         this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
///         return false;
///     }
/// }
pub fn on_any_try_move(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id))
    let effect = match effect_id {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    let is_explosion_move = matches!(
        effect,
        "explosion" | "mindblown" | "mistyexplosion" | "selfdestruct"
    );

    if !is_explosion_move {
        return EventResult::Continue;
    }

    // this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
    // Note: effectState.target would be the Damp holder's position
    // For now, we need to get this from the current event context
    // This is a limitation - we'd need effectState.target infrastructure
    // Let's use target_pos as a temporary workaround since the signature provides it
    let target_slot = if let Some(pos) = target_pos {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        pokemon.get_slot()
    } else {
        return EventResult::Boolean(false);
    };

    // Get the move name for the message
    let effect_name = if let Some(move_data) = battle.dex.moves().get(effect) {
        move_data.name.clone()
    } else {
        effect.to_string()
    };

    // For now, we can't properly get effectState.target, so we'll show a simplified message
    // TODO: Once effectState infrastructure is available, use effectState.target for damp_holder
    battle.add("cant", &[
        Arg::String(target_slot.clone()),
        Arg::Str("ability: Damp"),
        Arg::String(effect_name),
        Arg::String(format!("[of] {}", target_slot)),
    ]);

    // return false;
    EventResult::Boolean(false)
}

/// onAnyDamage(damage, target, source, effect) {
///     if (effect && effect.name === 'Aftermath') {
///         return false;
///     }
/// }
pub fn on_any_damage(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if let Some(effect) = effect_id {
        if effect == "Aftermath" {
            return EventResult::Boolean(false);
        }
    }
    EventResult::Continue
}

