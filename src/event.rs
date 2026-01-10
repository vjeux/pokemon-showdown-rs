//! Event System
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Implements the runEvent infrastructure for effect dispatch.
//! Effects (Abilities, Items, Moves, Status, Volatiles) can register handlers
//! for events, and the battle dispatches to them.

// Type modules
mod event_target;
mod event_type;
mod effect_type;
mod event_result;
mod handler_priority;
mod event_handler;
mod ability_flags;
mod move_flags;
mod condition_data;
mod immunity_type;
mod effect_handler;

// Re-export types from submodules
pub use event_target::EventTarget;
pub use event_type::EventType;
pub use effect_type::EffectType;
pub use event_result::EventResult;
pub use handler_priority::{HandlerPriority, compare_priorities};
pub use event_handler::EventHandler;
pub use ability_flags::AbilityFlags;
pub use move_flags::MoveFlags;
pub use condition_data::ConditionData;
pub use immunity_type::ImmunityType;
pub use effect_handler::{EffectHandler, collect_handlers, sort_handlers};
