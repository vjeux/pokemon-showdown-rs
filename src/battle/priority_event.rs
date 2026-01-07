// JS Source:
// 	priorityEvent(
// 		eventid: string, target: Pokemon | Side | Battle, source?: Pokemon | null,
// 		effect?: Effect, relayVar?: any, onEffect?: boolean
// 	): any {
// 		return this.runEvent(eventid, target, source, effect, relayVar, onEffect, true);
// 	}


use crate::*;
use crate::battle::Effect;
use crate::event::EventResult;

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
        effect: Option<&Effect>,
        relay_var: EventResult,
    ) -> EventResult {
        // For priority events, we use fastExit behavior (fast_exit = true)
        self.run_event(
            event_id,
            crate::event::EventTarget::from_pokemon(target),
            source,
            effect,
            relay_var,
            false,
            true
        )
    }
}
