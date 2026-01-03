//! Synchronize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterSetStatus(status, target, source, effect) {
///     if (!source || source === target) return;
///     if (effect && effect.id === 'toxicspikes') return;
///     if (status.id === 'slp' || status.id === 'frz') return;
///     this.add('-activate', target, 'ability: Synchronize');
///     // Hack to make status-prevention abilities think Synchronize is a status move
///     // and show messages when activating against it.
///     source.trySetStatus(status, target, { status: status.id, id: 'synchronize' } as Effect);
/// }
pub fn on_after_set_status(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;

    // if (!source || source === target) return;
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if source_pos == target_pos {
        return EventResult::Continue;
    }

    // if (effect && effect.id === 'toxicspikes') return;
    if effect_id == Some("toxicspikes") {
        return EventResult::Continue;
    }

    // if (status.id === 'slp' || status.id === 'frz') return;
    let status_id = match status {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    if status_id == "slp" || status_id == "frz" {
        return EventResult::Continue;
    }

    // this.add('-activate', target, 'ability: Synchronize');
    let target_slot = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(target_slot),
        Arg::Str("ability: Synchronize"),
    ]);

    // source.trySetStatus(status, target, { status: status.id, id: 'synchronize' } as Effect);
    // Hack to make status-prevention abilities think Synchronize is a status move
    // and show messages when activating against it.
    // Note: The source_effect parameter in Rust is currently not fully functional for custom Effect objects,
    // but we pass "synchronize" to at least identify the source
    Pokemon::try_set_status(battle, source_pos, crate::dex_data::ID::from(status_id), Some("synchronize"));

    EventResult::Continue
}

