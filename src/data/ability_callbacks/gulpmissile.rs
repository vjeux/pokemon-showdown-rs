//! Gulp Missile Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (!source.hp || !source.isActive || target.isSemiInvulnerable()) return;
///     if (['cramorantgulping', 'cramorantgorging'].includes(target.species.id)) {
///         this.damage(source.baseMaxhp / 4, source, target);
///         if (target.species.id === 'cramorantgulping') {
///             this.boost({ def: -1 }, source, target, null, true);
///         } else {
///             source.trySetStatus('par', target, move);
///         }
///         target.formeChange('cramorant', move);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let move_id = active_move.as_ref().map(|m| m.id.to_string()).unwrap_or_default();
    // Get source and target positions
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source.hp || !source.isActive || target.isSemiInvulnerable()) return;
    let (source_hp, source_active) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source.hp, source.is_active)
    };

    let target_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, target_pos);

    if source_hp == 0 || !source_active || target_semi_invulnerable {
        return EventResult::Continue;
    }

    // if (['cramorantgulping', 'cramorantgorging'].includes(target.species.id))
    let (is_gulping, is_gorging) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // Use species_id (ID form) not set.species (display name)
        // JavaScript: target.species.id
        let species_id = target.species_id.as_str();
        (species_id == "cramorantgulping", species_id == "cramorantgorging")
    };

    if !is_gulping && !is_gorging {
        return EventResult::Continue;
    }

    // this.damage(source.baseMaxhp / 4, source, target);
    let damage_amount = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(source.base_maxhp, 4)
    };

    battle.damage(damage_amount, Some(source_pos), Some(target_pos), None, false);

    // if (target.species.id === 'cramorantgulping')
    if is_gulping {
        // this.boost({ def: -1 }, source, target, null, true);
        battle.boost(
            &[("def", -1)],
            source_pos,
            Some(target_pos),
            None,
            true,
            false,
        );
    } else {
        // source.trySetStatus('par', target, move);
        // JavaScript: source = the attacker (being paralyzed), target = Cramorant (causing the paralysis)
        // For try_set_status: first param is pokemon getting status, source_pos is who caused it
        let move_effect = battle.make_move_effect(&ID::from(move_id.as_str()));
        Pokemon::try_set_status(battle, source_pos, ID::from("par"), Some(target_pos), Some(&move_effect));
    }

    // target.formeChange('cramorant', move);
    // target_pos is already (side_idx, pokemon_index), pass it directly
    let move_effect = battle.make_move_effect(&ID::from(move_id.as_str()));
    crate::pokemon::Pokemon::forme_change(battle, target_pos, ID::from("cramorant"), Some(move_effect), false, "0", None);

    EventResult::Continue
}

/// onSourceTryPrimaryHit(target, source, effect) {
///     if (effect?.id === 'surf' && source.hasAbility('gulpmissile') && source.species.name === 'Cramorant') {
///         const forme = source.hp <= source.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
///         source.formeChange(forme, effect);
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // Get source position
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get effect id
    let effect_id = match effect_id {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // if (effect?.id === 'surf' && source.hasAbility('gulpmissile') && source.species.name === 'Cramorant')
    if effect_id != "surf" {
        return EventResult::Continue;
    }

    let (has_ability, hp, maxhp, is_cramorant) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_ability = source.has_ability(battle, &["gulpmissile"]);

        // Check if current species is Cramorant (base form)
        // JavaScript: source.species.name === 'Cramorant'
        // Use species_id (current species after forme changes), not set.species (original from team)
        let species_data = battle.dex.species().get(source.species_id.as_str());
        let is_cramorant = species_data.map(|s| s.name.as_str() == "Cramorant").unwrap_or(false);

        (has_ability, source.hp, source.maxhp, is_cramorant)
    };

    if !has_ability || !is_cramorant {
        return EventResult::Continue;
    }

    // const forme = source.hp <= source.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
    let forme = if hp <= maxhp / 2 {
        "cramorantgorging"
    } else {
        "cramorantgulping"
    };

    // source.formeChange(forme, effect);
    // source_pos is already (side_idx, pokemon_index), pass it directly
    let move_effect = battle.make_move_effect(&ID::from(effect_id));
    crate::pokemon::Pokemon::forme_change(battle, source_pos, ID::from(forme), Some(move_effect), false, "0", None);

    EventResult::Continue
}
