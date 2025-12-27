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
pub mod event_system;
pub mod data;
pub mod state;
pub mod team_validator;
pub mod battle_stream;
pub mod choice;
pub mod stats;
pub mod random_teams;
pub mod move_types;

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
pub use items::{ItemEvent, ItemModifier, get_item_effect, get_item_type_boost, check_item_prevents_status};
pub use event::{EventType, EventResult, EffectType as EventEffectType, HandlerPriority, EventHandler, MoveFlags, AbilityFlags, ConditionData};
pub use move_types::{MoveDef, MoveCategory, MoveTargetType};
pub use data::items::{ItemCategory};
pub use data::conditions::{ConditionDef, ConditionType, MoveRestriction, get_condition, is_status_condition, is_volatile_condition, condition_traps, get_condition_damage};
pub use data::typechart::{TYPES, Effectiveness, TYPE_CHART, get_effectiveness, get_effectiveness_multi, is_immune, is_super_effective, is_not_very_effective};
pub use data::species::{SpeciesDef, BaseStats, GenderRatio, SPECIES, get_species, get_species_by_name, species_exists, get_species_by_type, get_species_by_tier};
pub use data::natures::{NatureDef, NatureStat, NATURES, get_nature, get_nature_by_name, nature_stat_multiplier, get_boosting_natures, get_lowering_natures, get_neutral_natures};
pub use data::formats::{FormatDef, RuleDef, GameType as FormatGameType, FormatMod, FORMATS, RULESETS, get_format, get_format_by_name, format_exists, get_ruleset, get_formats_by_gen, is_banned_in_format};
pub use state::{BattleState, SideState, PokemonState, FieldState, MoveSlotState, StatsState, BoostsState, ReplayData};
pub use team_validator::{TeamValidator, ValidatorSet, ValidationError, EVSpread, IVSpread, parse_team};
pub use battle_stream::{BattleStream, BattleMessage};
pub use choice::{Choice, ChoiceError, parse_choices, BattleRequest, RequestType, ActiveRequest, MoveRequest, SideRequest, PokemonRequest};
pub use stats::{calc_hp, calc_stat, calc_all_stats, StatSpread, get_boost_multiplier, apply_boost, calc_damage_range};
pub use random_teams::{RandomTeamGenerator, generate_random_team_string};
pub mod teams;
pub use teams::{pack_team, unpack_team, export_team};
