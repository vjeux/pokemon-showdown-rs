use crate::*;

impl Battle {

    /// Destroy the battle (cleanup for garbage collection)
    /// Equivalent to battle.ts destroy() (battle.ts:3346-3367)
    ///
    /// JS: destroy() {
    ///   this.field.destroy();
    ///   for (let i = 0; i < this.sides.length; i++) {
    ///     if (this.sides[i]) this.sides[i].destroy();
    ///   }
    ///   for (const action of this.queue.list) delete (action as any).pokemon;
    ///   this.queue.battle = null!;
    ///   (this as any).log = [];
    /// }
    //
    // 	destroy() {
    // 		// deallocate ourself
    //
    // 		// deallocate children and get rid of references to them
    // 		this.field.destroy();
    // 		(this as any).field = null!;
    //
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			if (this.sides[i]) {
    // 				this.sides[i].destroy();
    // 				this.sides[i] = null!;
    // 			}
    // 		}
    // 		for (const action of this.queue.list) {
    // 			delete (action as any).pokemon;
    // 		}
    //
    // 		this.queue.battle = null!;
    // 		this.queue = null!;
    // 		// in case the garbage collector really sucks, at least deallocate the log
    // 		(this as any).log = [];
    // 	}
    //
    pub fn destroy(&mut self) {
        // Note: In Rust, cleanup is handled automatically by the Drop trait
        // The JS version manually breaks circular references for garbage collection
        // This is not needed in Rust, so this method is effectively a no-op

        // We can still clear collections to free memory explicitly if desired
        self.sides.clear();
        self.queue.clear();
        self.log.clear();
    }
}
