//! ActiveMove struct - represents a move being executed

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::dex::{Accuracy, ConditionData, IsMax, MoveSecondary, Multihit, Ohko};
use crate::dex_data::{BoostsTable, ID};
use crate::battle::Effect;

use super::{Damage, IgnoreImmunity, MaxMoveData, MoveFlags, ZMoveData};

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: ActiveMove (sim/dex-moves.ts)
/// 35+ fields in JavaScript (plus inherited from MutableMove which is BasicEffect & MoveData & HitEffect)
pub struct ActiveMove {
    // =========================================================================
    // From BasicEffect (inherited via MutableMove)
    // =========================================================================
    /// Move ID
    pub id: ID,
    /// Move name
    pub name: String,
    /// Full name with effect type
    pub fullname: String,
    /// Move number in the Dex
    pub num: i32,
    /// Does this move exist?
    pub exists: bool,
    /// Generation this move was introduced
    pub gen: u8,
    /// Short description
    pub short_desc: String,
    /// Full description
    pub desc: String,
    /// Nonstandard status (Past, Future, etc.)
    pub is_nonstandard: Option<String>,
    /// Effect duration
    pub duration: Option<i32>,
    /// Cannot be copied by Baton Pass
    pub no_copy: bool,
    /// Affects fainted Pokemon
    pub affects_fainted: bool,
    /// Source effect that created this move
    pub source_effect_name: Option<String>,

    // =========================================================================
    // From MoveData (inherited via MutableMove)
    // =========================================================================
    /// Condition data for moves that create conditions
    pub condition: Option<ConditionData>,
    /// Base power of the move
    pub base_power: i32,
    /// Accuracy value
    pub accuracy: Accuracy,
    /// Power Points
    pub pp: i32,
    /// Move category (Physical/Special/Status)
    pub category: String,
    /// Move type
    pub move_type: String,
    /// Move priority (-7 to +7)
    pub priority: i8,
    /// Target type
    pub target: String,
    /// Move flags
    pub flags: MoveFlags,
    /// Real move ID (for called/transformed moves)
    pub real_move: Option<String>,
    /// Fixed damage value
    pub damage: Option<Damage>,
    /// Contest type
    pub contest_type: Option<String>,
    /// No PP boosts allowed
    pub no_pp_boosts: bool,
    /// Is this a Z-move?
    pub is_z: Option<String>,
    /// Z-move data
    pub z_move: Option<ZMoveData>,
    /// Is this a Max move?
    #[serde(rename = "isMax", default, deserialize_with = "crate::dex::deserialize_is_max")]
    pub is_max: Option<IsMax>,
    /// Max move data
    pub max_move: Option<MaxMoveData>,
    /// OHKO type
    #[serde(default, deserialize_with = "crate::dex::deserialize_ohko")]
    pub ohko: Option<Ohko>,
    /// Thaws the target
    pub thaws_target: bool,
    /// Healing fraction
    pub heal: Option<(i32, i32)>,
    /// Draining fraction
    pub drain: Option<(i32, i32)>,
    /// Forces the target to switch out
    pub force_switch: bool,
    /// Self-switch effect
    pub self_switch: Option<String>,
    /// Self stat boosts (e.g., Bulk Up)
    pub self_boost: Option<BoostsTable>,
    /// Self-destruct type
    pub self_destruct: Option<String>,
    /// Breaks through protection
    pub breaks_protect: bool,
    /// Recoil damage fraction
    pub recoil: Option<(i32, i32)>,
    /// Mind Blown recoil flag
    pub mindblown_recoil: bool,
    /// Steals target's stat boosts
    pub steals_boosts: bool,
    /// Struggle recoil flag
    pub struggle_recoil: bool,
    /// Single secondary effect
    pub secondary: Option<MoveSecondary>,
    /// Multiple secondary effects
    pub secondaries: Vec<MoveSecondary>,
    /// Self-targeting effect
    pub self_effect: Option<MoveSecondary>,
    /// Has Sheer Force flag
    pub has_sheer_force: bool,
    /// Always hits (ignores accuracy)
    pub always_hit: bool,
    /// Base move type before type-changing effects
    pub base_move_type: Option<String>,
    /// Base power modifier
    pub base_power_modifier: Option<f64>,
    /// Critical hit modifier
    pub crit_modifier: Option<f64>,
    /// Critical hit ratio (1 = normal, 2 = high crit rate)
    pub crit_ratio: i32,
    /// Override which Pokemon's offensive stat to use
    pub override_offensive_pokemon: Option<String>,
    /// Override which offensive stat to use
    pub override_offensive_stat: Option<String>,
    /// Override which Pokemon's defensive stat to use
    pub override_defensive_pokemon: Option<String>,
    /// Override which defensive stat to use
    pub override_defensive_stat: Option<String>,
    /// Force STAB (Same Type Attack Bonus)
    pub force_stab: bool,
    /// Ignore target's ability
    pub ignore_ability: bool,
    /// Ignore accuracy checks
    pub ignore_accuracy: bool,
    /// Ignore target's evasion
    pub ignore_evasion: bool,
    /// Ignore positive evasion boosts
    pub ignore_positive_evasion: Option<bool>,
    /// Ignore type immunity
    pub ignore_immunity: Option<IgnoreImmunity>,
    /// Ignore defensive stat changes
    pub ignore_defensive: bool,
    /// Ignore offensive stat changes
    pub ignore_offensive: bool,
    /// Ignore negative offensive stat changes
    pub ignore_negative_offensive: bool,
    /// Ignore positive defensive stat changes
    pub ignore_positive_defensive: bool,
    /// Infiltrates through Substitute, etc.
    pub infiltrates: bool,
    /// Will always crit
    pub will_crit: Option<bool>,
    /// Multi-accuracy flag (for moves like Triple Axel)
    pub multi_accuracy: bool,
    /// Number of hits for multi-hit moves
    pub multi_hit: Option<Multihit>,
    /// Multi-hit type (e.g., "parentalbond")
    pub multi_hit_type: Option<String>,
    /// No damage variance (no random factor)
    pub no_damage_variance: Option<bool>,
    /// Non-ghost target type (for Curse)
    pub non_ghost_target: Option<String>,
    /// Spread move damage modifier
    pub spread_modifier: Option<f64>,
    /// Usable while sleeping
    pub sleep_usable: bool,
    /// Smart target selection
    pub smart_target: Option<bool>,
    /// Tracks target (e.g., Lock-On)
    pub tracks_target: Option<bool>,
    /// Calls another move
    pub calls_move: bool,
    /// Has crash damage on miss (High Jump Kick, etc.)
    pub has_crash_damage: bool,
    /// Is this confusion self-hit?
    pub is_confusion_self_hit: Option<bool>,
    /// Is this a stalling move (Protect, etc.)?
    pub stalling_move: bool,
    /// Base move ID (for moves that call other moves)
    pub base_move: Option<ID>,

