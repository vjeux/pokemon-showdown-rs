//! Event System Implementation
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Ported from pokemon-showdown/sim/battle.ts (lines 571-937)
//!
//! The event system is the core mechanism for extensibility in Pokemon Showdown.
//! Abilities, items, moves, status effects, weather, and terrain all interact
//! with the battle through event handlers.
//!
//! Key Functions:
//! - singleEvent(): Fires a single event handler (battle.ts:571-652)
//! - runEvent(): Fires all matching event handlers with priority ordering (battle.ts:758-937)

// Type modules
mod effect_type;
mod effect_data;
mod effect_state;
mod event_handler;
mod effect;

// Re-export types from submodules
pub use effect_type::EffectType;
pub use effect_data::EffectData;
pub use effect_state::EffectState;
pub use effect_state::SharedEffectState;
pub use event_handler::EventHandler;
pub use effect::{Effect, EventCallback};
