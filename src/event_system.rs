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

use crate::dex_data::ID;

/// Event handler result - what a handler can return
#[derive(Debug)]
pub enum EventResult {
    /// Handler returned undefined - continue with current relayVar
    Continue,
    /// Handler returned a boolean value
    Bool(bool),
    /// Handler returned a numeric value
    Number(f64),
    /// Handler returned an integer value
    Int(i32),
    /// Handler returned false/null - stop processing
    Stop,
}

/// Effect types - matches JavaScript effectType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Ability,
    Item,
    Move,
    Status,
    Volatile,
    Weather,
    Terrain,
    SideCondition,
    FieldCondition,
    Format,
    Rule,
    ValidatorRule,
}

impl EffectType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Ability" => Some(Self::Ability),
            "Item" => Some(Self::Item),
            "Move" => Some(Self::Move),
            "Status" => Some(Self::Status),
            "Volatile" => Some(Self::Volatile),
            "Weather" => Some(Self::Weather),
            "Terrain" => Some(Self::Terrain),
            "SideCondition" => Some(Self::SideCondition),
            "FieldCondition" => Some(Self::FieldCondition),
            "Format" => Some(Self::Format),
            "Rule" => Some(Self::Rule),
            "ValidatorRule" => Some(Self::ValidatorRule),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Ability => "Ability",
            Self::Item => "Item",
            Self::Move => "Move",
            Self::Status => "Status",
            Self::Volatile => "Volatile",
            Self::Weather => "Weather",
            Self::Terrain => "Terrain",
            Self::SideCondition => "SideCondition",
            Self::FieldCondition => "FieldCondition",
            Self::Format => "Format",
            Self::Rule => "Rule",
            Self::ValidatorRule => "ValidatorRule",
        }
    }
}

/// Effect metadata - represents an effect with its properties
/// Equivalent to Effect interface in TypeScript
#[derive(Debug, Clone)]
pub struct EffectData {
    /// Effect name/ID
    pub name: String,
    /// Effect type (Ability, Move, Item, etc.)
    pub effect_type: EffectType,
    /// Whether this effect was Prankster boosted
    pub prankster_boosted: bool,
}

impl EffectData {
    pub fn new(name: String, effect_type: EffectType) -> Self {
        Self {
            name,
            effect_type,
            prankster_boosted: false,
        }
    }

    pub fn with_prankster(mut self, prankster_boosted: bool) -> Self {
        self.prankster_boosted = prankster_boosted;
        self
    }
}

// EventInfo is defined in battle.rs - use that instead of duplicating

/// Effect state - matches JavaScript EffectState
/// Stores state for temporary effects (volatiles, side conditions, etc.)
#[derive(Debug, Clone, Default)]
pub struct EffectState {
    /// Effect ID
    pub id: ID,
    /// Target of the effect
    pub target: Option<(usize, usize)>,
    /// Source of the effect
    pub source: Option<(usize, usize)>,
    /// Duration remaining (turns)
    pub duration: Option<i32>,
    /// Time (for sorting)
    pub time: Option<i32>,
    /// Custom data storage (for effect-specific state)
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

impl EffectState {
    /// Get a boolean field from custom data
    /// Equivalent to volatile.fieldName or effectState.fieldName in TypeScript
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.data.get(key).and_then(|v| v.as_bool())
    }

