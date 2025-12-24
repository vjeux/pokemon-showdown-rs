//! Item Handler Functions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains handler functions for items with complex behavior
//! that cannot be expressed declaratively using ItemEffect.
//!
//! Most items (~80%) use the data-driven ItemEffect system.
//! Complex items (~20%) use handler functions defined here.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::{EventType, EventResult};

/// Context passed to item handlers
#[derive(Debug, Clone, Default)]
pub struct ItemContext {
    /// The move being used (if applicable)
    pub move_id: Option<ID>,
    /// The move's type (if applicable)
    pub move_type: Option<String>,
    /// The move's category (Physical, Special, Status)
    pub move_category: Option<String>,
    /// Damage being dealt/taken
    pub damage: Option<u32>,
    /// Is this a contact move?
    pub is_contact: bool,
    /// Type effectiveness multiplier
    pub type_effectiveness: Option<f64>,
    /// Source Pokemon's types
    pub source_types: Vec<String>,
    /// Target Pokemon's types
    pub target_types: Vec<String>,
    /// Current HP fraction (0.0 to 1.0)
    pub hp_fraction: Option<f64>,
    /// Status being applied
    pub status: Option<String>,
    /// Boost being applied
    pub boost: Option<(String, i8)>,
    /// Is this from the source's perspective?
    pub is_source: bool,
    /// Weather active
    pub weather: Option<String>,
    /// Terrain active
    pub terrain: Option<String>,
    /// Is holder dynamaxed?
    pub is_dynamaxed: bool,
    /// Does holder have a specific ability?
    pub holder_ability: Option<String>,
}

impl ItemContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_move(mut self, move_id: &str, move_type: &str, category: &str) -> Self {
        self.move_id = Some(ID::new(move_id));
        self.move_type = Some(move_type.to_string());
        self.move_category = Some(category.to_string());
        self
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = Some(damage);
        self
    }

    pub fn with_contact(mut self, is_contact: bool) -> Self {
        self.is_contact = is_contact;
        self
    }

    pub fn with_hp_fraction(mut self, fraction: f64) -> Self {
        self.hp_fraction = Some(fraction);
        self
    }

    pub fn with_status(mut self, status: &str) -> Self {
        self.status = Some(status.to_string());
        self
    }

    pub fn with_effectiveness(mut self, effectiveness: f64) -> Self {
        self.type_effectiveness = Some(effectiveness);
        self
    }

    pub fn with_holder_ability(mut self, ability: &str) -> Self {
        self.holder_ability = Some(ability.to_string());
        self
    }

    pub fn with_dynamax(mut self, is_dynamaxed: bool) -> Self {
        self.is_dynamaxed = is_dynamaxed;
        self
    }
}

/// Handler function signature
/// Returns EventResult to indicate what action to take
pub type ItemHandler = fn(&ItemContext) -> EventResult;

/// Collection of handlers for different events
#[derive(Clone, Default)]
pub struct ItemHandlers {
    pub on_start: Option<ItemHandler>,
    pub on_switch_in: Option<ItemHandler>,
    pub on_switch_out: Option<ItemHandler>,
    pub on_residual: Option<ItemHandler>,
    pub on_before_move: Option<ItemHandler>,
    pub on_modify_move: Option<ItemHandler>,
    pub on_modify_atk: Option<ItemHandler>,
    pub on_modify_def: Option<ItemHandler>,
    pub on_modify_spa: Option<ItemHandler>,
    pub on_modify_spd: Option<ItemHandler>,
    pub on_modify_spe: Option<ItemHandler>,
    pub on_modify_accuracy: Option<ItemHandler>,
    pub on_modify_crit_ratio: Option<ItemHandler>,
    pub on_modify_damage: Option<ItemHandler>,
    pub on_base_power: Option<ItemHandler>,
    pub on_source_modify_damage: Option<ItemHandler>,
    pub on_damaging_hit: Option<ItemHandler>,
    pub on_after_hit: Option<ItemHandler>,
    pub on_try_hit: Option<ItemHandler>,
    pub on_take_item: Option<ItemHandler>,
    pub on_try_boost: Option<ItemHandler>,
    pub on_after_boost: Option<ItemHandler>,
    pub on_set_status: Option<ItemHandler>,
    pub on_update: Option<ItemHandler>,
    pub on_eat: Option<ItemHandler>,
    pub on_try_eat_item: Option<ItemHandler>,
    pub on_try_heal: Option<ItemHandler>,
    pub on_faint: Option<ItemHandler>,
    pub on_disable_move: Option<ItemHandler>,
}

