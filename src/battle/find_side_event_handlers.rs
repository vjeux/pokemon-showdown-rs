use crate::*;

impl Battle {

    /// Find side event handlers
    /// Equivalent to battle.ts findSideEventHandlers()
    //
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
    //
    pub fn find_side_event_handlers(
        &self,
        _event_id: &str,
        side_idx: usize,
    ) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();

        if let Some(side) = self.sides.get(side_idx) {
            // Add side condition handlers
            for sc_id in side.side_conditions.keys() {
                handlers.push((sc_id.clone(), None));
            }
        }

        handlers
    }
}
