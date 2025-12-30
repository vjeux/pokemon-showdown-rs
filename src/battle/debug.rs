use crate::*;

impl Battle {

    // =========================================================================
    // Methods ported from battle.ts
    // =========================================================================

    /// Log a debug message if debug mode is enabled
    /// Equivalent to battle.ts debug()
    //
    // 	debug(activity: string) {
    // 		if (this.debugMode) {
    // 			this.add('debug', activity);
    // 		}
    // 	}
    //
    pub fn debug(&mut self, activity: &str) {
        if self.debug_mode {
            self.add("debug", &[activity.into()]);
        }
    }
}
