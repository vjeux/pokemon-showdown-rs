//! Perish Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
///     this.add('-ability', target, 'Perish Body');
///     source.addVolatile('perishsong');
///     target.addVolatile('perishsong');
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::battle::Arg;
    use crate::Pokemon;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!this.checkMoveMakesContact(move, source, target))
    let makes_contact = battle.check_move_makes_contact(&crate::dex_data::ID::from(move_id), source_pos, target_pos, false);
    if !makes_contact {
        return EventResult::Continue;
    }

    // || source.volatiles['perishsong'])
    let source_has_perishsong = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.volatiles.contains_key(&crate::dex_data::ID::from("perishsong"))
    };

    if source_has_perishsong {
        return EventResult::Continue;
    }

    // this.add('-ability', target, 'Perish Body');
    let target_slot = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-ability", &[
        Arg::String(target_slot),
        Arg::Str("Perish Body"),
    ]);

    // source.addVolatile('perishsong');
    Pokemon::add_volatile(battle, source_pos, crate::dex_data::ID::from("perishsong"), None, None, None, None);

    // target.addVolatile('perishsong');
    Pokemon::add_volatile(battle, target_pos, crate::dex_data::ID::from("perishsong"), None, None, None, None);

    EventResult::Continue
}

