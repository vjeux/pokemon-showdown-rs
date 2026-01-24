//! Air Balloon Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect, EffectType};
use crate::event::EventResult;

/// onStart(target) {
///     if (!target.ignoringItem() && !this.field.getPseudoWeather('gravity')) {
///         this.add('-item', target, 'Air Balloon');
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!target.ignoringItem() && !this.field.getPseudoWeather('gravity')) {
    //     this.add('-item', target, 'Air Balloon');
    // }

    use crate::dex_data::ID;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if target is ignoring item
    let ignoring_item = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.ignoring_item(battle, false)
    };

    // Check if gravity is active
    let has_gravity = battle.field.has_pseudo_weather(&ID::from("gravity"));

    // if (!target.ignoringItem() && !this.field.getPseudoWeather('gravity'))
    if !ignoring_item && !has_gravity {
        // this.add('-item', target, 'Air Balloon');
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };
        battle.add("-item", &[target_ident.as_str().into(), "Air Balloon".into()]);
    }

    EventResult::Continue
}

/// onDamagingHit(damage, target, source, move) {
///     this.add('-enditem', target, 'Air Balloon');
///     target.item = '';
///     this.clearEffectState(target.itemState);
///     this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // this.add('-enditem', target, 'Air Balloon');
    // target.item = '';
    // this.clearEffectState(target.itemState);
    // this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));

    use crate::dex_data::ID;

    // this.add('-enditem', target, 'Air Balloon');
    let target_ident = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };
    battle.add("-enditem", &[target_ident.as_str().into(), "Air Balloon".into()]);

    // target.item = '';
    // this.clearEffectState(target.itemState);
    {
        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_mut.item = ID::empty();
        // Clear item state inline (equivalent to battle.clearEffectState(target.itemState))
        target_mut.item_state.borrow_mut().id = ID::empty();
        target_mut.item_state.borrow_mut().effect_order = 0;
        // All typed fields on item_state will keep their values but that's fine
        // since the item is gone. The id being empty indicates no active item state.
    }

    // this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
    let item_effect = battle.make_item_effect(&ID::from("airballoon"));
    battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(target_pos)), None, Some(&item_effect), EventResult::Continue, false, false);

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, effect) {
///     this.debug('effect: ' + effect.id);
///     if (effect.effectType === 'Move') {
///         this.add('-enditem', target, 'Air Balloon');
///         target.item = '';
///         this.clearEffectState(target.itemState);
///         this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    // this.debug('effect: ' + effect.id);
    // if (effect.effectType === 'Move') {
    //     this.add('-enditem', target, 'Air Balloon');
    //     target.item = '';
    //     this.clearEffectState(target.itemState);
    //     this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
    // }

    use crate::dex_data::ID;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let effect = match effect {
        Some(e) => e,
        None => return EventResult::Continue,
    };

    // this.debug('effect: ' + effect.id);
    battle.debug(&format!("effect: {}", effect.id));

    // if (effect.effectType === 'Move')
    // Check if the effect type is Move (not just if a move with this ID exists!)
    let is_move = effect.effect_type == EffectType::Move;

    if is_move {
        // this.add('-enditem', target, 'Air Balloon');
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };
        battle.add("-enditem", &[target_ident.as_str().into(), "Air Balloon".into()]);

        // target.item = '';
        // this.clearEffectState(target.itemState);
        {
            let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_mut.item = ID::empty();
            // Clear item state inline (equivalent to battle.clearEffectState(target.itemState))
            target_mut.item_state.borrow_mut().id = ID::empty();
            target_mut.item_state.borrow_mut().effect_order = 0;
            // All typed fields on item_state will keep their values but that's fine
            // since the item is gone. The id being empty indicates no active item state.
        }

        // this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
        let item_effect = battle.make_item_effect(&ID::from("airballoon"));
        battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(target_pos)), None, Some(&item_effect), EventResult::Continue, false, false);
    }

    EventResult::Continue
}
