use crate::*;

impl Battle {

    /// Check if the active move has Sheer Force boost
    /// Equivalent to move.hasSheerForce in battle-actions.ts
    /// Returns true if the active move would have secondary effects suppressed by Sheer Force
    pub fn active_move_has_sheer_force(&self) -> bool {
        // TODO: Implement proper hasSheerForce logic
        // In TypeScript: this.hasSheerForce = !!(data.hasSheerForce && !this.secondaries);
        // For now, return false to allow compilation
        false
    }
}