    /// Set a boolean field in custom data
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.data.insert(key.to_string(), serde_json::Value::Bool(value));
    }

    /// Get an integer field from custom data
    /// Equivalent to volatile.fieldName or effectState.fieldName in TypeScript
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.data.get(key).and_then(|v| v.as_i64()).map(|n| n as i32)
    }

    /// Set an integer field in custom data
    pub fn set_i32(&mut self, key: &str, value: i32) {
        self.data.insert(key.to_string(), serde_json::Value::Number(value.into()));
    }

    /// Get a usize field from custom data (for slot indices)
    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.data.get(key).and_then(|v| v.as_u64()).map(|n| n as usize)
    }

    /// Set a usize field in custom data
    pub fn set_usize(&mut self, key: &str, value: usize) {
        self.data.insert(key.to_string(), serde_json::Value::Number((value as u64).into()));
    }

    /// Get source slot from custom data
    /// Equivalent to volatile.sourceSlot or effectState.sourceSlot in TypeScript
    pub fn get_source_slot(&self) -> Option<usize> {
        self.get_usize("sourceSlot")
    }

    /// Set source slot in custom data
    pub fn set_source_slot(&mut self, slot: usize) {
        self.set_usize("sourceSlot", slot);
    }

    /// Check if lostFocus flag is set
    /// Equivalent to effectState.lostFocus in TypeScript (focuspunch)
    pub fn get_lost_focus(&self) -> bool {
        self.get_bool("lostFocus").unwrap_or(false)
    }

    /// Set lostFocus flag
    pub fn set_lost_focus(&mut self, value: bool) {
        self.set_bool("lostFocus", value);
    }

    /// Get layers count
    /// Equivalent to volatile.layers in TypeScript (gmaxchistrike, psychup)
    pub fn get_layers(&self) -> i32 {
        self.get_i32("layers").unwrap_or(0)
    }

    /// Set layers count
    pub fn set_layers(&mut self, layers: i32) {
        self.set_i32("layers", layers);
    }

    /// Get hasDragonType flag
    /// Equivalent to volatile.hasDragonType in TypeScript (dragoncheer)
    pub fn get_has_dragon_type(&self) -> bool {
        self.get_bool("hasDragonType").unwrap_or(false)
    }

    /// Set hasDragonType flag
    pub fn set_has_dragon_type(&mut self, value: bool) {
        self.set_bool("hasDragonType", value);
    }

    /// Get hp value
    /// Equivalent to effectState.hp in TypeScript (wish)
    pub fn get_hp(&self) -> Option<i32> {
        self.get_i32("hp")
    }

    /// Set hp value
    pub fn set_hp(&mut self, hp: i32) {
        self.set_i32("hp", hp);
    }

    /// Get starting turn
    /// Equivalent to effectState.startingTurn in TypeScript (wish)
    pub fn get_starting_turn(&self) -> Option<i32> {
        self.get_i32("startingTurn")
    }

    /// Set starting turn
    pub fn set_starting_turn(&mut self, turn: i32) {
        self.set_i32("startingTurn", turn);
    }

    /// Get pranksterBoosted flag
    /// Equivalent to effectState.pranksterBoosted in TypeScript (magiccoat)
    pub fn get_prankster_boosted(&self) -> bool {
        self.get_bool("pranksterBoosted").unwrap_or(false)
    }

    /// Set pranksterBoosted flag
    pub fn set_prankster_boosted(&mut self, value: bool) {
        self.set_bool("pranksterBoosted", value);
    }
}

/// Event handler with priority information
#[derive(Debug, Clone)]
pub struct EventHandler {
    /// Effect that owns this handler
    pub effect_id: ID,
    /// Type of effect
    pub effect_type: EffectType,
    /// Priority value (for move events)
    pub priority: i32,
    /// Sub-order for same priority
    pub sub_order: i32,
    /// Speed stat (for speed-based sorting)
    pub speed: i32,
    /// Pokemon that holds this effect
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (for special sorting)
    pub order: i32,
}

impl EventHandler {
    pub fn new(effect_id: ID, effect_type: EffectType) -> Self {
        Self {
            effect_id,
            effect_type,
            priority: 0,
            sub_order: 0,
            speed: 0,
            effect_holder: None,
            order: 0,
        }
    }
}

/// Event callback signature
/// Takes (battle, args) and returns an EventResult
pub type EventCallback = fn(&mut crate::battle::Battle) -> EventResult;

/// Represents an effect that can have event handlers
pub trait Effect {
    /// Get the effect ID
    fn id(&self) -> &ID;

    /// Get the effect type
    fn effect_type(&self) -> EffectType;

    /// Check if this effect has a breakable flag (for Mold Breaker suppression)
    fn is_breakable(&self) -> bool {
        false
    }

    /// Get event callback for a given event ID
    fn get_event_callback(&self, event_id: &str) -> Option<EventCallback> {
        None
    }
}