impl ItemHandlers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get handler for a specific event
    pub fn get_handler(&self, event: &EventType) -> Option<ItemHandler> {
        match event {
            EventType::Start => self.on_start,
            EventType::SwitchIn => self.on_switch_in,
            EventType::SwitchOut => self.on_switch_out,
            EventType::Residual => self.on_residual,
            EventType::BeforeMove => self.on_before_move,
            EventType::ModifyMove => self.on_modify_move,
            EventType::ModifyAtk => self.on_modify_atk,
            EventType::ModifyDef => self.on_modify_def,
            EventType::ModifySpA => self.on_modify_spa,
            EventType::ModifySpD => self.on_modify_spd,
            EventType::ModifySpe => self.on_modify_spe,
            EventType::ModifyAccuracy => self.on_modify_accuracy,
            EventType::ModifyCritRatio => self.on_modify_crit_ratio,
            EventType::ModifyDamage => self.on_modify_damage,
            EventType::BasePower => self.on_base_power,
            EventType::SourceModifyDamage => self.on_source_modify_damage,
            EventType::DamagingHit => self.on_damaging_hit,
            EventType::AfterHit => self.on_after_hit,
            EventType::TryHit => self.on_try_hit,
            EventType::TryBoost => self.on_try_boost,
            EventType::AfterBoost => self.on_after_boost,
            EventType::SetStatus => self.on_set_status,
            EventType::Update => self.on_update,
            EventType::TryHeal => self.on_try_heal,
            _ => None,
        }
    }
}

// ============================================================================
// Handler Functions for Complex Items
// ============================================================================

/// Booster Energy - activates Protosynthesis/Quark Drive
fn booster_energy_on_update(ctx: &ItemContext) -> EventResult {
    // Booster Energy activates if holder has Protosynthesis (without sun)
    // or Quark Drive (without Electric Terrain)
    match ctx.holder_ability.as_deref() {
        Some("protosynthesis") if ctx.weather.as_deref() != Some("sunnyday") => {
            EventResult::Override("protosynthesis".to_string())
        }
        Some("quarkdrive") if ctx.terrain.as_deref() != Some("electricterrain") => {
            EventResult::Override("quarkdrive".to_string())
        }
        _ => EventResult::Continue,
    }
}

/// Clear Amulet - prevents stat drops from opponents
fn clear_amulet_on_try_boost(ctx: &ItemContext) -> EventResult {
    if let Some((_, change)) = &ctx.boost {
        if *change < 0 && !ctx.is_source {
            // Block negative boosts from opponents
            return EventResult::Fail;
        }
    }
    EventResult::Continue
}

/// Assault Vest - prevents using status moves
fn assault_vest_on_disable_move(ctx: &ItemContext) -> EventResult {
    if ctx.move_category.as_deref() == Some("Status") {
        // Me First is an exception
        if ctx.move_id.as_ref().map(|id| id.as_str()) != Some("mefirst") {
            return EventResult::Fail;
        }
    }
    EventResult::Continue
}

/// Assault Vest - 1.5x SpD
fn assault_vest_on_modify_spd(ctx: &ItemContext) -> EventResult {
    EventResult::Modify(1.5)
}

