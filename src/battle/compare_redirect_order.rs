use crate::*;
use crate::battle::PriorityItem;

impl Battle {

    /// Compare for redirect order (abilities like Lightning Rod)
    //
    // 	static compareRedirectOrder(this: void, a: AnyObject, b: AnyObject) {
    // 		return ((b.priority || 0) - (a.priority || 0)) ||
    // 			((b.speed || 0) - (a.speed || 0)) ||
    // 			((a.effectHolder?.abilityState && b.effectHolder?.abilityState) ?
    // 				-(b.effectHolder.abilityState.effectOrder - a.effectHolder.abilityState.effectOrder) : 0) ||
    // 				0;
    // 	}
    //
    pub fn compare_redirect_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // Priority first
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Speed second
        let speed_cmp = b.speed.cmp(&a.speed);
        if speed_cmp != std::cmp::Ordering::Equal {
            return speed_cmp;
        }

        // Effect order (for abilities with same priority/speed)
        b.effect_order.cmp(&a.effect_order)
    }
}