    // =========================================================================
    // From HitEffect (inherited via MoveData)
    // =========================================================================
    /// Stat boosts to apply
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    pub status: Option<String>,
    /// Volatile status to inflict
    pub volatile_status: Option<String>,
    /// Side condition to apply
    pub side_condition: Option<String>,
    /// Slot condition to apply
    pub slot_condition: Option<String>,
    /// Pseudo-weather to apply
    pub pseudo_weather: Option<String>,
    /// Terrain to set
    pub terrain: Option<String>,
    /// Weather to set
    pub weather: Option<String>,

    // =========================================================================
    // ActiveMove-specific fields
    // =========================================================================
    /// Hit number in multi-hit moves
    pub hit: i32,
    /// Total damage dealt
    pub total_damage: i32,
    /// Move hit data for tracking crit, type effectiveness, etc.
    pub move_hit_data: HashMap<String, crate::pokemon::MoveHitData>,
    /// Hit a spread move (doubles/triples)
    pub spread_hit: bool,
    /// Last hit of a multi-hit move
    pub last_hit: Option<bool>,
    /// Move is from external source (not direct Pokemon action)
    pub is_external: bool,
    /// Is Z or Max powered
    pub is_z_or_max_powered: bool,
    /// Prankster boosted this move
    pub prankster_boosted: bool,
    /// Move has been bounced by Magic Coat/Bounce
    pub has_bounced: bool,
    /// Source effect that triggered this move (Dancer, Instruct, etc.)
    pub source_effect: Option<Effect>,
    /// Aura Break ability is active
    pub has_aura_break: Option<bool>,
    /// Pokemon with aura ability boosting this move
    pub aura_booster: Option<(usize, usize)>,
    /// Move caused crash damage (High Jump Kick, Jump Kick)
    pub caused_crash_damage: Option<bool>,
    /// User dropped its item (Fling, etc.)
    pub self_dropped: bool,
    /// Move is Stellar type boosted
    pub stellar_boosted: bool,
    /// Effect ID that changed this move's type
    pub type_changer_boosted: Option<Effect>,
    /// Magnitude value for Magnitude move
    pub magnitude: Option<i32>,
    /// Whether this move will cause a forme change (relicsong)
    pub will_change_forme: bool,
    /// Status roll result for secondary status effects
    pub status_roll: Option<String>,
    /// Force a specific status condition
    pub force_status: Option<String>,

    // Tablets of Ruin tracking
    /// Pokemon with Tablets of Ruin lowering this move's attack
    pub ruined_atk: Option<(usize, usize)>,
    /// Pokemon with Vessel of Ruin lowering this move's sp. attack
    pub ruined_spa: Option<(usize, usize)>,
    /// Pokemon with Sword of Ruin lowering target's defense
    pub ruined_def: Option<(usize, usize)>,
    /// Pokemon with Beads of Ruin lowering target's sp. defense
    pub ruined_spd: Option<(usize, usize)>,

    // Allies for spread moves
    /// Allied Pokemon for spread move calculations
    pub allies: Option<Vec<(usize, usize)>>,
    /// Ability affecting this move
    pub ability: Option<ID>,

    // Hit targets (populated during execution)
    /// Targets hit by this move
    pub hit_targets: Vec<(usize, usize)>,
}
