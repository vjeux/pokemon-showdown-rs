//! Ability Shield Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSetAbility(ability, target, source, effect) {
///     if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
///         this.add('-ability', source, effect);
///     }
///     this.add('-block', target, 'item: Ability Shield');
///     return null;
/// }
pub fn on_set_ability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace')
    if let Some(effect) = effect_id {
        // Check if effect is an ability by trying to get it from dex
        if let Some(ability_data) = battle.dex.get_ability(effect) {
            if ability_data.name != "Trace" {
                // this.add('-ability', source, effect);
                if let Some(src_pos) = source_pos {
                    let source_slot = {
                        let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        source_pokemon.get_slot()
                    };

                    battle.add(
                        "-ability",
                        &[
                            crate::battle::Arg::from(source_slot),
                            crate::battle::Arg::from(effect),
                        ],
                    );
                }
            }
        }
    }

    // this.add('-block', target, 'item: Ability Shield');
    if let Some(tgt_pos) = target_pos {
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(tgt_pos.0, tgt_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-block",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("item: Ability Shield"),
            ],
        );
    }

    // return null; - in JS, returning null prevents the ability change
    EventResult::Stop
}
