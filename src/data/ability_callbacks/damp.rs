//! Damp Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onAnyTryMove(target, source, effect) {
///     if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id)) {
///         this.attrLastMove('[still]');
///         this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
///         return false;
///     }
/// }
pub fn on_any_try_move(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
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
    // this.effectState.target is the Damp holder's position
    let damp_holder_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Boolean(false),
    };

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(false),
    };

    let (damp_holder_slot, target_slot) = {
        let damp_holder = match battle.pokemon_at(damp_holder_pos.0, damp_holder_pos.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        (damp_holder.get_slot(), target.get_slot())
    };

    // Get the move name for the message
    let effect_name = if let Some(move_data) = battle.dex.moves().get(effect) {
        move_data.name.clone()
    } else {
        effect.to_string()
    };

    battle.add("cant", &[
        Arg::String(damp_holder_slot),
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
pub fn on_any_damage(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // JavaScript checks effect.name === 'Aftermath', but in Rust we get the effect ID (lowercase)
    if let Some(effect) = effect_id {
        if effect == "aftermath" {
            return EventResult::Boolean(false);
        }
    }
    EventResult::Continue
}

