//! Item Effect System
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles item effects in battle using a hybrid approach:
//! 1. Data-driven effects (ItemEffect) for simple items
//! 2. Handler functions for complex items
//!
//! The battle engine should use run_item_event() as the main entry point.

use crate::dex_data::ID;
use crate::event::{EventType, EventResult};
use crate::data::items::{get_item, ItemDef};
use crate::data::item_effects::{ItemEffect, Stat, Trigger};
use crate::item_handlers::{get_handler, ItemContext};

/// Item effect types for different game events (legacy enum for compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemEvent {
    /// When Pokemon switches in
    OnSwitchIn,
    /// At end of turn (residual)
    OnResidual,
    /// Before a move is used
    BeforeMove,
    /// After a move hits
    AfterMoveHit,
    /// When calculating damage
    OnModifyDamage,
    /// When calculating stats
    OnModifyStat,
    /// When taking damage
    OnTakeDamage,
    /// When HP drops below 50%
    OnHPBelowHalf,
    /// When HP drops below 25%
    OnHPBelowQuarter,
    /// When receiving a status
    OnSetStatus,
    /// When calculating accuracy
    OnModifyAccuracy,
    /// When calculating critical hit
    OnModifyCritRatio,
    /// When fainted
    OnFaint,
    /// On contact with foe
    OnContact,
}

impl ItemEvent {
    /// Convert to EventType
    pub fn to_event_type(&self) -> EventType {
        match self {
            ItemEvent::OnSwitchIn => EventType::SwitchIn,
            ItemEvent::OnResidual => EventType::Residual,
            ItemEvent::BeforeMove => EventType::BeforeMove,
            ItemEvent::AfterMoveHit => EventType::AfterHit,
            ItemEvent::OnModifyDamage => EventType::ModifyDamage,
            ItemEvent::OnModifyStat => EventType::ModifyAtk, // Generic stat
            ItemEvent::OnTakeDamage => EventType::Damage,
            ItemEvent::OnHPBelowHalf => EventType::Update,
            ItemEvent::OnHPBelowQuarter => EventType::Update,
            ItemEvent::OnSetStatus => EventType::SetStatus,
            ItemEvent::OnModifyAccuracy => EventType::ModifyAccuracy,
            ItemEvent::OnModifyCritRatio => EventType::ModifyCritRatio,
            ItemEvent::OnFaint => EventType::Residual,
            ItemEvent::OnContact => EventType::DamagingHit,
        }
    }
}

/// Modifiers returned by item effects
#[derive(Debug, Clone, Default)]
pub struct ItemModifier {
    /// Multiply damage by this factor (1.0 = no change)
    pub damage_multiplier: Option<f64>,
    /// Multiply stat by this factor
    pub stat_multiplier: Option<f64>,
    /// Block the action entirely
    pub block: bool,
    /// Additional message to log
    pub message: Option<String>,
    /// Boost to apply
    pub boost: Option<(String, i8)>,
    /// Status to apply
    pub apply_status: Option<String>,
    /// Prevent status
    pub prevent_status: bool,
    /// Heal amount (fraction of max HP)
    pub heal_fraction: Option<f64>,
    /// Damage amount (fraction of max HP)
    pub damage_fraction: Option<f64>,
    /// Consume the item
    pub consume: bool,
    /// Restore PP
    pub restore_pp: Option<u8>,
    /// Critical ratio modifier
    pub crit_ratio_boost: Option<i8>,
    /// Accuracy modifier
    pub accuracy_multiplier: Option<f64>,
    /// Priority modifier
    pub priority_modifier: Option<i8>,
    /// Speed multiplier
    pub speed_multiplier: Option<f64>,
}

