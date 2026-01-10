//! Handler Priority

/// Priority ordering for event handlers
/// TODO: Not in JavaScript - Rust-specific for managing handler priority ordering
#[derive(Debug, Clone, Default)]
pub struct HandlerPriority {
    /// Order value (false = highest priority in JS, we use Option<i32>)
    /// In JS: false = first, true = last, numbers in between
    pub order: Option<i32>,
    /// Priority value (higher = earlier)
    pub priority: i32,
    /// Sub-order for same priority
    pub sub_order: i32,
    /// Speed of the Pokemon (for speed ties)
    pub speed: i32,
}

impl HandlerPriority {
    /// Create with just priority
    pub fn with_priority(priority: i32) -> Self {
        Self {
            priority,
            ..Default::default()
        }
    }
}

/// Compare two handler priorities for sorting
/// Returns true if a should come before b
pub fn compare_priorities(
    a: &HandlerPriority,
    b: &HandlerPriority,
    trick_room: bool,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    // First compare order (None = highest priority, like JS false)
    match (a.order, b.order) {
        (None, Some(_)) => return Ordering::Less,
        (Some(_), None) => return Ordering::Greater,
        (Some(ao), Some(bo)) => {
            if ao != bo {
                return ao.cmp(&bo);
            }
        }
        (None, None) => {}
    }

    // Then priority (higher = earlier)
    if a.priority != b.priority {
        return b.priority.cmp(&a.priority);
    }

    // Then sub_order
    if a.sub_order != b.sub_order {
        return a.sub_order.cmp(&b.sub_order);
    }

    // Finally speed (higher = earlier, unless Trick Room)
    if trick_room {
        a.speed.cmp(&b.speed)
    } else {
        b.speed.cmp(&a.speed)
    }
}
