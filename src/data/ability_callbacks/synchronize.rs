//! Synchronize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_after_set_status(battle: &mut Battle, _status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
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

    // Get the status from the target Pokemon (since the status parameter isn't passed correctly)
    // JavaScript: status.id comes from the status that was just set on the target
    let status_id = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.status.clone()
    };

    // if (status.id === 'slp' || status.id === 'frz') return;
    if status_id.as_str() == "slp" || status_id.as_str() == "frz" {
        return EventResult::Continue;
    }

    // Synchronize shouldn't trigger if no status was set
    if status_id.as_str().is_empty() {
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
    // Note: target (Ralts with Synchronize) is passed as the source for the sync effect
    let sync_effect = crate::battle::Effect::ability("synchronize");
    Pokemon::try_set_status(battle, source_pos, crate::dex_data::ID::from(status_id.as_str()), Some(target_pos), Some(&sync_effect));

    EventResult::Continue
}

