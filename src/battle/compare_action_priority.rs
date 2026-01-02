use crate::*;

impl Battle {

    /// Compare priority of two actions for queue insertion
    /// Matches JavaScript's comparePriority from battle.js line 257
    /// Returns: negative if a should go before b, positive if b should go before a, 0 if equal
    ///
    /// Note: This is a static method (doesn't use self) so it can be called from
    /// BattleQueue methods without needing a Battle reference for the comparison logic.
    //
    // JS: comparePriority(a, b) {
    //   return -((b.order || 4294967296) - (a.order || 4294967296)) ||
    //          (b.priority || 0) - (a.priority || 0) ||
    //          (b.speed || 0) - (a.speed || 0) ||
    //          -((b.subOrder || 0) - (a.subOrder || 0)) ||
    //          -((b.effectOrder || 0) - (a.effectOrder || 0)) || 0;
    // }
    pub fn compare_action_priority(a: &crate::battle_queue::Action, b: &crate::battle_queue::Action) -> i32 {

        // Get order (default 4294967296 in JS)
        let a_order = a.order() as i64;
        let b_order = b.order() as i64;

        // JS: -((b.order || 4294967296) - (a.order || 4294967296))
        let order_cmp = -((b_order - a_order) as i32);
        if order_cmp != 0 {
            return order_cmp;
        }

        // JS: (b.priority || 0) - (a.priority || 0)
        let priority_cmp = b.priority() - a.priority();
        if priority_cmp != 0 {
            return priority_cmp as i32;
        }

        // JS: (b.speed || 0) - (a.speed || 0)
        // Use total_cmp for f64 comparison (handles NaN properly)
        let speed_cmp = b.speed().total_cmp(&a.speed());
        if speed_cmp != std::cmp::Ordering::Equal {
            return match speed_cmp {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
            };
        }

        // JS: -((b.subOrder || 0) - (a.subOrder || 0))
        let sub_order_cmp = -((b.sub_order() - a.sub_order()) as i32);
        if sub_order_cmp != 0 {
            return sub_order_cmp;
        }

        // JS: -((b.effectOrder || 0) - (a.effectOrder || 0))
        let effect_order_cmp = -((b.effect_order() - a.effect_order()) as i32);
        if effect_order_cmp != 0 {
            return effect_order_cmp;
        }

        0
    }
}
