//! Power of Alchemy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onAllyFaint(target) {
///     if (!this.effectState.target.hp) return;
///     const ability = target.getAbility();
///     if (ability.flags['noreceiver'] || ability.id === 'noability') return;
///     this.effectState.target.setAbility(ability, target);
/// }
pub fn on_ally_faint(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    
    use crate::Pokemon;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!this.effectState.target.hp) return;
    let receiver_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let receiver_hp = {
        let receiver = match battle.pokemon_at(receiver_pos.0, receiver_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        receiver.hp
    };

    if receiver_hp == 0 {
        return EventResult::Continue;
    }

    // const ability = target.getAbility();
    let (target_ability_id, target_ability_noreceiver) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let ability_id = target.ability.clone();

        // Check if ability has noreceiver flag
        let noreceiver = battle.dex.abilities()
            .get_by_id(&ability_id)
            .and_then(|ability| ability.flags.get("noreceiver"))
            .map(|v| *v != 0)
            .unwrap_or(false);

        (ability_id, noreceiver)
    };

    // if (ability.flags['noreceiver'] || ability.id === 'noability') return;
    if target_ability_noreceiver || target_ability_id.as_str() == "noability" {
        return EventResult::Continue;
    }

    // this.effectState.target.setAbility(ability, target);
    Pokemon::set_ability(
        battle,
        receiver_pos,
        target_ability_id,
        Some(target_pos),
        None,
        false,
        false,
    );

    EventResult::Continue
}