impl ItemModifier {
    /// Convert from EventResult
    pub fn from_event_result(result: &EventResult) -> Option<Self> {
        match result {
            EventResult::Modify(m) => Some(ItemModifier {
                stat_multiplier: Some(*m),
                ..Default::default()
            }),
            EventResult::Heal(h) => Some(ItemModifier {
                heal_fraction: Some(*h as f64),
                ..Default::default()
            }),
            EventResult::Damage(d) => Some(ItemModifier {
                damage_fraction: Some(*d as f64),
                ..Default::default()
            }),
            EventResult::Fail => Some(ItemModifier {
                block: true,
                ..Default::default()
            }),
            _ => None,
        }
    }
}

/// Item effect handler result
pub type ItemResult = Option<ItemModifier>;

/// Apply an ItemEffect and return the result
pub fn apply_item_effect(effect: &ItemEffect, ctx: &ItemContext) -> EventResult {
    match effect {
        // Stat modifiers
        ItemEffect::ModifyStat { multiplier, .. } => {
            EventResult::Modify(*multiplier)
        }

        ItemEffect::BoostType { move_type, multiplier } => {
            if ctx.move_type.as_deref() == Some(move_type.as_str()) {
                EventResult::Modify(*multiplier)
            } else {
                EventResult::Continue
            }
        }

        ItemEffect::BoostDamage { multiplier } => {
            EventResult::Modify(*multiplier)
        }

        // HP effects
        ItemEffect::ResidualHeal { fraction } => {
            // Return as a modifier; battle will interpret
            EventResult::Modify(*fraction)
        }

        ItemEffect::ResidualHealOrDamage { required_type, heal_fraction, damage_fraction } => {
            // Check if holder has the required type
            if ctx.target_types.iter().any(|t| t.eq_ignore_ascii_case(required_type)) {
                EventResult::Modify(*heal_fraction)
            } else {
                EventResult::Modify(-damage_fraction) // Negative = damage
            }
        }

        ItemEffect::HealOnThreshold { threshold, heal_fraction } => {
            if let Some(hp_frac) = ctx.hp_fraction {
                if hp_frac <= *threshold {
                    return EventResult::Modify(*heal_fraction);
                }
            }
            EventResult::Continue
        }

        ItemEffect::RecoilOnAttack { fraction } => {
            EventResult::Modify(-fraction) // Negative = damage to self
        }

        // Status effects
        ItemEffect::CureStatus { status } => {
            if ctx.status.as_deref() == Some(status.as_str()) {
                EventResult::Stop // Block the status
            } else {
                EventResult::Continue
            }
        }

        ItemEffect::CureAllStatus => {
            EventResult::Stop
        }

        ItemEffect::PreventStatus { status } => {
            if ctx.status.as_deref() == Some(status.as_str()) {
                EventResult::Fail
            } else {
                EventResult::Continue
            }
        }

        // Boost effects
        ItemEffect::BoostStats { boosts } => {
            // Return the first boost; battle will handle applying all
            if let Some((stat, stages)) = boosts.first() {
                EventResult::ModifyInt(*stages as i32)
            } else {
                EventResult::Continue
            }
        }

        ItemEffect::PreventStatDrops => {
            if let Some((_, change)) = &ctx.boost {
                if *change < 0 && !ctx.is_source {
                    return EventResult::Fail;
                }
            }
            EventResult::Continue
        }

        ItemEffect::RestoreLoweredStats => {
            EventResult::Stop // Signal that stats should be restored
        }

        // Type resistance
        ItemEffect::ResistSuperEffective { resist_type, multiplier } => {
            if ctx.move_type.as_deref() == Some(resist_type.as_str()) {
                if let Some(eff) = ctx.type_effectiveness {
                    if eff > 1.0 {
                        return EventResult::Modify(*multiplier);
                    }
                }
            }
            EventResult::Continue
        }

        // Move modification
        ItemEffect::ChoiceLock { stat, multiplier } => {
            // Return the multiplier; battle handles the lock
            EventResult::Modify(*multiplier)
        }

        ItemEffect::BoostCritRatio { stages } => {
            EventResult::ModifyInt(*stages as i32)
        }

        ItemEffect::BoostAccuracy { multiplier } => {
            EventResult::Modify(*multiplier)
        }

        ItemEffect::ReduceOpponentAccuracy { multiplier } => {
            EventResult::Modify(*multiplier)
        }

        ItemEffect::PriorityChance { chance } => {
            // Battle should roll for this
            EventResult::Modify(*chance)
        }

        ItemEffect::SkipChargeTurn => {
            EventResult::Stop
        }

        ItemEffect::MaxMultiHit => {
            EventResult::ModifyInt(5) // Max 5 hits
        }

        // Defensive effects
        ItemEffect::FocusSash => {
            if let Some(hp_frac) = ctx.hp_fraction {
                if hp_frac >= 1.0 {
                    return EventResult::Stop; // Survive at 1 HP
                }
            }
            EventResult::Continue
        }

        ItemEffect::ContactDamage { fraction } => {
            if ctx.is_contact && !ctx.is_source {
                EventResult::Modify(*fraction)
            } else {
                EventResult::Continue
            }
        }

        ItemEffect::HazardImmunity => {
            EventResult::Stop
        }

        ItemEffect::WeatherImmunity => {
            EventResult::Stop
        }

        ItemEffect::PowderImmunity => {
            EventResult::Stop
        }

        ItemEffect::TypeImmunity { immune_type } => {
            if ctx.move_type.as_deref() == Some(immune_type.as_str()) {
                EventResult::Fail
            } else {
                EventResult::Continue
            }
        }

        // Grounding
        ItemEffect::GroundHolder => {
            EventResult::True
        }

        ItemEffect::Airborne => {
            EventResult::True
        }

        // Switching
        ItemEffect::EjectOnHit => {
            if ctx.damage.is_some() {
                EventResult::Override("eject".to_string())
            } else {
                EventResult::Continue
            }
        }

        ItemEffect::ForceOpponentSwitch => {
            EventResult::Override("forceswitch".to_string())
        }

        ItemEffect::EscapeTrapping => {
            EventResult::True
        }

        // Volatile status
        ItemEffect::AddVolatile { volatile } => {
            EventResult::Override(volatile.clone())
        }

        ItemEffect::RemoveVolatile { volatile } => {
            EventResult::Override(format!("remove:{}", volatile))
        }

        ItemEffect::MentalHerb => {
            EventResult::Stop
        }

        // Consumption
        ItemEffect::ConsumeItem => {
            EventResult::Stop
        }

        ItemEffect::CannotRemove => {
            EventResult::Fail
        }

        ItemEffect::IgnoreKlutz => {
            EventResult::True
        }

        // Fling
        ItemEffect::FlingEffect { effect } => {
            apply_item_effect(effect, ctx)
        }

        // Compound effects
        ItemEffect::Compound { effects } => {
            for eff in effects {
                let result = apply_item_effect(eff, ctx);
                if !matches!(result, EventResult::Continue) {
                    return result;
                }
            }
            EventResult::Continue
        }

        // Triggered effects
        ItemEffect::Triggered { trigger, effect } => {
            let should_trigger = match trigger {
                Trigger::OnDamagingHitType(t) => {
                    ctx.damage.is_some() && ctx.move_type.as_deref() == Some(t.as_str())
                }
                Trigger::OnHPBelow(threshold) => {
                    ctx.hp_fraction.map_or(false, |hp| hp <= *threshold)
                }
                Trigger::OnHPBelowGluttony(threshold) => {
                    // Gluttony triggers at 50% instead of 25%
                    ctx.holder_ability.as_deref() == Some("gluttony")
                        && ctx.hp_fraction.map_or(false, |hp| hp <= 0.5)
                        || ctx.hp_fraction.map_or(false, |hp| hp <= *threshold)
                }
                Trigger::OnStatus(status) => {
                    ctx.status.as_deref() == Some(status.as_str())
                }
                Trigger::OnAnyStatus => {
                    ctx.status.is_some()
                }
                Trigger::OnSuperEffectiveHit(t) => {
                    ctx.type_effectiveness.map_or(false, |eff| eff > 1.0)
                        && ctx.move_type.as_deref() == Some(t.as_str())
                }
                Trigger::OnResidual => true,
                Trigger::OnSwitchIn => true,
                Trigger::OnUseMove => true,
                Trigger::OnStatDrop => {
                    ctx.boost.as_ref().map_or(false, |(_, change)| *change < 0)
                }
                Trigger::OnContactHit => {
                    ctx.is_contact && ctx.damage.is_some()
                }
            };

            if should_trigger {
                apply_item_effect(effect, ctx)
            } else {
                EventResult::Continue
            }
        }

        _ => EventResult::Continue,
    }
}

