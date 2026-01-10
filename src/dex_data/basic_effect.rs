//! Basic effect structure

use serde::{Deserialize, Serialize};

use super::{EffectType, ID, Nonstandard};

/// Basic effect - base struct for abilities, items, moves, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: BasicEffect (sim/dex-data.ts)
/// 16 fields in JavaScript
pub struct BasicEffect {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: fullname: string
    pub fullname: String,
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// JavaScript: exists: boolean
    pub exists: bool,
    /// JavaScript: num: number
    pub num: i32,
    /// JavaScript: gen: number
    pub gen: u8,
    /// JavaScript: shortDesc: string
    pub short_desc: String,
    /// JavaScript: desc: string
    pub desc: String,
    /// JavaScript: isNonstandard?: Nonstandard
    pub is_nonstandard: Option<Nonstandard>,
    /// JavaScript: duration?: number
    pub duration: Option<i32>,
    /// JavaScript: noCopy: boolean
    pub no_copy: bool,
    /// JavaScript: affectsFainted: boolean
    pub affects_fainted: bool,
    /// JavaScript: status?: ID
    pub status: Option<ID>,
    /// JavaScript: weather?: ID
    pub weather: Option<ID>,
    /// JavaScript: sourceEffect: string
    pub source_effect: String,
}

impl Default for BasicEffect {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            fullname: String::new(),
            effect_type: EffectType::Condition,
            exists: false,
            num: 0,
            gen: 0,
            short_desc: String::new(),
            desc: String::new(),
            is_nonstandard: None,
            duration: None,
            no_copy: false,
            affects_fainted: false,
            status: None,
            weather: None,
            source_effect: String::new(),
        }
    }
}

impl BasicEffect {
    pub fn new(name: &str) -> Self {
        let id = ID::new(name);
        Self {
            id: id.clone(),
            name: name.to_string(),
            fullname: name.to_string(),
            exists: !id.is_empty(),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for BasicEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