/// Eviolite - 1.5x Def/SpD for not fully evolved Pokemon
/// Note: The actual check for evolution status happens in battle
fn eviolite_on_modify_def(ctx: &ItemContext) -> EventResult {
    EventResult::Modify(1.5)
}

fn eviolite_on_modify_spd(ctx: &ItemContext) -> EventResult {
    EventResult::Modify(1.5)
}

/// Expert Belt - 1.2x damage on super-effective hits
fn expert_belt_on_modify_damage(ctx: &ItemContext) -> EventResult {
    if let Some(effectiveness) = ctx.type_effectiveness {
        if effectiveness > 1.0 {
            return EventResult::Modify(1.2);
        }
    }
    EventResult::Continue
}

/// Metronome item - boost damage for consecutive move use
/// The actual counter tracking happens in battle state
fn metronome_on_base_power(ctx: &ItemContext) -> EventResult {
    // This would need access to the metronome counter from battle state
    // For now, return Continue and let battle handle the full logic
    EventResult::Continue
}

/// Big Root - boost draining moves by 30%
fn big_root_on_try_heal(ctx: &ItemContext) -> EventResult {
    // Check if this is a drain move
    if let Some(move_id) = &ctx.move_id {
        let drain_moves = ["drain", "leechseed", "ingrain", "aquaring", "strengthsap"];
        if drain_moves.iter().any(|m| move_id.as_str() == *m) {
            // 5324/4096 = ~1.3
            return EventResult::Modify(1.3);
        }
    }
    EventResult::Continue
}

/// Bright Powder - reduce opponent's accuracy by ~10%
fn bright_powder_on_modify_accuracy(ctx: &ItemContext) -> EventResult {
    // 3686/4096 = ~0.9
    EventResult::Modify(0.9)
}

/// Blunder Policy - boost Speed after missing
/// This is triggered by the battle when a move misses
fn blunder_policy_on_after_hit(ctx: &ItemContext) -> EventResult {
    // The actual miss check happens in battle
    // This handler would be called when appropriate
    EventResult::Continue
}

/// Protective Pads - ignore contact effects
fn protective_pads_on_damaging_hit(ctx: &ItemContext) -> EventResult {
    if ctx.is_contact && ctx.is_source {
        // Block Rocky Helmet, Iron Barbs, etc.
        return EventResult::Stop;
    }
    EventResult::Continue
}

// ============================================================================
// Handler Registry
// ============================================================================

/// Registry of item handlers
pub static ITEM_HANDLERS: Lazy<HashMap<ID, ItemHandlers>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Booster Energy
    m.insert(
        ID::new("boosterenergy"),
        ItemHandlers {
            on_update: Some(booster_energy_on_update),
            ..Default::default()
        },
    );

    // Clear Amulet
    m.insert(
        ID::new("clearamulet"),
        ItemHandlers {
            on_try_boost: Some(clear_amulet_on_try_boost),
            ..Default::default()
        },
    );

    // Assault Vest
    m.insert(
        ID::new("assaultvest"),
        ItemHandlers {
            on_modify_spd: Some(assault_vest_on_modify_spd),
            on_disable_move: Some(assault_vest_on_disable_move),
            ..Default::default()
        },
    );

    // Eviolite
    m.insert(
        ID::new("eviolite"),
        ItemHandlers {
            on_modify_def: Some(eviolite_on_modify_def),
            on_modify_spd: Some(eviolite_on_modify_spd),
            ..Default::default()
        },
    );

    // Expert Belt
    m.insert(
        ID::new("expertbelt"),
        ItemHandlers {
            on_modify_damage: Some(expert_belt_on_modify_damage),
            ..Default::default()
        },
    );

    // Big Root
    m.insert(
        ID::new("bigroot"),
        ItemHandlers {
            on_try_heal: Some(big_root_on_try_heal),
            ..Default::default()
        },
    );

    // Bright Powder
    m.insert(
        ID::new("brightpowder"),
        ItemHandlers {
            on_modify_accuracy: Some(bright_powder_on_modify_accuracy),
            ..Default::default()
        },
    );

    // Protective Pads
    m.insert(
        ID::new("protectivepads"),
        ItemHandlers {
            on_damaging_hit: Some(protective_pads_on_damaging_hit),
            ..Default::default()
        },
    );

    m
});