/// Main entry point: Run item event using the hybrid system
///
/// This function checks:
/// 1. First, custom handlers for complex items
/// 2. Then, data-driven effects from ItemDef
///
/// Returns the result of the first matching handler/effect
pub fn run_item_event(item_id: &ID, event: &EventType, ctx: &ItemContext) -> EventResult {
    // 1. Check for custom handler first
    if let Some(handler) = get_handler(item_id, event) {
        let result = handler(ctx);
        if !matches!(result, EventResult::Continue) {
            return result;
        }
    }

    // 2. Check data-driven effects
    if let Some(item_def) = get_item(item_id) {
        for effect in item_def.get_effects_for_event(event) {
            let result = apply_item_effect(effect, ctx);
            if !matches!(result, EventResult::Continue) {
                return result;
            }
        }
    }

    EventResult::Continue
}

/// Legacy function for backward compatibility
/// Get item effect for a specific item and event using the old match-based system
pub fn get_item_effect(item_id: &ID, event: ItemEvent) -> ItemResult {
    let ctx = ItemContext::new();
    let event_type = event.to_event_type();

    let result = run_item_event(item_id, &event_type, &ctx);
    ItemModifier::from_event_result(&result)
}

/// Check if an item provides type damage boost
pub fn get_item_type_boost(item_id: &ID) -> Option<(&'static str, f64)> {
    let item = item_id.as_str();
    match item {
        "charcoal" | "flameplate" => Some(("Fire", 1.2)),
        "mysticwater" | "splashplate" | "seaincense" | "waveincense" => Some(("Water", 1.2)),
        "miracleseed" | "meadowplate" | "roseincense" => Some(("Grass", 1.2)),
        "magnet" | "zapplate" => Some(("Electric", 1.2)),
        "nevermeltice" | "icicleplate" => Some(("Ice", 1.2)),
        "blackbelt" | "fistplate" => Some(("Fighting", 1.2)),
        "poisonbarb" | "toxicplate" => Some(("Poison", 1.2)),
        "softsand" | "earthplate" => Some(("Ground", 1.2)),
        "sharpbeak" | "skyplate" => Some(("Flying", 1.2)),
        "twistedspoon" | "mindplate" | "oddincense" => Some(("Psychic", 1.2)),
        "silverpowder" | "insectplate" => Some(("Bug", 1.2)),
        "hardstone" | "stoneplate" | "rockincense" => Some(("Rock", 1.2)),
        "spelltag" | "spookyplate" => Some(("Ghost", 1.2)),
        "dragonfang" | "dracoplate" => Some(("Dragon", 1.2)),
        "blackglasses" | "dreadplate" => Some(("Dark", 1.2)),
        "metalcoat" | "ironplate" => Some(("Steel", 1.2)),
        "silkscarf" => Some(("Normal", 1.2)),
        "pixieplate" => Some(("Fairy", 1.2)),
        _ => None,
    }
}

