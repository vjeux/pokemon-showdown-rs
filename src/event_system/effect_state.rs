//! Effect State

use crate::dex_data::ID;

/// Effect state - matches JavaScript EffectState
/// Stores state for temporary effects (volatiles, side conditions, etc.)
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: EffectState (sim/global-types.ts)
/// JavaScript uses dynamic properties [k: string]: any, Rust uses typed fields
pub struct EffectState {
    /// Effect ID
    /// JavaScript: id: ID
    pub id: ID,
    /// Target of the effect
    /// JavaScript: target?: Pokemon
    pub target: Option<(usize, usize)>,
    /// Whether the target is a Field (for pseudo-weather/weather/terrain sub_order calculation)
    /// JavaScript: In JS, target can be Pokemon, Side, or Field. When it's Field, effectType=Condition gets sub_order=5
    /// This flag is set when an event is executed on a field condition (during runEvent handler execution)
    pub target_is_field: bool,
    /// Source of the effect
    /// JavaScript: source?: Pokemon
    pub source: Option<(usize, usize)>,
    /// Duration remaining (turns)
    /// JavaScript: duration?: number
    pub duration: Option<i32>,
    /// Turn when this effect was created
    pub created_turn: Option<i32>,
    /// Time (for sleep/confusion duration countdown)
    /// JavaScript: time?: number
    pub time: Option<i32>,
    /// Effect order (for sorting multiple effects)
    pub effect_order: i32,
    /// Side index (for side conditions)
    /// JavaScript: side?: Side
    pub side: Option<usize>,
    /// Target side index (for side conditions)
    /// JavaScript: targetSide?: Side
    pub target_side: Option<usize>,
    /// Source effect that caused this effect
    /// JavaScript: sourceEffect?: Effect
    pub source_effect: Option<crate::battle::Effect>,
    /// Source slot (for slot conditions)
    /// JavaScript: sourceSlot?: number
    pub source_slot: Option<usize>,
    /// Hit count (for multi-hit moves like Rollout)
    /// JavaScript: hitCount?: number
    pub hit_count: Option<i32>,
    /// Contact hit count (for Rocky Helmet, etc.)
    /// JavaScript: contactHitCount?: number
    pub contact_hit_count: Option<i32>,
    /// Prankster boosted flag (for priority moves affected by Prankster)
    /// JavaScript: pranksterBoosted?: boolean
    pub prankster_boosted: bool,

    // ========== Condition/Volatile state fields ==========

