//! Toxic Debris Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     const side = source.isAlly(target) ? source.side.foe : source.side;
///     const toxicSpikes = side.sideConditions['toxicspikes'];
///     if (move.category === 'Physical' && (!toxicSpikes || toxicSpikes.layers < 2)) {
///         this.add('-activate', target, 'ability: Toxic Debris');
///         side.addSideCondition('toxicspikes', target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // Get target and source positions
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const side = source.isAlly(target) ? source.side.foe : source.side;
    let is_ally = battle.is_ally(source_pos, target_pos);
    let target_side_index = source_pos.0;

    // If source is ally to target, we need the foe side of source's side
    // Otherwise we need source's side
    let side_index = if is_ally {
        // Get foe index from source's side
        match battle.sides[source_pos.0].foe_index {
            Some(foe_idx) => foe_idx,
            None => return EventResult::Continue,
        }
    } else {
        source_pos.0
    };

    // const toxicSpikes = side.sideConditions['toxicspikes'];
    let toxic_spikes_layers = {
        let side = &battle.sides[side_index];
        let toxic_spikes = side.get_side_condition(&ID::from("toxicspikes"));
        toxic_spikes.and_then(|ts| ts.get_i32("layers")).unwrap_or(0)
    };

    // if (move.category === 'Physical' && (!toxicSpikes || toxicSpikes.layers < 2))
    let move_category = battle.active_move.as_ref().map(|m| m.category.clone()).unwrap_or_default();

    if move_category == "Physical" && toxic_spikes_layers < 2 {
        // this.add('-activate', target, 'ability: Toxic Debris');
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(target_slot),
            Arg::Str("ability: Toxic Debris"),
        ]);

        // side.addSideCondition('toxicspikes', target);
        battle.sides[side_index].add_side_condition(ID::from("toxicspikes"), None);
    }

    EventResult::Continue
}