/// Get handlers for an item
pub fn get_item_handlers(item_id: &ID) -> Option<&'static ItemHandlers> {
    ITEM_HANDLERS.get(item_id)
}

/// Check if an item has custom handlers (complex behavior)
pub fn has_custom_handlers(item_id: &ID) -> bool {
    ITEM_HANDLERS.contains_key(item_id)
}

/// Get a specific handler for an item and event
pub fn get_handler(item_id: &ID, event: &EventType) -> Option<ItemHandler> {
    get_item_handlers(item_id).and_then(|h| h.get_handler(event))
}

/// Run an item handler if it exists
pub fn run_item_handler(item_id: &ID, event: &EventType, ctx: &ItemContext) -> EventResult {
    if let Some(handler) = get_handler(item_id, event) {
        handler(ctx)
    } else {
        EventResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_context_builder() {
        let ctx = ItemContext::new()
            .with_move("thunderbolt", "Electric", "Special")
            .with_damage(100)
            .with_contact(false);

        assert_eq!(ctx.move_id, Some(ID::new("thunderbolt")));
        assert_eq!(ctx.move_type, Some("Electric".to_string()));
        assert_eq!(ctx.damage, Some(100));
        assert!(!ctx.is_contact);
    }

    #[test]
    fn test_assault_vest_handlers() {
        let handlers = get_item_handlers(&ID::new("assaultvest"));
        assert!(handlers.is_some());

        let handlers = handlers.unwrap();
        assert!(handlers.on_modify_spd.is_some());
        assert!(handlers.on_disable_move.is_some());
    }

    #[test]
    fn test_assault_vest_spd_boost() {
        let ctx = ItemContext::new();
        let result = assault_vest_on_modify_spd(&ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 1.5).abs() < 0.001));
    }

    #[test]
    fn test_expert_belt_super_effective() {
        let ctx = ItemContext::new().with_effectiveness(2.0);
        let result = expert_belt_on_modify_damage(&ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 1.2).abs() < 0.001));
    }

    #[test]
    fn test_expert_belt_neutral() {
        let ctx = ItemContext::new().with_effectiveness(1.0);
        let result = expert_belt_on_modify_damage(&ctx);
        assert!(matches!(result, EventResult::Continue));
    }

    #[test]
    fn test_clear_amulet_blocks_drops() {
        let mut ctx = ItemContext::new();
        ctx.boost = Some(("atk".to_string(), -1));
        ctx.is_source = false;

        let result = clear_amulet_on_try_boost(&ctx);
        assert!(matches!(result, EventResult::Fail));
    }

    #[test]
    fn test_clear_amulet_allows_self_drops() {
        let mut ctx = ItemContext::new();
        ctx.boost = Some(("atk".to_string(), -1));
        ctx.is_source = true;

        let result = clear_amulet_on_try_boost(&ctx);
        assert!(matches!(result, EventResult::Continue));
    }

    #[test]
    fn test_has_custom_handlers() {
        assert!(has_custom_handlers(&ID::new("assaultvest")));
        assert!(has_custom_handlers(&ID::new("eviolite")));
        assert!(!has_custom_handlers(&ID::new("leftovers"))); // Uses data-driven
    }

    #[test]
    fn test_run_item_handler() {
        let ctx = ItemContext::new();
        let result = run_item_handler(&ID::new("assaultvest"), &EventType::ModifySpD, &ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 1.5).abs() < 0.001));
    }

    #[test]
    fn test_run_nonexistent_handler() {
        let ctx = ItemContext::new();
        let result = run_item_handler(&ID::new("leftovers"), &EventType::ModifySpD, &ctx);
        assert!(matches!(result, EventResult::Continue));
    }
}
