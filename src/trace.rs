//! Trace logging for debugging battle divergences
//!
//! Enable via environment variables:
//! - TRACE_EVENTS=1    - Log all event firing and handlers
//! - TRACE_ABILITIES=1 - Log ability activations
//! - TRACE_ITEMS=1     - Log item activations
//! - TRACE_VOLATILES=1 - Log volatile add/remove
//! - TRACE_BOOSTS=1    - Log boost changes
//! - TRACE_ALL=1       - Enable all traces

use std::sync::OnceLock;

static TRACE_EVENTS: OnceLock<bool> = OnceLock::new();
static TRACE_ABILITIES: OnceLock<bool> = OnceLock::new();
static TRACE_ITEMS: OnceLock<bool> = OnceLock::new();
static TRACE_VOLATILES: OnceLock<bool> = OnceLock::new();
static TRACE_BOOSTS: OnceLock<bool> = OnceLock::new();

fn env_enabled(key: &str) -> bool {
    std::env::var(key).unwrap_or_default() == "1"
}

pub fn trace_events_enabled() -> bool {
    *TRACE_EVENTS.get_or_init(|| env_enabled("TRACE_EVENTS") || env_enabled("TRACE_ALL"))
}

pub fn trace_abilities_enabled() -> bool {
    *TRACE_ABILITIES.get_or_init(|| env_enabled("TRACE_ABILITIES") || env_enabled("TRACE_ALL"))
}

pub fn trace_items_enabled() -> bool {
    *TRACE_ITEMS.get_or_init(|| env_enabled("TRACE_ITEMS") || env_enabled("TRACE_ALL"))
}

pub fn trace_volatiles_enabled() -> bool {
    *TRACE_VOLATILES.get_or_init(|| env_enabled("TRACE_VOLATILES") || env_enabled("TRACE_ALL"))
}

pub fn trace_boosts_enabled() -> bool {
    *TRACE_BOOSTS.get_or_init(|| env_enabled("TRACE_BOOSTS") || env_enabled("TRACE_ALL"))
}

/// Log an event firing
#[macro_export]
macro_rules! trace_event {
    ($($arg:tt)*) => {
        if $crate::trace::trace_events_enabled() {
            eprintln!("[EVENT] {}", format!($($arg)*));
        }
    };
}

/// Log an ability activation
#[macro_export]
macro_rules! trace_ability {
    ($($arg:tt)*) => {
        if $crate::trace::trace_abilities_enabled() {
            eprintln!("[ABILITY] {}", format!($($arg)*));
        }
    };
}

/// Log an item activation
#[macro_export]
macro_rules! trace_item {
    ($($arg:tt)*) => {
        if $crate::trace::trace_items_enabled() {
            eprintln!("[ITEM] {}", format!($($arg)*));
        }
    };
}

/// Log a volatile add/remove
#[macro_export]
macro_rules! trace_volatile {
    ($($arg:tt)*) => {
        if $crate::trace::trace_volatiles_enabled() {
            eprintln!("[VOLATILE] {}", format!($($arg)*));
        }
    };
}

/// Log a boost change
#[macro_export]
macro_rules! trace_boost {
    ($($arg:tt)*) => {
        if $crate::trace::trace_boosts_enabled() {
            eprintln!("[BOOST] {}", format!($($arg)*));
        }
    };
}
