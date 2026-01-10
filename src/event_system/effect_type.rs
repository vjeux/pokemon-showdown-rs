//! Effect Type

/// Effect types - matches JavaScript effectType
/// TODO: Not in JavaScript - Rust-specific enum for effect type constants
/// JavaScript uses string literals for effect types ('Ability', 'Item', 'Move', etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub fn parse(s: &str) -> Option<Self> {
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
