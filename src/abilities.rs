//! Ability Effect System
//!
//! This module handles ability effects in battle.
//! Abilities can modify various game mechanics through event handlers.

use crate::dex_data::ID;

/// Ability effect types for different game events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityEvent {
    /// When Pokemon switches in
    OnSwitchIn,
    /// When Pokemon switches out
    OnSwitchOut,
    /// At start of battle
    OnStart,
    /// At end of turn (residual)
    OnResidual,
    /// Before a move is used
    BeforeMove,
    /// After a move hits
    AfterMoveHit,
    /// When calculating damage
    OnModifyDamage,
    /// When calculating STAB bonus
    OnModifySTAB,
    /// When calculating base power
    OnModifyBasePower,
    /// When calculating stats
    OnModifyStat,
    /// When taking damage
    OnTakeDamage,
    /// When receiving a status
    OnSetStatus,
    /// When trying to trap foe
    OnTrapFoe,
    /// Immunity check
    OnImmunity,
    /// Weather changes
    OnWeatherChange,
    /// Terrain changes
    OnTerrainChange,
    /// When a Pokemon faints
    OnFaint,
    /// After being hit by a move
    OnDamagingHit,
}

/// Modifiers returned by ability effects
#[derive(Debug, Clone, Default)]
pub struct AbilityModifier {
    /// Multiply damage by this factor (1.0 = no change)
    pub damage_multiplier: Option<f64>,
    /// Multiply stat by this factor
    pub stat_multiplier: Option<f64>,
    /// Block the action entirely
    pub block: bool,
    /// Immune to the effect
    pub immune: bool,
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
}

/// Ability effect handler result
pub type AbilityResult = Option<AbilityModifier>;

