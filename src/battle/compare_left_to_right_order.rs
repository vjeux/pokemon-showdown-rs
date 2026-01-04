// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::event::EventResult;
use crate::battle::PriorityItem;

impl Battle {

    /// Compare for left-to-right order (hazards, etc.)
    //
    // 	static compareLeftToRightOrder(this: void, a: AnyObject, b: AnyObject) {
    // 		return -((b.order || 4294967296) - (a.order || 4294967296)) ||
    // 			((b.priority || 0) - (a.priority || 0)) ||
    // 			-((b.index || 0) - (a.index || 0)) ||
    // 			0;
    // 	}
    //
    pub fn compare_left_to_right_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // Order first
        let order_cmp = a
            .order
            .unwrap_or(i32::MAX)
            .cmp(&b.order.unwrap_or(i32::MAX));
        if order_cmp != std::cmp::Ordering::Equal {
            return order_cmp;
        }

        // Priority second
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Index (position) - lower index first
        a.index.cmp(&b.index)
    }
}
