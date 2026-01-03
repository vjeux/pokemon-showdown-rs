// 1:1 port of findSideEventHandlers from battle.ts

use crate::*;
use crate::battle::{EventListener, EffectType};

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
        _callback_name: &str,
        side_idx: usize,
        _get_key: Option<&str>,
        custom_holder: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();

        // Get side reference
        let side = match self.sides.get(side_idx) {
            Some(s) => s,
            None => return handlers,
        };

        // JS: for (const id in side.sideConditions) {
        for (sc_id, _sc_state) in &side.side_conditions {
            // JS: const sideConditionData = side.sideConditions[id];
            // JS: const sideCondition = this.dex.conditions.getByID(id as ID);
            // JS: const callback = this.getCallback(side, sideCondition, callbackName);
            // TODO: Should check callback via getCallback (architectural difference)

            // JS: if (callback !== undefined || (getKey && sideConditionData[getKey])) {
            // TODO: Should check getKey condition
            // For now, always add all side conditions

            // JS: handlers.push(this.resolvePriority({
            // JS:     effect: sideCondition, callback, state: sideConditionData,
            // JS:     end: customHolder ? null : side.removeSideCondition, effectHolder: customHolder || side,
            // JS: }, callbackName));
            handlers.push(EventListener {
                effect_id: sc_id.clone(),
                effect_type: EffectType::SideCondition,
                target: None,
                index: None,
                state: None, // TODO: Should be sideConditionData
                effect_holder: custom_holder, // JS: customHolder || side (side not representable as tuple)
                order: None,
                priority: 0,
                sub_order: 0,
                effect_order: None,
                speed: None,
            });
        }

        // JS: return handlers;
        handlers
    }
}