/// Get ability effect for a specific ability and event
pub fn get_ability_effect(ability_id: &ID, event: AbilityEvent) -> AbilityResult {
    let ability_name = ability_id.as_str();

    match (ability_name, event) {
        // INTIMIDATE - Lower foe's Attack on switch-in
        ("intimidate", AbilityEvent::OnSwitchIn) => Some(AbilityModifier {
            boost: Some(("atk".to_string(), -1)),
            message: Some("Intimidate lowered the foe's Attack!".to_string()),
            ..Default::default()
        }),

        // DRIZZLE - Summon Rain on switch-in
        ("drizzle", AbilityEvent::OnSwitchIn) => Some(AbilityModifier {
            message: Some("Drizzle summoned rain!".to_string()),
            ..Default::default()
        }),

        // DROUGHT - Summon Sun on switch-in
        ("drought", AbilityEvent::OnSwitchIn) => Some(AbilityModifier {
            message: Some("Drought intensified the sun's rays!".to_string()),
            ..Default::default()
        }),

        // SNOW WARNING - Summon Snow on switch-in
        ("snowwarning", AbilityEvent::OnSwitchIn) => Some(AbilityModifier {
            message: Some("Snow Warning summoned a hailstorm!".to_string()),
            ..Default::default()
        }),

        // SAND STREAM - Summon Sand on switch-in
        ("sandstream", AbilityEvent::OnSwitchIn) => Some(AbilityModifier {
            message: Some("Sand Stream whipped up a sandstorm!".to_string()),
            ..Default::default()
        }),

        // ADAPTABILITY - Boost STAB to 2x
        ("adaptability", AbilityEvent::OnModifySTAB) => Some(AbilityModifier {
            damage_multiplier: Some(2.0 / 1.5), // STAB goes from 1.5x to 2x
            ..Default::default()
        }),

        // TECHNICIAN - Boost moves with 60 or less base power by 1.5x
        ("technician", AbilityEvent::OnModifyBasePower) => Some(AbilityModifier {
            damage_multiplier: Some(1.5),
            ..Default::default()
        }),

        // GUTS - 1.5x Attack when statused
        ("guts", AbilityEvent::OnModifyStat) => Some(AbilityModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // MARVEL SCALE - 1.5x Defense when statused
        ("marvelscale", AbilityEvent::OnModifyStat) => Some(AbilityModifier {
            stat_multiplier: Some(1.5),
            ..Default::default()
        }),

        // WATER ABSORB - Immune to Water, heal 25% HP
        ("waterabsorb", AbilityEvent::OnImmunity) => Some(AbilityModifier {
            immune: true,
            heal_fraction: Some(0.25),
            message: Some("Water Absorb restored HP!".to_string()),
            ..Default::default()
        }),

        // VOLT ABSORB - Immune to Electric, heal 25% HP
        ("voltabsorb", AbilityEvent::OnImmunity) => Some(AbilityModifier {
            immune: true,
            heal_fraction: Some(0.25),
            message: Some("Volt Absorb restored HP!".to_string()),
            ..Default::default()
        }),

        // FLASH FIRE - Immune to Fire, boost Fire moves
        ("flashfire", AbilityEvent::OnImmunity) => Some(AbilityModifier {
            immune: true,
            message: Some("Flash Fire raised the power of Fire-type moves!".to_string()),
            ..Default::default()
        }),

        // LEVITATE - Immune to Ground
        ("levitate", AbilityEvent::OnImmunity) => Some(AbilityModifier {
            immune: true,
            ..Default::default()
        }),

        // STURDY - Survive at 1 HP if at full health
        ("sturdy", AbilityEvent::OnTakeDamage) => Some(AbilityModifier {
            message: Some("Sturdy prevented the knockout!".to_string()),
            ..Default::default()
        }),

        // MAGIC GUARD - Immune to indirect damage
        ("magicguard", AbilityEvent::OnResidual) => Some(AbilityModifier {
            block: true,
            ..Default::default()
        }),

        // POISON HEAL - Heal from poison instead of taking damage
        ("poisonheal", AbilityEvent::OnResidual) => Some(AbilityModifier {
            heal_fraction: Some(0.125), // 1/8 HP
            ..Default::default()
        }),

        // SPEED BOOST - Boost Speed at end of turn
        ("speedboost", AbilityEvent::OnResidual) => Some(AbilityModifier {
            boost: Some(("spe".to_string(), 1)),
            message: Some("Speed Boost raised Speed!".to_string()),
            ..Default::default()
        }),

        // MOODY - Random stat boost at end of turn
        ("moody", AbilityEvent::OnResidual) => Some(AbilityModifier {
            message: Some("Moody changed stats!".to_string()),
            ..Default::default()
        }),

        // STATIC - 30% chance to paralyze on contact
        ("static", AbilityEvent::OnDamagingHit) => Some(AbilityModifier {
            apply_status: Some("par".to_string()),
            message: Some("Static paralyzed the attacker!".to_string()),
            ..Default::default()
        }),

        // FLAME BODY - 30% chance to burn on contact
        ("flamebody", AbilityEvent::OnDamagingHit) => Some(AbilityModifier {
            apply_status: Some("brn".to_string()),
            message: Some("Flame Body burned the attacker!".to_string()),
            ..Default::default()
        }),

        // POISON POINT - 30% chance to poison on contact
        ("poisonpoint", AbilityEvent::OnDamagingHit) => Some(AbilityModifier {
            apply_status: Some("psn".to_string()),
            message: Some("Poison Point poisoned the attacker!".to_string()),
            ..Default::default()
        }),

        // ROUGH SKIN - Deal 1/8 damage to attacker on contact
        ("roughskin", AbilityEvent::OnDamagingHit) => Some(AbilityModifier {
            damage_fraction: Some(0.125),
            message: Some("Rough Skin damaged the attacker!".to_string()),
            ..Default::default()
        }),

        // IRON BARBS - Deal 1/8 damage to attacker on contact
        ("ironbarbs", AbilityEvent::OnDamagingHit) => Some(AbilityModifier {
            damage_fraction: Some(0.125),
            message: Some("Iron Barbs damaged the attacker!".to_string()),
            ..Default::default()
        }),

        // IMMUNITY - Immune to Poison
        ("immunity", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // LIMBER - Immune to Paralysis
        ("limber", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // INSOMNIA - Immune to Sleep
        ("insomnia", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // VITAL SPIRIT - Immune to Sleep
        ("vitalspirit", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // WATER VEIL - Immune to Burn
        ("waterveil", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // MAGMA ARMOR - Immune to Freeze
        ("magmaarmor", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // OWN TEMPO - Immune to Confusion
        ("owntempo", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // OBLIVIOUS - Immune to infatuation
        ("oblivious", AbilityEvent::OnSetStatus) => Some(AbilityModifier {
            prevent_status: true,
            ..Default::default()
        }),

        // THICK FAT - Halve Fire and Ice damage
        ("thickfat", AbilityEvent::OnModifyDamage) => Some(AbilityModifier {
            damage_multiplier: Some(0.5),
            ..Default::default()
        }),

        // FILTER/SOLID ROCK - Reduce super effective damage
        ("filter" | "solidrock", AbilityEvent::OnModifyDamage) => Some(AbilityModifier {
            damage_multiplier: Some(0.75),
            ..Default::default()
        }),

        // PRISM ARMOR - Reduce super effective damage
        ("prismarmor", AbilityEvent::OnModifyDamage) => Some(AbilityModifier {
            damage_multiplier: Some(0.75),
            ..Default::default()
        }),

        // TINTED LENS - Double damage for resisted hits
        ("tintedlens", AbilityEvent::OnModifyDamage) => Some(AbilityModifier {
            damage_multiplier: Some(2.0),
            ..Default::default()
        }),

        // SNIPER - Crits deal 2.25x instead of 1.5x
        ("sniper", AbilityEvent::OnModifyDamage) => Some(AbilityModifier {
            damage_multiplier: Some(1.5), // 2.25 / 1.5 = 1.5
            ..Default::default()
        }),

        // MOXIE - Boost Attack after KO
        ("moxie", AbilityEvent::OnFaint) => Some(AbilityModifier {
            boost: Some(("atk".to_string(), 1)),
            message: Some("Moxie raised Attack!".to_string()),
            ..Default::default()
        }),

        // BEAST BOOST - Boost highest stat after KO
        ("beastboost", AbilityEvent::OnFaint) => Some(AbilityModifier {
            boost: Some(("atk".to_string(), 1)), // Simplified - should be highest stat
            message: Some("Beast Boost raised a stat!".to_string()),
            ..Default::default()
        }),

        // CHILLING NEIGH - Boost Attack after KO
        ("chillingneigh", AbilityEvent::OnFaint) => Some(AbilityModifier {
            boost: Some(("atk".to_string(), 1)),
            message: Some("Chilling Neigh raised Attack!".to_string()),
            ..Default::default()
        }),

        // GRIM NEIGH - Boost Sp. Atk after KO
        ("grimneigh", AbilityEvent::OnFaint) => Some(AbilityModifier {
            boost: Some(("spa".to_string(), 1)),
            message: Some("Grim Neigh raised Special Attack!".to_string()),
            ..Default::default()
        }),

        // SOUL-HEART - Boost Sp. Atk when any Pokemon faints
        ("soulheart", AbilityEvent::OnFaint) => Some(AbilityModifier {
            boost: Some(("spa".to_string(), 1)),
            message: Some("Soul-Heart raised Special Attack!".to_string()),
            ..Default::default()
        }),

        // Default - no effect
        _ => None,
    }
}

/// Check if an ability provides immunity to a specific type
pub fn check_ability_immunity(ability_id: &ID, move_type: &str) -> bool {
    let ability = ability_id.as_str();
    match (ability, move_type.to_lowercase().as_str()) {
        ("levitate", "ground") => true,
        ("flashfire", "fire") => true,
        ("waterabsorb" | "dryskin" | "stormdrain", "water") => true,
        ("voltabsorb" | "lightningrod" | "motordrive", "electric") => true,
        ("sapsipper", "grass") => true,
        ("eartheater", "ground") => true,
        ("wellbakedbody", "fire") => true,
        _ => false,
    }
}

/// Check if an ability prevents a specific status
pub fn check_ability_prevents_status(ability_id: &ID, status: &str) -> bool {
    let ability = ability_id.as_str();
    match (ability, status) {
        ("immunity" | "pastelveil", "psn" | "tox") => true,
        ("limber", "par") => true,
        ("insomnia" | "vitalspirit" | "sweetveil", "slp") => true,
        ("waterveil" | "waterbubble", "brn") => true,
        ("magmaarmor", "frz") => true,
        ("owntempo", "confusion") => true,
        ("oblivious", "attract" | "taunt") => true,
        ("comatose", _) => true, // Already "asleep"
        ("flowerveil", _) => true, // For Grass types
        ("leafguard", _) => true, // In sun
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intimidate_effect() {
        let ability = ID::new("intimidate");
        let effect = get_ability_effect(&ability, AbilityEvent::OnSwitchIn);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.boost, Some(("atk".to_string(), -1)));
    }

    #[test]
    fn test_levitate_immunity() {
        let ability = ID::new("levitate");
        assert!(check_ability_immunity(&ability, "Ground"));
        assert!(!check_ability_immunity(&ability, "Fire"));
    }

    #[test]
    fn test_immunity_status_prevention() {
        let ability = ID::new("immunity");
        assert!(check_ability_prevents_status(&ability, "psn"));
        assert!(check_ability_prevents_status(&ability, "tox"));
        assert!(!check_ability_prevents_status(&ability, "brn"));
    }

    #[test]
    fn test_limber_status_prevention() {
        let ability = ID::new("limber");
        assert!(check_ability_prevents_status(&ability, "par"));
        assert!(!check_ability_prevents_status(&ability, "brn"));
    }

    #[test]
    fn test_water_absorb_immunity() {
        let ability = ID::new("waterabsorb");
        assert!(check_ability_immunity(&ability, "Water"));
        assert!(!check_ability_immunity(&ability, "Electric"));
    }

    #[test]
    fn test_speed_boost_residual() {
        let ability = ID::new("speedboost");
        let effect = get_ability_effect(&ability, AbilityEvent::OnResidual);
        assert!(effect.is_some());
        let modifier = effect.unwrap();
        assert_eq!(modifier.boost, Some(("spe".to_string(), 1)));
    }
}