/// Check if item provides status immunity
pub fn check_item_prevents_status(item_id: &ID, status: &str) -> bool {
    let item = item_id.as_str();
    match (item, status) {
        ("lumberry", _) => true, // Cures any status
        ("rawstberry", "brn") => true,
        ("cheriberry", "par") => true,
        ("pechaberry", "psn" | "tox") => true,
        ("aspearberry", "frz") => true,
        ("chestoberry", "slp") => true,
        ("mentalherb", "attract" | "taunt" | "encore" | "disable" | "torment") => true,
        ("covertcloak", _) => false, // Protects from secondary effects, not primary status
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_item_event_no_effect() {
        let ctx = ItemContext::new();
        let result = run_item_event(&ID::new("nonexistent"), &EventType::Residual, &ctx);
        assert!(matches!(result, EventResult::Continue));
    }

    #[test]
    fn test_apply_residual_heal_effect() {
        let effect = ItemEffect::ResidualHeal { fraction: 0.0625 };
        let ctx = ItemContext::new();
        let result = apply_item_effect(&effect, &ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 0.0625).abs() < 0.001));
    }

    #[test]
    fn test_apply_type_boost_effect() {
        let effect = ItemEffect::BoostType {
            move_type: "Fire".to_string(),
            multiplier: 1.2,
        };

        // Matching type
        let ctx = ItemContext::new().with_move("flamethrower", "Fire", "Special");
        let result = apply_item_effect(&effect, &ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 1.2).abs() < 0.001));

        // Non-matching type
        let ctx = ItemContext::new().with_move("thunderbolt", "Electric", "Special");
        let result = apply_item_effect(&effect, &ctx);
        assert!(matches!(result, EventResult::Continue));
    }

    #[test]
    fn test_apply_triggered_effect() {
        let effect = ItemEffect::Triggered {
            trigger: Trigger::OnHPBelow(0.5),
            effect: Box::new(ItemEffect::HealOnThreshold {
                threshold: 0.5,
                heal_fraction: 0.25,
            }),
        };

        // HP at 40% - should trigger
        let ctx = ItemContext::new().with_hp_fraction(0.4);
        let result = apply_item_effect(&effect, &ctx);
        assert!(matches!(result, EventResult::Modify(m) if (m - 0.25).abs() < 0.001));

        // HP at 60% - should not trigger
        let ctx = ItemContext::new().with_hp_fraction(0.6);
        let result = apply_item_effect(&effect, &ctx);
        assert!(matches!(result, EventResult::Continue));
    }

    #[test]
    fn test_apply_compound_effect() {
        let effect = ItemEffect::Compound {
            effects: vec![
                ItemEffect::BoostDamage { multiplier: 1.3 },
                ItemEffect::RecoilOnAttack { fraction: 0.1 },
            ],
        };

        let ctx = ItemContext::new();
        let result = apply_item_effect(&effect, &ctx);
        // Should return first non-Continue result
        assert!(matches!(result, EventResult::Modify(m) if (m - 1.3).abs() < 0.001));
    }

    #[test]
    fn test_get_item_type_boost() {
        assert_eq!(get_item_type_boost(&ID::new("charcoal")), Some(("Fire", 1.2)));
        assert_eq!(get_item_type_boost(&ID::new("mysticwater")), Some(("Water", 1.2)));
        assert_eq!(get_item_type_boost(&ID::new("leftovers")), None);
    }

    #[test]
    fn test_check_item_prevents_status() {
        assert!(check_item_prevents_status(&ID::new("lumberry"), "par"));
        assert!(check_item_prevents_status(&ID::new("lumberry"), "brn"));
        assert!(check_item_prevents_status(&ID::new("rawstberry"), "brn"));
        assert!(!check_item_prevents_status(&ID::new("rawstberry"), "par"));
    }

    #[test]
    fn test_item_event_to_event_type() {
        assert_eq!(ItemEvent::OnResidual.to_event_type(), EventType::Residual);
        assert_eq!(ItemEvent::OnSwitchIn.to_event_type(), EventType::SwitchIn);
        assert_eq!(ItemEvent::OnSetStatus.to_event_type(), EventType::SetStatus);
    }
}
