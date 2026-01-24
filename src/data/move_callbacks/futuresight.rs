//! Future Sight Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target) {
///     if (!target.side.addSlotCondition(target, 'futuremove')) return false;
///     Object.assign(target.side.slotConditions[target.position]['futuremove'], {
///         move: 'futuresight',
///         source,
///         moveData: {
///             id: 'futuresight',
///             name: "Future Sight",
///             accuracy: 100,
///             basePower: 120,
///             category: "Special",
///             priority: 0,
///             flags: { allyanim: 1, metronome: 1, futuremove: 1 },
///             ignoreImmunity: false,
///             effectType: 'Move',
///             type: 'Psychic',
///         },
///     });
///     this.add('-start', source, 'move: Future Sight');
///     return this.NOT_FAIL;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // Get source and target
    let source = source_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if we're executing a queued future move (not queuing a new one)
    // The futuremove::on_end_with_data sets this flag before executing the move
    // In that case, just return Continue to let the move deal damage normally
    if battle.executing_future_move {
        debug_elog!("[FUTURESIGHT::ON_TRY] executing_future_move is true, this is a future move execution - returning Continue to deal damage");
        return EventResult::Continue;
    }

    // if (!target.side.addSlotCondition(target, 'futuremove')) return false;
    // Try to add the futuremove condition
    let added = {
        let target_side_index = target.0;
        let target_position = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.position
        };

        if let Some(target_side) = battle.sides.get_mut(target_side_index) {
            target_side.add_slot_condition(target_position, ID::from("futuremove"), None)
        } else {
            false
        }
    };

    if !added {
        // addSlotCondition returned false - condition already exists, can't queue another
        return EventResult::Boolean(false);
    }

    // Manually call the condition's onStart since add_slot_condition doesn't do it
    // JavaScript: this.battle.singleEvent('Start', status, conditionState, this.active[target], source, sourceEffect)
    {
        use crate::data::condition_callbacks;
        condition_callbacks::futuremove::on_start(battle, target, Some(source), None);
    }

    // Object.assign(target.side.slotConditions[target.position]['futuremove'], {
    //     move: 'futuresight',
    //     source,
    //     moveData: { ... },
    // });
    {
        let target_side_index = target.0;
        let target_position = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.position
        };

        if let Some(target_side) = battle.sides.get_mut(target_side_index) {
            if let Some(slot_conditions) = target_side.slot_conditions.get_mut(target_position) {
                if let Some(future_move_condition) =
                    slot_conditions.get_mut(&ID::from("futuremove"))
                {
                    // Set move data
                    future_move_condition.borrow_mut().move_id = Some("futuresight".to_string());
                    future_move_condition.borrow_mut().source = Some(source);

                    // Set moveData object
                    let mut move_data_map = std::collections::HashMap::new();
                    move_data_map.insert(
                        "id".to_string(),
                        serde_json::Value::String("futuresight".to_string()),
                    );
                    move_data_map.insert(
                        "name".to_string(),
                        serde_json::Value::String("Future Sight".to_string()),
                    );
                    move_data_map.insert(
                        "accuracy".to_string(),
                        serde_json::Value::Number(100.into()),
                    );
                    move_data_map.insert(
                        "basePower".to_string(),
                        serde_json::Value::Number(120.into()),
                    );
                    move_data_map.insert(
                        "category".to_string(),
                        serde_json::Value::String("Special".to_string()),
                    );
                    move_data_map.insert("priority".to_string(), serde_json::Value::Number(0.into()));

                    let mut flags = serde_json::Map::new();
                    flags.insert("allyanim".to_string(), serde_json::Value::Number(1.into()));
                    flags.insert("metronome".to_string(), serde_json::Value::Number(1.into()));
                    flags.insert(
                        "futuremove".to_string(),
                        serde_json::Value::Number(1.into()),
                    );
                    move_data_map.insert("flags".to_string(), serde_json::Value::Object(flags));

                    move_data_map.insert("ignoreImmunity".to_string(), serde_json::Value::Bool(false));
                    move_data_map.insert(
                        "effectType".to_string(),
                        serde_json::Value::String("Move".to_string()),
                    );
                    move_data_map.insert(
                        "type".to_string(),
                        serde_json::Value::String("Psychic".to_string()),
                    );

                    future_move_condition.borrow_mut().move_data = Some(move_data_map);
                }
            }
        }
    }

    // this.add('-start', source, 'move: Future Sight');
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[source_ident.as_str().into(), "move: Future Sight".into()],
    );

    // return this.NOT_FAIL;
    // NOT_FAIL means the move succeeded but shouldn't execute normal damage/effects
    // The future move is queued to execute later
    EventResult::NotFail
}
