//! Item Effect System
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines declarative item effects that can be interpreted
//! by the battle engine. Most items use these data-driven effects,
//! while complex items use handler functions.

use serde::{Deserialize, Serialize};

/// Stat identifiers for stat modification effects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
    Accuracy,
    Evasion,
}

impl Stat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "atk" | "attack" => Some(Stat::Atk),
            "def" | "defense" => Some(Stat::Def),
            "spa" | "specialattack" | "spattack" => Some(Stat::SpA),
            "spd" | "specialdefense" | "spdefense" => Some(Stat::SpD),
            "spe" | "speed" => Some(Stat::Spe),
            "accuracy" | "acc" => Some(Stat::Accuracy),
            "evasion" | "eva" => Some(Stat::Evasion),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Stat::Atk => "atk",
            Stat::Def => "def",
            Stat::SpA => "spa",
            Stat::SpD => "spd",
            Stat::Spe => "spe",
            Stat::Accuracy => "accuracy",
            Stat::Evasion => "evasion",
        }
    }
}

/// Trigger conditions for item effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Trigger {
    /// Trigger when taking damage from a specific move type
    OnDamagingHitType(String),
    /// Trigger when HP falls below threshold (as fraction, e.g., 0.5 = 50%)
    OnHPBelow(f64),
    /// Trigger when HP falls below threshold with Gluttony ability
    OnHPBelowGluttony(f64),
    /// Trigger when afflicted with a specific status
    OnStatus(String),
    /// Trigger when any status is applied
    OnAnyStatus,
    /// Trigger when super-effective damage is taken from a type
    OnSuperEffectiveHit(String),
    /// Trigger at end of turn
    OnResidual,
    /// Trigger when Pokemon switches in
    OnSwitchIn,
    /// Trigger when Pokemon uses a move
    OnUseMove,
    /// Trigger when receiving a stat drop
    OnStatDrop,
    /// Trigger when hit by contact move
    OnContactHit,
}

