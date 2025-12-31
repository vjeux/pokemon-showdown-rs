use crate::*;
use crate::battle::PriorityItem;

impl Battle {

    /// Compare priority of two actions/handlers
    /// Equivalent to battle.ts comparePriority()
    /// Returns negative if a comes first, positive if b comes first, 0 if equal
    // TypeScript source:
    // /**
    // 	 * The default sort order for actions, but also event listeners.
    // 	 *
    // 	 * 1. Order, low to high (default last)
    // 	 * 2. Priority, high to low (default 0)
    // 	 * 3. Speed, high to low (default 0)
    // 	 * 4. SubOrder, low to high (default 0)
    // 	 * 5. EffectOrder, low to high (default 0)
    // 	 *
    // 	 * Doesn't reference `this` so doesn't need to be bound.
    // 	 */
    // 	comparePriority(this: void, a: AnyObject, b: AnyObject) {
    // 		return -((b.order || 4294967296) - (a.order || 4294967296)) ||
    // 			((b.priority || 0) - (a.priority || 0)) ||
    // 			((b.speed || 0) - (a.speed || 0)) ||
    // 			-((b.subOrder || 0) - (a.subOrder || 0)) ||
    // 			-((b.effectOrder || 0) - (a.effectOrder || 0)) ||
    // 			0;
    // 	}
    //
    pub fn compare_priority(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // 1. Order, low to high (default last)
        let order_cmp = a
            .order
            .unwrap_or(i32::MAX)
            .cmp(&b.order.unwrap_or(i32::MAX));
        if order_cmp != std::cmp::Ordering::Equal {
            return order_cmp;
        }

        // 2. Priority, high to low (default 0)
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // 3. Speed, high to low (default 0)
        let speed_cmp = b.speed.total_cmp(&a.speed);
        if speed_cmp != std::cmp::Ordering::Equal {
            return speed_cmp;
        }

        // 4. SubOrder, low to high (default 0)
        let sub_order_cmp = a.sub_order.cmp(&b.sub_order);
        if sub_order_cmp != std::cmp::Ordering::Equal {
            return sub_order_cmp;
        }

        // 5. EffectOrder, low to high (default 0)
        a.effect_order.cmp(&b.effect_order)
    }
}
