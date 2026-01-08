//! Sturdy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon, target, move) {
///     if (move.ohko) {
///         this.add('-immune', pokemon, '[from] ability: Sturdy');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::battle::Arg;

    // if (move.ohko)
    let has_ohko = {
        let move_data = match battle.dex.moves().get(move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        move_data.ohko.is_some()
    };

    if has_ohko {
        let target_id = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let side_id = format!("p{}", target.side_index + 1);
            if target.is_active {
                let pos_letter = (b'a' + target.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, target.name)
            } else {
                format!("{}: {}", side_id, target.name)
            }
        };

        // this.add('-immune', pokemon, '[from] ability: Sturdy');
        battle.add("-immune", &[
            Arg::String(target_id),
            Arg::Str("[from] ability: Sturdy"),
        ]);

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onDamage(damage, target, source, effect) {
///     if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move') {
///         this.add('-ability', target, 'Sturdy');
///         return target.hp - 1;
///     }
/// }
pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move')
    let (at_full_hp, current_hp, would_faint, is_move) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let at_full = target.hp == target.maxhp;
        let hp = target.hp;
        let would_faint = damage >= hp;

        // Check if effect is a move
        let is_move = if let Some(eff_id) = effect_id {
            battle.dex.moves().get(eff_id).is_some()
        } else {
            false
        };

        (at_full, hp, would_faint, is_move)
    };

    if at_full_hp && would_faint && is_move {
        let target_id = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let side_id = format!("p{}", target.side_index + 1);
            if target.is_active {
                let pos_letter = (b'a' + target.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, target.name)
            } else {
                format!("{}: {}", side_id, target.name)
            }
        };

        // this.add('-ability', target, 'Sturdy');
        battle.add("-ability", &[
            Arg::String(target_id),
            Arg::Str("Sturdy"),
        ]);

        // return target.hp - 1;
        return EventResult::Number(current_hp - 1);
    }

    EventResult::Continue
}

