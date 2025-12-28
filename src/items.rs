//! Item Effect System
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles item effects in battle.
//! Item callbacks are now in data/item_callbacks.rs for 1:1 JS porting.

use crate::dex_data::ID;

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

/// Legacy function for backward compatibility
/// Get item effect for a specific item and event using the old match-based system
pub fn get_item_effect(_item_id: &ID, _event: ItemEvent) -> ItemResult {
    // Item effects are now handled by item_callbacks.rs
    None
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
    fn test_get_item_type_boost() {
        assert_eq!(
            get_item_type_boost(&ID::new("charcoal")),
            Some(("Fire", 1.2))
        );
        assert_eq!(
            get_item_type_boost(&ID::new("mysticwater")),
            Some(("Water", 1.2))
        );
        assert_eq!(get_item_type_boost(&ID::new("leftovers")), None);
    }

    #[test]
    fn test_check_item_prevents_status() {
        assert!(check_item_prevents_status(&ID::new("lumberry"), "par"));
        assert!(check_item_prevents_status(&ID::new("lumberry"), "brn"));
        assert!(check_item_prevents_status(&ID::new("rawstberry"), "brn"));
        assert!(!check_item_prevents_status(&ID::new("rawstberry"), "par"));
    }
}