    /// Move ID for locked moves (choicelock, lockedmove, twoturnmove, futuremove)
    /// JavaScript: move?: ID
    #[serde(rename = "move")]
    pub move_id: Option<String>,
    /// Target location for two-turn moves
    /// JavaScript: targetLoc?: number
    pub target_loc: Option<i32>,
    /// Stall counter (protect, detect, etc.)
    /// JavaScript: counter?: number
    pub counter: Option<i32>,
    /// Dynamax turns counter
    /// JavaScript: turns?: number
    pub turns: Option<i32>,
    /// True duration for locked moves (separate from duration)
    /// JavaScript: trueDuration?: number
    pub true_duration: Option<i32>,
    /// Start time for sleep
    /// JavaScript: startTime?: number
    pub start_time: Option<i32>,
    /// Bound divisor for partially trapped
    /// JavaScript: boundDivisor?: number
    pub bound_divisor: Option<i32>,
    /// Toxic stage counter
    /// JavaScript: stage?: number
    pub stage: Option<i32>,
    /// Layers count (spikes, toxic spikes, stockpile, g-max chi strike, dragon cheer)
    /// JavaScript: layers?: number
    pub layers: Option<i32>,
    /// Has dragon type flag (dragon cheer)
    /// JavaScript: hasDragonType?: boolean
    pub has_dragon_type: Option<bool>,
    /// Got hit flag (shell trap)
    /// JavaScript: gotHit?: boolean
    pub got_hit: Option<bool>,
    /// Accuracy override flag (blizzard, bleakwind storm in weather)
    /// JavaScript: accuracy?: boolean
    pub accuracy: Option<bool>,
    /// Source slot for counter/mirror coat
    /// JavaScript: slot?: number
    pub slot: Option<i32>,
    /// Damage stored for counter/mirror coat/bide
    /// JavaScript: damage?: number
    pub damage: Option<i32>,
    /// Linked Pokemon positions (for linked volatiles like psychic noise)
    /// JavaScript: linkedPokemon?: Pokemon[]
    pub linked_pokemon: Option<Vec<(usize, usize)>>,
    /// Linked status ID (for linked volatiles)
    /// JavaScript: linkedStatus?: ID
    pub linked_status: Option<String>,
    /// Ending turn for future moves
    /// JavaScript: endingTurn?: number
    pub ending_turn: Option<i32>,
    /// Counterpart position (for fake out)
    /// JavaScript: counterpart?: Pokemon
    pub counterpart: Option<(usize, usize)>,
    /// Locked move flag (for disable end)
    /// JavaScript: locked?: boolean
    pub locked: Option<bool>,
    /// Locked target Pokemon position (for partialtrappinglock)
    /// JavaScript: locked?: Pokemon (overloaded meaning)
    pub locked_target: Option<(usize, usize)>,
    /// Fury cutter multiplier
    /// JavaScript: multiplier?: number
    pub multiplier: Option<i32>,
    /// HP value (for wish)
    /// JavaScript: hp?: number
    pub hp: Option<i32>,
    /// Starting turn (for wish)
    /// JavaScript: startingTurn?: number
    pub starting_turn: Option<i32>,
    /// Lost focus flag (for focus punch)
    /// JavaScript: lostFocus?: boolean
    pub lost_focus: Option<bool>,
    /// Move slot index (for leppa berry)
    /// JavaScript: move_slot_index?: number
    pub move_slot_index: Option<usize>,
    /// Defense boost tracking (for stockpile)
    /// JavaScript: def?: number
    pub def: Option<i32>,
    /// Special Defense boost tracking (for stockpile)
    /// JavaScript: spd?: number
    pub spd: Option<i32>,

    // ========== Ability state fields ==========

    /// Berry ID for cud chew
    /// JavaScript: berry?: ID
    pub berry: Option<String>,
    /// Choice lock move for gorilla tactics
    /// JavaScript: choiceLock?: ID
    pub choice_lock: Option<String>,
    /// Syrup triggered flag (super sweet syrup)
    /// JavaScript: syrupTriggered?: boolean
    pub syrup_triggered: Option<bool>,
    /// Unnerved flag (as one, unnerve)
    /// JavaScript: unnerved?: boolean
    pub unnerved: Option<bool>,
    /// Embodied flag (embody aspect abilities)
    /// JavaScript: embodied?: boolean
    pub embodied: Option<bool>,
    /// Fallen count (supreme overlord)
    /// JavaScript: fallen?: number
    pub fallen: Option<i32>,
    /// Gluttony active flag
    /// JavaScript: gluttony?: boolean
    pub gluttony: Option<bool>,
    /// Ending flag (neutralizing gas)
    /// JavaScript: ending?: boolean
    pub ending: Option<bool>,
    /// Shield boost flag (dauntless shield)
    /// JavaScript: shieldBoost?: boolean
    pub shield_boost: Option<bool>,
    /// From booster flag (protosynthesis/quark drive)
    /// JavaScript: fromBooster?: boolean
    pub from_booster: Option<bool>,
    /// Best stat (protosynthesis/quark drive)
    /// JavaScript: bestStat?: string
    pub best_stat: Option<String>,
    /// Libero triggered flag
    /// JavaScript: libero?: boolean
    pub libero: Option<bool>,
    /// Seek flag (trace)
    /// JavaScript: seek?: boolean
    pub seek: Option<bool>,
    /// Resisted flag (tera shell)
    /// JavaScript: resisted?: boolean
    pub resisted: Option<bool>,
    /// Checked anger shell flag (anger shell ability)
    /// JavaScript: checkedAngerShell?: boolean
    pub checked_anger_shell: Option<bool>,
    /// Checked berserk flag (berserk ability)
    /// JavaScript: checkedBerserk?: boolean
    pub checked_berserk: Option<bool>,
    /// Busted flag (disguise/iceface abilities)
    /// JavaScript: busted?: boolean
    pub busted: Option<bool>,
    /// Berry weaken flag (ripen ability)
    /// JavaScript: berryWeaken?: boolean
    pub berry_weaken: Option<bool>,

