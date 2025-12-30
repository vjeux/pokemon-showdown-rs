use crate::*;

impl Battle {

    /// Priority event - exits on first non-undefined result
    /// Equivalent to battle.ts priorityEvent()
    // TypeScript source:
    // /**
    // 	 * priorityEvent works just like runEvent, except it exits and returns
    // 	 * on the first non-undefined value instead of only on null/false.
    // 	 */
    // 	priorityEvent(
    // 		eventid: string, target: Pokemon | Side | Battle, source?: Pokemon | null,
    // 		effect?: Effect, relayVar?: any, onEffect?: boolean
    // 	): any {
    // 		return this.runEvent(eventid, target, source, effect, relayVar, onEffect, true);
    // 	}
    //
    pub fn priority_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        relay_var: Option<i32>,
    ) -> Option<i32> {
        // For priority events, we use fastExit behavior
        self.run_event(event_id, target, source, effect, relay_var)
    }
}
