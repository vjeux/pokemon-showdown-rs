use crate::*;

impl Battle {

    /// Log a debug error message
    /// Equivalent to battle.ts debugError() (battle.ts:3158-3160)
    ///
    //
    // 	debugError(activity: string) {
    // 		this.add('debug', activity);
    // 	}
    //
    pub fn debug_error(&mut self, activity: &str) {
        self.add("debug", &[Arg::Str(activity)]);
    }
}