/// Declarative item effects that can be interpreted by the battle engine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemEffect {
    // === Stat Modifiers ===
    /// Multiply a stat by a factor (e.g., Choice Band: Atk * 1.5)
    ModifyStat {
        stat: Stat,
        multiplier: f64,
    },

    /// Boost move base power for a specific type (e.g., Charcoal: Fire * 1.2)
    BoostType {
        move_type: String,
        multiplier: f64,
    },

    /// Boost all damage dealt (e.g., Life Orb: 1.3x)
    BoostDamage {
        multiplier: f64,
    },

    // === HP Effects ===
    /// Heal a fraction of max HP at end of turn (e.g., Leftovers: 1/16)
    ResidualHeal {
        fraction: f64,
    },

    /// Heal or damage based on type (e.g., Black Sludge)
    ResidualHealOrDamage {
        required_type: String,
        heal_fraction: f64,
        damage_fraction: f64,
    },

    /// Heal when HP falls below threshold (e.g., Sitrus Berry at 50%)
    HealOnThreshold {
        threshold: f64,
        heal_fraction: f64,
    },

    /// Deal recoil damage to holder after attacking (e.g., Life Orb: 10%)
    RecoilOnAttack {
        fraction: f64,
    },

    // === Status Effects ===
    /// Cure a specific status (e.g., Rawst Berry cures burn)
    CureStatus {
        status: String,
    },

    /// Cure all non-volatile statuses (e.g., Lum Berry)
    CureAllStatus,

    /// Prevent a specific status from being applied
    PreventStatus {
        status: String,
    },

    /// Prevent confusion from berries (for Pokemon with wrong nature)
    ConfusionOnEat {
        disliked_flavor: String,
    },

    // === Boost Effects ===
    /// Apply stat stage changes (e.g., Absorb Bulb: +1 SpA)
    BoostStats {
        boosts: Vec<(Stat, i8)>,
    },

    /// Prevent negative stat changes (e.g., Clear Amulet)
    PreventStatDrops,

    /// Restore lowered stats (e.g., White Herb)
    RestoreLoweredStats,

    // === Type Resistance ===
    /// Reduce super-effective damage from a type (e.g., Chople Berry: 0.5x Fighting)
    ResistSuperEffective {
        resist_type: String,
        multiplier: f64,
    },

    // === Move Modification ===
    /// Choice item: lock move and boost stat
    ChoiceLock {
        stat: Stat,
        multiplier: f64,
    },

    /// Boost critical hit ratio (e.g., Scope Lens: +1 stage)
    BoostCritRatio {
        stages: i8,
    },

    /// Boost accuracy (e.g., Wide Lens: 1.1x)
    BoostAccuracy {
        multiplier: f64,
    },

    /// Reduce opponent's accuracy (e.g., Bright Powder)
    ReduceOpponentAccuracy {
        multiplier: f64,
    },

    /// Chance to move first (e.g., Quick Claw: 20%)
    PriorityChance {
        chance: f64,
    },

    /// Skip charge turn for two-turn moves (e.g., Power Herb)
    SkipChargeTurn,

    /// Force multi-hit moves to hit maximum times (e.g., Loaded Dice)
    MaxMultiHit,

    // === Defensive Effects ===
    /// Survive a KO at 1 HP when at full health (e.g., Focus Sash)
    FocusSash,

    /// Deal damage to attacker on contact (e.g., Rocky Helmet: 1/6)
    ContactDamage {
        fraction: f64,
    },

    /// Immune to entry hazards (e.g., Heavy-Duty Boots)
    HazardImmunity,

    /// Immune to weather damage (e.g., Safety Goggles)
    WeatherImmunity,

    /// Immune to powder moves (e.g., Safety Goggles)
    PowderImmunity,

    /// Immune to a move type (e.g., Air Balloon: Ground)
    TypeImmunity {
        immune_type: String,
    },

    // === Grounding Effects ===
    /// Makes holder grounded (e.g., Iron Ball)
    GroundHolder,

    /// Makes holder airborne (e.g., Air Balloon)
    Airborne,

    // === Switching Effects ===
    /// Force switch after taking damage (e.g., Eject Button)
    EjectOnHit,

    /// Force opponent to switch after hitting holder (e.g., Red Card)
    ForceOpponentSwitch,

    /// Allow escape from trapping (e.g., Shed Shell)
    EscapeTrapping,

    // === Volatile Status ===
    /// Add a volatile status to holder
    AddVolatile {
        volatile: String,
    },

    /// Remove a volatile status from holder
    RemoveVolatile {
        volatile: String,
    },

    /// Cure attraction, taunt, encore, etc. (e.g., Mental Herb)
    MentalHerb,

    // === Consumption ===
    /// Consume the item after effect
    ConsumeItem,

    /// Item cannot be removed/knocked off
    CannotRemove,

    /// Ignore Klutz ability
    IgnoreKlutz,

    // === Fling Effect ===
    /// Effect when item is flung
    FlingEffect {
        effect: Box<ItemEffect>,
    },

    // === Compound Effects ===
    /// Multiple effects together
    Compound {
        effects: Vec<ItemEffect>,
    },

    /// Conditional effect based on trigger
    Triggered {
        trigger: Trigger,
        effect: Box<ItemEffect>,
    },
}

impl ItemEffect {
    /// Check if this effect consumes the item
    pub fn consumes_item(&self) -> bool {
        match self {
            ItemEffect::ConsumeItem => true,
            ItemEffect::HealOnThreshold { .. } => true,
            ItemEffect::CureStatus { .. } => true,
            ItemEffect::CureAllStatus => true,
            ItemEffect::ResistSuperEffective { .. } => true,
            ItemEffect::FocusSash => true,
            ItemEffect::RestoreLoweredStats => true,
            ItemEffect::SkipChargeTurn => true,
            ItemEffect::EjectOnHit => true,
            ItemEffect::ForceOpponentSwitch => true,
            ItemEffect::MentalHerb => true,
            ItemEffect::Compound { effects } => effects.iter().any(|e| e.consumes_item()),
            ItemEffect::Triggered { effect, .. } => effect.consumes_item(),
            _ => false,
        }
    }

    /// Create a compound effect from multiple effects
    pub fn compound(effects: Vec<ItemEffect>) -> Self {
        ItemEffect::Compound { effects }
    }

    /// Create a triggered effect
    pub fn triggered(trigger: Trigger, effect: ItemEffect) -> Self {
        ItemEffect::Triggered {
            trigger,
            effect: Box::new(effect),
        }
    }

    /// Common patterns as constructors

