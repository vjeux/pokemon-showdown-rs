//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod prng;
pub mod dex_data;
pub mod dex;
pub mod field;
pub mod battle_queue;
pub mod pokemon;
pub mod side;
pub mod battle;
pub mod battle_actions;
pub mod abilities;
pub mod items;
pub mod event;
pub mod data;

// Re-export main types
pub use prng::{PRNG, PRNGSeed, Gen5RNG};
pub use dex_data::{ID, to_id, Gender, StatID, StatsTable, BoostID, BoostsTable};
pub use dex_data::{EffectType, Nonstandard, GameType, SideID, MoveTarget};
pub use dex_data::{EffectState, BasicEffect, Nature, TypeInfo};
pub use dex::{Dex, SpeciesData, MoveData, AbilityData, ItemData};
pub use field::Field;
pub use battle_queue::{BattleQueue, Action, MoveAction, SwitchAction};
pub use pokemon::{Pokemon, PokemonSet, MoveSlot};
pub use side::Side;
pub use battle::{Battle, BattleOptions, PlayerOptions};
pub use battle_actions::{BattleActions, DamageResult, MoveHitData};
pub use abilities::{AbilityEvent, AbilityModifier, get_ability_effect, check_ability_immunity, check_ability_prevents_status};
pub use items::{ItemEvent, ItemModifier, get_item_effect, get_item_type_boost, check_item_prevents_status};
pub use event::{EventType, EventResult, EffectType as EventEffectType, HandlerPriority, EventHandler, MoveFlags, AbilityFlags, ConditionData};
pub use data::abilities::{AbilityDef, BasePowerBoost, get_ability, ability_grants_type_immunity, ability_absorbs_type, ability_grants_status_immunity};
pub use data::moves::{MoveDef, MoveCategory, MoveTargetType, SecondaryEffect, get_move, is_pivot_move, is_status_move, get_base_power, get_accuracy};
