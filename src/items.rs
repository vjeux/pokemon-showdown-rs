//! Item Effect System
//!
//! This module handles item effects in battle.
//! Items can modify various game mechanics through event handlers.

use crate::dex_data::ID;

/// Item effect types for different game events
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

/// Item effect handler result
pub type ItemResult = Option<ItemModifier>;

/// Get item effect for a specific item and event
pub fn get_item_effect(item_id: &ID, event: ItemEvent) -> ItemResult {
    let item_name = item_id.as_str();

    match (item_name, event) {
        // LEFTOVERS - Heal 1/16 HP at end of turn
        ("leftovers", ItemEvent::OnResidual) => Some(ItemModifier {
            heal_fraction: Some(1.0 / 16.0),
            message: Some("Leftovers restored a little HP!".to_string()),
            ..Default::default()
        }),

        // BLACK SLUDGE - Heal 1/16 HP for Poison types, damage others
        ("blacksludge", ItemEvent::OnResidual) => Some(ItemModifier {
            heal_fraction: Some(1.0 / 16.0), // For Poison types
            message: Some("Black Sludge restored HP!".to_string()),
            ..Default::default()
        }),

        // SITRUS BERRY - Heal 25% HP when below 50%
        ("sitrusberry", ItemEvent::OnHPBelowHalf) => Some(ItemModifier {
            heal_fraction: Some(0.25),
            consume: true,
            message: Some("Sitrus Berry restored HP!".to_string()),
            ..Default::default()
        }),

        // ORAN BERRY - Heal 10 HP when below 50%
        ("oranberry", ItemEvent::OnHPBelowHalf) => Some(ItemModifier {
            heal_fraction: Some(0.10), // Simplified - normally fixed 10 HP
            consume: true,
            message: Some("Oran Berry restored HP!".to_string()),
            ..Default::default()
        }),

        // FIGY BERRY, AGUAV, IAPAPA, MAGO, WIKI - Heal 33% HP when below 25%
        ("figyberry" | "aguavberry" | "iapapaberry" | "magoberry" | "wikiberry", ItemEvent::OnHPBelowQuarter) => Some(ItemModifier {
            heal_fraction: Some(1.0 / 3.0),
            consume: true,
            message: Some("A berry restored HP!".to_string()),
            ..Default::default()
        }),

        // LUM BERRY - Cure any status
        ("lumberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Lum Berry cured the status!".to_string()),
            ..Default::default()
        }),

        // RAWST BERRY - Cure burn
        ("rawstberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Rawst Berry cured the burn!".to_string()),
            ..Default::default()
        }),

        // CHERI BERRY - Cure paralysis
        ("cheriberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Cheri Berry cured paralysis!".to_string()),
            ..Default::default()
        }),

        // PECHA BERRY - Cure poison
        ("pechaberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Pecha Berry cured poison!".to_string()),
            ..Default::default()
        }),

        // ASPEAR BERRY - Cure freeze
        ("aspearberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Aspear Berry thawed the freeze!".to_string()),
            ..Default::default()
        }),

        // CHESTO BERRY - Cure sleep
        ("chestoberry", ItemEvent::OnSetStatus) => Some(ItemModifier {
            prevent_status: true,
            consume: true,
            message: Some("Chesto Berry woke up!".to_string()),
            ..Default::default()
        }),

        // CHOICE BAND - 1.5x Attack, locked into one move
        ("choiceband", ItemEvent::OnModifyStat) => Some(ItemModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // CHOICE SPECS - 1.5x Sp. Atk, locked into one move
        ("choicespecs", ItemEvent::OnModifyStat) => Some(ItemModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // CHOICE SCARF - 1.5x Speed, locked into one move
        ("choicescarf", ItemEvent::OnModifyStat) => Some(ItemModifier {
            stat_multiplier: Some(1.5),
            speed_multiplier: Some(1.5),
            ..Default::default()
        }),

        // LIFE ORB - 1.3x damage, lose 10% HP
        ("lifeorb", ItemEvent::OnModifyDamage) => Some(ItemModifier {
            damage_multiplier: Some(1.3),
            ..Default::default()
        }),
        ("lifeorb", ItemEvent::AfterMoveHit) => Some(ItemModifier {
            damage_fraction: Some(0.1), // Recoil
            ..Default::default()
        }),

        // EXPERT BELT - 1.2x damage for super effective moves
        ("expertbelt", ItemEvent::OnModifyDamage) => Some(ItemModifier {
            damage_multiplier: Some(1.2),
            ..Default::default()
        }),

        // MUSCLE BAND - 1.1x physical damage
        ("muscleband", ItemEvent::OnModifyDamage) => Some(ItemModifier {
            damage_multiplier: Some(1.1),
            ..Default::default()
        }),

        // WISE GLASSES - 1.1x special damage
        ("wiseglasses", ItemEvent::OnModifyDamage) => Some(ItemModifier {
            damage_multiplier: Some(1.1),
            ..Default::default()
        }),

        // EVIOLITE - 1.5x Def/SpD for not fully evolved Pokemon
        ("eviolite", ItemEvent::OnModifyStat) => Some(ItemModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // ASSAULT VEST - 1.5x SpD, can't use status moves
        ("assaultvest", ItemEvent::OnModifyStat) => Some(ItemModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // FOCUS SASH - Survive at 1 HP if at full health
        ("focussash", ItemEvent::OnTakeDamage) => Some(ItemModifier {
            message: Some("Focus Sash prevented the knockout!".to_string()),
            consume: true,
            ..Default::default()
        }),

        // ROCKY HELMET - Deal 1/6 damage to attacker on contact
        ("rockyhelmet", ItemEvent::OnContact) => Some(ItemModifier {
            damage_fraction: Some(1.0 / 6.0),
            message: Some("Rocky Helmet damaged the attacker!".to_string()),
            ..Default::default()
        }),

        // SCOPE LENS - Increase crit ratio
        ("scopelens", ItemEvent::OnModifyCritRatio) => Some(ItemModifier {
            crit_ratio_boost: Some(1),
            ..Default::default()
        }),

        // RAZOR CLAW - Increase crit ratio
        ("razorclaw", ItemEvent::OnModifyCritRatio) => Some(ItemModifier {
            crit_ratio_boost: Some(1),
            ..Default::default()
        }),

        // WIDE LENS - 1.1x accuracy
        ("widelens", ItemEvent::OnModifyAccuracy) => Some(ItemModifier {
            accuracy_multiplier: Some(1.1),
            ..Default::default()
        }),

        // ZOOM LENS - 1.2x accuracy if moving last
        ("zoomlens", ItemEvent::OnModifyAccuracy) => Some(ItemModifier {
            accuracy_multiplier: Some(1.2),
            ..Default::default()
        }),

        // QUICK CLAW - Chance to move first
        ("quickclaw", ItemEvent::BeforeMove) => Some(ItemModifier {
            priority_modifier: Some(1),
            message: Some("Quick Claw let it move first!".to_string()),
            ..Default::default()
        }),

        // TYPE BOOSTING ITEMS
        // Charcoal - 1.2x Fire
        ("charcoal" | "mysticwater" | "miracleseed" | "magnet" | "nevermeltice" |
         "blackbelt" | "poisonbarb" | "softsand" | "sharpbeak" | "twistedspoon" |
         "silverpowder" | "hardstone" | "spellplate" | "dragonfang" | "blackglasses" |
         "metalcoat" | "silkscarf" | "pixieplate", ItemEvent::OnModifyDamage) => Some(ItemModifier {
            damage_multiplier: Some(1.2),
            ..Default::default()
        }),

        // HEAVY-DUTY BOOTS - Immune to entry hazards
        ("heavydutyboots", ItemEvent::OnSwitchIn) => Some(ItemModifier {
            block: true, // Block hazard damage
            ..Default::default()
        }),

        // SAFETY GOGGLES - Immune to powder moves and weather damage
        ("safetygoggles", ItemEvent::OnResidual) => Some(ItemModifier {
            block: true, // Block weather damage
            ..Default::default()
        }),

        // AIR BALLOON - Immune to Ground
        ("airballoon", ItemEvent::OnSwitchIn) => Some(ItemModifier {
            message: Some("floats in the air with its Air Balloon!".to_string()),
            ..Default::default()
        }),

        // Default - no effect
        _ => None,
    }
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
    fn test_leftovers_effect() {
        let item = ID::new("leftovers");
        let effect = get_item_effect(&item, ItemEvent::OnResidual);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.heal_fraction, Some(1.0 / 16.0));
    }

    #[test]
    fn test_sitrus_berry_effect() {
        let item = ID::new("sitrusberry");
        let effect = get_item_effect(&item, ItemEvent::OnHPBelowHalf);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.heal_fraction, Some(0.25));
        assert!(modifier.consume);
    }

    #[test]
    fn test_choice_band_effect() {
        let item = ID::new("choiceband");
        let effect = get_item_effect(&item, ItemEvent::OnModifyStat);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.stat_multiplier, Some(1.5));
    }

    #[test]
    fn test_life_orb_effect() {
        let item = ID::new("lifeorb");
        let effect = get_item_effect(&item, ItemEvent::OnModifyDamage);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.damage_multiplier, Some(1.3));
    }

    #[test]
    fn test_type_boost_charcoal() {
        let item = ID::new("charcoal");
        let boost = get_item_type_boost(&item);
        assert!(boost.is_some());
        let (type_name, multiplier) = boost.unwrap();
        assert_eq!(type_name, "Fire");
        assert_eq!(multiplier, 1.2);
    }

    #[test]
    fn test_lum_berry_status_prevention() {
        let item = ID::new("lumberry");
        assert!(check_item_prevents_status(&item, "brn"));
        assert!(check_item_prevents_status(&item, "par"));
        assert!(check_item_prevents_status(&item, "slp"));
    }

    #[test]
    fn test_rawst_berry_status_prevention() {
        let item = ID::new("rawstberry");
        assert!(check_item_prevents_status(&item, "brn"));
        assert!(!check_item_prevents_status(&item, "par"));
    }
}
