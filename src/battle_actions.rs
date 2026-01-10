//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This is a 1:1 port of sim/battle-actions.ts
//! Handles all battle actions: moves, switches, damage calculation, etc.

use std::collections::HashSet;
use once_cell::sync::Lazy;

// Type modules
mod damage_result;
mod active_move;
mod move_flags;
mod move_data;
mod enums;
mod effects;
mod params;
mod options;
mod results;
mod battle_actions;

// Function modules
mod new;
mod get_boost_modifier;
mod calc_recoil_damage;
mod combine_results;
mod target_type_choices;
mod can_mega_evo;
mod can_ultra_burst;
mod run_mega_evo;
mod can_terastallize;
mod terastallize;
mod run_z_power;
pub mod use_move;
mod use_move_inner;
mod switch_in;
mod drag_in;
mod run_switch;
mod get_damage;
mod get_spread_damage;
mod modify_damage;
mod run_move_effects;
mod spread_move_hit;
mod move_hit;
mod try_move_hit;
mod try_spread_move_hit;
mod hit_step_accuracy;
mod run_move;
mod get_z_move;
mod can_z_move;
mod get_active_z_move;
mod get_max_move;
mod get_active_max_move;
mod after_move_secondary_event;
mod force_switch;
mod hit_step_break_protect;
mod hit_step_invulnerability_event;
mod hit_step_steal_boosts;
mod hit_step_try_hit_event;
mod hit_step_try_immunity;
mod hit_step_type_immunity;
mod try_primary_hit_event;
mod self_drops;
mod secondaries;
mod hit_step_move_hit_loop;
mod add_pseudoweather;

// Re-export types from submodules
pub use damage_result::DamageResult;
pub use active_move::ActiveMove;
pub use move_flags::MoveFlags;
pub use move_data::{MoveHitData, MaxMoveData, ZMoveData, ZMoveOption};
pub use enums::{IgnoreImmunity, Damage, DamageValue, SwitchCopyFlag, SpreadMoveTarget};
pub use effects::{SecondaryEffect, SelfEffect, HitEffect};
pub use params::{ZMoveParams, CanZMoveParams, DamageCalcParams};
pub use options::{AfterMoveResult, MoveEffects, RunMoveOptions, UseMoveOptions};
pub use results::{RunMoveResult, ZPowerResult, TerastallizeResult, SwitchInResult};
pub use battle_actions::{BattleActions, max_moves, z_moves};

// Re-export functions
pub use can_mega_evo::can_mega_evo;
pub use can_ultra_burst::can_ultra_burst;
pub use combine_results::combine_results;
pub use can_terastallize::can_terastallize;
pub use run_mega_evo::run_mega_evo;
pub use terastallize::terastallize;
pub use use_move::use_move;
pub use use_move_inner::use_move_inner;
pub use switch_in::switch_in;
pub use drag_in::drag_in;
pub use run_switch::run_switch;
pub use get_damage::get_damage;
pub use get_spread_damage::get_spread_damage;
pub use modify_damage::modify_damage;
pub use run_move_effects::run_move_effects;
pub use spread_move_hit::spread_move_hit;
pub use move_hit::move_hit;
pub use try_move_hit::try_move_hit;
pub use try_spread_move_hit::try_spread_move_hit;
pub use hit_step_accuracy::hit_step_accuracy;
pub use run_move::run_move;
pub use get_z_move::get_z_move;
pub use can_z_move::can_z_move;
pub use get_active_z_move::get_active_z_move;
pub use get_max_move::get_max_move;
pub use get_active_max_move::get_active_max_move;
pub use after_move_secondary_event::after_move_secondary_event;
pub use force_switch::force_switch;
pub use hit_step_break_protect::hit_step_break_protect;
pub use hit_step_invulnerability_event::hit_step_invulnerability_event;
pub use hit_step_steal_boosts::hit_step_steal_boosts;
pub use hit_step_try_hit_event::hit_step_try_hit_event;
pub use hit_step_try_immunity::hit_step_try_immunity;
pub use hit_step_type_immunity::hit_step_type_immunity;
pub use try_primary_hit_event::try_primary_hit_event;
pub use self_drops::self_drops;
pub use secondaries::secondaries;
pub use hit_step_move_hit_loop::hit_step_move_hit_loop;

/// Choosable target types for moves
/// JavaScript equivalent: CHOOSABLE_TARGETS constant (sim/battle-actions.ts)
pub static CHOOSABLE_TARGETS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("normal");
    set.insert("any");
    set.insert("adjacentAlly");
    set.insert("adjacentAllyOrSelf");
    set.insert("adjacentFoe");
    set
});

/// Spread move damage type alias
pub type SpreadMoveDamage = Vec<DamageResult>;

/// Extension trait for SpreadMoveDamage to provide reduce functionality
pub trait SpreadMoveDamageExt {
    /// Reduce a damage array using combine_results
    fn reduce(&self) -> DamageResult;
}

impl SpreadMoveDamageExt for SpreadMoveDamage {
    fn reduce(&self) -> DamageResult {
        self.iter()
            .fold(DamageResult::Undefined, |acc, &d| combine_results(acc, d))
    }
}

/// Spread move targets array
pub type SpreadMoveTargets = Vec<SpreadMoveTarget>;
