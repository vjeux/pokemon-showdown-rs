// 1:1 port of findSideEventHandlers from battle.ts

use crate::*;
use crate::battle::{Effect, EventListener, EffectType};

impl Battle {
    /// Find side event handlers
    /// Equivalent to battle.ts findSideEventHandlers()
    ///
    // JS Source:
    // 	findSideEventHandlers(side: Side, callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    //
    // 		for (const id in side.sideConditions) {
    // 			const sideConditionData = side.sideConditions[id];
    // 			const sideCondition = this.dex.conditions.getByID(id as ID);
    // 			const callback = this.getCallback(side, sideCondition, callbackName);
    // 			if (callback !== undefined || (getKey && sideConditionData[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: sideCondition, callback, state: sideConditionData,
    // 					end: customHolder ? null : side.removeSideCondition, effectHolder: customHolder || side,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		return handlers;
    // 	}
    pub fn find_side_event_handlers(
        &self,
        callback_name: &str,
        side_idx: usize,
        get_key: Option<&str>,
        custom_holder: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();

        // Get side reference
        let side = match self.sides.get(side_idx) {
            Some(s) => s,
            None => {
                eprintln!("[FIND_SIDE_HANDLERS] Side {} not found", side_idx);
                return handlers;
            }
        };

        eprintln!("[FIND_SIDE_HANDLERS] side_idx={}, callback_name={}, side_conditions count={}",
            side_idx, callback_name, side.side_conditions.len());

        for (sc_id, _) in &side.side_conditions {
            eprintln!("[FIND_SIDE_HANDLERS]   side condition: {}", sc_id.as_str());
        }

        // JS: for (const id in side.sideConditions) {
        for (sc_id, sc_state) in &side.side_conditions {
            // JS: const sideConditionData = side.sideConditions[id];
            // JS: const sideCondition = this.dex.conditions.getByID(id as ID);
            // JS: const callback = this.getCallback(side, sideCondition, callbackName);
            let has_callback = self.has_side_condition_callback(sc_id, callback_name);
            eprintln!("[FIND_SIDE_HANDLERS]   Checking {}, has_callback({})={}",
                sc_id.as_str(), callback_name, has_callback);

            // JS: if (callback !== undefined || (getKey && sideConditionData[getKey])) {
            let has_get_key = get_key.is_some_and(|key| {
                // Check for the key on EffectState struct fields, not in data HashMap
                match key {
                    "duration" => sc_state.duration.is_some(),
                    _ => false, // Other keys can be added as needed
                }
            });

            if has_callback || has_get_key {
                // JS: handlers.push(this.resolvePriority({
                // JS:     effect: sideCondition, callback, state: sideConditionData,
                // JS:     end: customHolder ? null : side.removeSideCondition, effectHolder: customHolder || side,
                // JS: }, callbackName));

                // Get side condition name from dex
                let sc_name = self.dex.conditions().get_by_id(sc_id)
                    .and_then(|c| c.name.clone())
                    .unwrap_or_else(|| sc_id.to_string());

                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: sc_id.clone(),
                        name: sc_name,
                        effect_type: EffectType::SideCondition,
                        effect_holder: custom_holder,
                        side_index: Some(side_idx),
                        prankster_boosted: false,
                    },
                    target: None,
                    index: None,
                    state: Some(sc_state.clone()),
                    effect_holder: custom_holder, // JS: customHolder || side (side not representable as tuple)
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    // Propagate effect_order from state for proper tie-breaking
                    // JavaScript sets effectOrder during resolvePriority for SwitchIn/RedirectTarget events
                    effect_order: Some(sc_state.effect_order),
                    speed: None,
                });
            }
        }

        // JS: return handlers;
        handlers
    }
}