    // ========== Item/boost state fields ==========

    /// Accumulated boosts (opportunist, mirror herb, white herb)
    /// JavaScript: boosts?: BoostsTable
    pub boosts: Option<crate::dex_data::BoostsTable>,
    /// Ready flag (mirror herb)
    /// JavaScript: ready?: boolean
    pub ready: Option<bool>,
    /// Eject flag (eject pack)
    /// JavaScript: eject?: boolean
    pub eject: Option<bool>,
    /// Started flag (booster energy)
    /// JavaScript: started?: boolean
    pub started: Option<bool>,
    /// Weather suppressed flag (utility umbrella)
    /// JavaScript: weatherSuppress?: boolean
    pub weather_suppress: Option<bool>,
    /// Inactive flag (utility umbrella)
    /// JavaScript: inactive?: boolean
    pub inactive: Option<bool>,

    // ========== Move-specific fields ==========

    /// Allies list for beat up
    /// JavaScript: allies?: Pokemon[]
    pub allies: Option<Vec<(usize, usize)>>,
    /// Sources list for pursuit side condition
    /// JavaScript: sources?: Pokemon[]
    pub sources: Option<Vec<(usize, usize)>>,
    /// Magnitude value for Magnitude move
    /// JavaScript: magnitude?: number
    pub magnitude: Option<i32>,
    /// Move data for future moves (Doom Desire, Future Sight)
    /// JavaScript: moveData?: object
    pub move_data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl EffectState {
    /// Create a new EffectState with the given ID
    pub fn new(id: ID) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Get layers count (now uses typed field)
    pub fn get_layers(&self) -> i32 {
        self.layers.unwrap_or(0)
    }

    /// Set layers count (now uses typed field)
    pub fn set_layers(&mut self, layers: i32) {
        self.layers = Some(layers);
    }

    /// Get hasDragonType flag (now uses typed field)
    pub fn get_has_dragon_type(&self) -> bool {
        self.has_dragon_type.unwrap_or(false)
    }

    /// Set hasDragonType flag (now uses typed field)
    pub fn set_has_dragon_type(&mut self, value: bool) {
        self.has_dragon_type = Some(value);
    }

    /// Get hp value (now uses typed field)
    pub fn get_hp(&self) -> Option<i32> {
        self.hp
    }

    /// Set hp value (now uses typed field)
    pub fn set_hp(&mut self, hp: i32) {
        self.hp = Some(hp);
    }

    /// Get starting turn (now uses typed field)
    pub fn get_starting_turn(&self) -> Option<i32> {
        self.starting_turn
    }

    /// Set starting turn (now uses typed field)
    pub fn set_starting_turn(&mut self, turn: i32) {
        self.starting_turn = Some(turn);
    }

    /// Get lost focus flag (now uses typed field)
    pub fn get_lost_focus(&self) -> bool {
        self.lost_focus.unwrap_or(false)
    }

    /// Set lost focus flag (now uses typed field)
    pub fn set_lost_focus(&mut self, value: bool) {
        self.lost_focus = Some(value);
    }

    /// Get source slot (now uses typed field)
    pub fn get_source_slot(&self) -> Option<usize> {
        self.source_slot
    }

    /// Set source slot (now uses typed field)
    pub fn set_source_slot(&mut self, slot: usize) {
        self.source_slot = Some(slot);
    }
}
