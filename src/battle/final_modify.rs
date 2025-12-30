use crate::*;

impl Battle {

    /// Final stat modification with 4096 denominator
    /// Apply final modifier to value
    /// Equivalent to battle.ts finalModify() (battle.ts:2344-2347)
    ///
    //
    // 	finalModify(relayVar: number) {
    // 		relayVar = this.modify(relayVar, this.event.modifier);
    // 		this.event.modifier = 1;
    // 		return relayVar;
    // 	}
    //
    pub fn final_modify(&mut self, value: i32) -> i32 {
        // JS: relayVar = this.modify(relayVar, this.event.modifier);
        let modifier = self.get_event_modifier();
        let result = self.modify_internal(value, modifier);

        // JS: this.event.modifier = 1;
        if let Some(ref mut event) = self.current_event {
            event.modifier = 4096;
        }

        result
    }
}