    /// Sitrus Berry: Heal 25% when HP < 50%
    pub fn sitrus_berry() -> Self {
        ItemEffect::compound(vec![
            ItemEffect::Triggered {
                trigger: Trigger::OnHPBelow(0.5),
                effect: Box::new(ItemEffect::HealOnThreshold {
                    threshold: 0.5,
                    heal_fraction: 0.25,
                }),
            },
            ItemEffect::ConsumeItem,
        ])
    }

    /// Choice Band: +50% Atk, lock move
    pub fn choice_band() -> Self {
        ItemEffect::ChoiceLock {
            stat: Stat::Atk,
            multiplier: 1.5,
        }
    }

    /// Choice Specs: +50% SpA, lock move
    pub fn choice_specs() -> Self {
        ItemEffect::ChoiceLock {
            stat: Stat::SpA,
            multiplier: 1.5,
        }
    }

    /// Choice Scarf: +50% Speed, lock move
    pub fn choice_scarf() -> Self {
        ItemEffect::ChoiceLock {
            stat: Stat::Spe,
            multiplier: 1.5,
        }
    }

    /// Leftovers: Heal 1/16 HP each turn
    pub fn leftovers() -> Self {
        ItemEffect::ResidualHeal {
            fraction: 1.0 / 16.0,
        }
    }

    /// Life Orb: +30% damage, 10% recoil
    pub fn life_orb() -> Self {
        ItemEffect::compound(vec![
            ItemEffect::BoostDamage { multiplier: 1.3 },
            ItemEffect::RecoilOnAttack { fraction: 0.1 },
        ])
    }

    /// Type-boosting item (Charcoal, Mystic Water, etc.)
    pub fn type_boost(move_type: &str) -> Self {
        ItemEffect::BoostType {
            move_type: move_type.to_string(),
            multiplier: 1.2,
        }
    }

    /// Type-resist berry (Chople, Babiri, etc.)
    pub fn type_resist_berry(resist_type: &str) -> Self {
        ItemEffect::compound(vec![
            ItemEffect::Triggered {
                trigger: Trigger::OnSuperEffectiveHit(resist_type.to_string()),
                effect: Box::new(ItemEffect::ResistSuperEffective {
                    resist_type: resist_type.to_string(),
                    multiplier: 0.5,
                }),
            },
            ItemEffect::ConsumeItem,
        ])
    }

    /// Status-cure berry (Cheri, Rawst, etc.)
    pub fn status_cure_berry(status: &str) -> Self {
        ItemEffect::compound(vec![
            ItemEffect::Triggered {
                trigger: Trigger::OnStatus(status.to_string()),
                effect: Box::new(ItemEffect::CureStatus {
                    status: status.to_string(),
                }),
            },
            ItemEffect::ConsumeItem,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_from_str() {
        assert_eq!(Stat::from_str("atk"), Some(Stat::Atk));
        assert_eq!(Stat::from_str("SpA"), Some(Stat::SpA));
        assert_eq!(Stat::from_str("speed"), Some(Stat::Spe));
        assert_eq!(Stat::from_str("invalid"), None);
    }

    #[test]
    fn test_choice_band_effect() {
        let effect = ItemEffect::choice_band();
        match effect {
            ItemEffect::ChoiceLock { stat, multiplier } => {
                assert_eq!(stat, Stat::Atk);
                assert_eq!(multiplier, 1.5);
            }
            _ => panic!("Expected ChoiceLock effect"),
        }
    }

    #[test]
    fn test_life_orb_effect() {
        let effect = ItemEffect::life_orb();
        match effect {
            ItemEffect::Compound { effects } => {
                assert_eq!(effects.len(), 2);
            }
            _ => panic!("Expected Compound effect"),
        }
    }

    #[test]
    fn test_consumes_item() {
        assert!(ItemEffect::ConsumeItem.consumes_item());
        assert!(ItemEffect::sitrus_berry().consumes_item());
        assert!(!ItemEffect::leftovers().consumes_item());
        assert!(!ItemEffect::choice_band().consumes_item());
    }

    #[test]
    fn test_type_boost() {
        let effect = ItemEffect::type_boost("Fire");
        match effect {
            ItemEffect::BoostType { move_type, multiplier } => {
                assert_eq!(move_type, "Fire");
                assert_eq!(multiplier, 1.2);
            }
            _ => panic!("Expected BoostType effect"),
        }
    }

    #[test]
    fn test_type_resist_berry() {
        let effect = ItemEffect::type_resist_berry("Fighting");
        assert!(effect.consumes_item());
    }
}
