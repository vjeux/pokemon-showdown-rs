//! Healreplacement Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// healreplacement: {
///     onStart(target, source, sourceEffect) {
///         this.effectState.sourceEffect = sourceEffect;
///         this.add('-activate', source, 'healreplacement');
///     },
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // this.effectState.sourceEffect = sourceEffect;
    // For slot conditions, the source effect should be stored in the slot condition's effect state
    // Get the sourceEffect from active_move which triggered this
    if let Some(ref active_move) = battle.active_move {
        let source_effect = crate::battle::Effect::move_(active_move.id.clone());

        // Store sourceEffect in the slot condition's effect state
        let pokemon_position = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].position;
        let healreplacement_id = ID::from("healreplacement");

        if let Some(condition_state) = battle.sides[pokemon_pos.0]
            .slot_conditions
            .get_mut(pokemon_position)
            .and_then(|slot_conds| slot_conds.get_mut(&healreplacement_id))
        {
            condition_state.borrow_mut().source_effect = Some(source_effect);
        }
    }

    // this.add('-activate', source, 'healreplacement');
    // The source is the Pokemon that used the Z-move
    let source_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-activate",
        &[
            Arg::String(source_ident),
            Arg::Str("healreplacement"),
        ],
    );

    EventResult::Continue
}

/// onSwitchIn
/// JavaScript source (data/conditions.ts):
/// ```js
/// healreplacement: {
///     onSwitchIn(target) {
///         if (!target.fainted) {
///             target.heal(target.maxhp);
///             this.add('-heal', target, target.getHealth, '[from] move: ' + this.effectState.sourceEffect, '[zeffect]');
///             target.side.removeSlotCondition(target, 'healreplacement');
///         }
///     },
/// }
/// ```
pub fn on_switch_in(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (!target.fainted) {
    let is_fainted = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.is_fainted()
    };

    if is_fainted {
        return EventResult::Continue;
    }

    // Get sourceEffect from slot condition effect state
    let pokemon_position = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].position;
    let healreplacement_id = ID::from("healreplacement");

    let source_effect = battle.sides[pokemon_pos.0]
        .slot_conditions
        .get(pokemon_position)
        .and_then(|slot_conds| slot_conds.get(&healreplacement_id))
        .map(|state| state.borrow().source_effect.as_ref().map(|eff| eff.as_str().to_string()))
        .flatten()
        .unwrap_or_else(|| "Z-Move".to_string());

    // target.heal(target.maxhp);
    let maxhp = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].maxhp;
    
    battle.heal(maxhp, Some(pokemon_pos), Some(pokemon_pos), Some(&Effect::condition(ID::from("zpower"))));

    // this.add('-heal', target, target.getHealth, '[from] move: ' + this.effectState.sourceEffect, '[zeffect]');
    // Note: The heal method already adds the -heal message, but we need to add the [from] and [zeffect] attributes
    // Actually, looking at the JavaScript, heal() is called which adds the message
    // But then add() is called again with specific attributes
    // In Rust, heal() adds a basic -heal message, so we may need to modify it or add another message
    // For now, I'll add the message after heal completes
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    let hp_display = {
        let pokemon = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1).unwrap();
        format!("{}/{}", pokemon.hp, pokemon.maxhp)
    };

    // The heal() method already added a message, but we need to replace it with one that includes [from] and [zeffect]
    // Actually, let me check if heal() can accept these parameters...
    // Looking at the heal signature, it doesn't support custom attributes
    // So we need to pop the last -heal message and add our own
    // Or better yet, we can just add a second message with the full details
    // Actually, the JavaScript target.getHealth returns the HP display string
    // Let me add the message with the correct format
    battle.add(
        "-heal",
        &[
            Arg::String(pokemon_ident),
            Arg::String(hp_display),
            Arg::String(format!("[from] move: {}", source_effect)),
            Arg::Str("[zeffect]"),
        ],
    );

    // target.side.removeSlotCondition(target, 'healreplacement');
    battle.sides[pokemon_pos.0]
        .slot_conditions
        .get_mut(pokemon_position)
        .map(|slot_conds| slot_conds.remove(&healreplacement_id));

    EventResult::Continue
}

