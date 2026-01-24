//! Helper methods to create Effect structs with proper display names.
//!
//! These methods look up the proper display name from dex at creation time,
//! avoiding the need for dex lookups later in single_event.

use crate::dex_data::ID;
use super::{Battle, Effect, EffectType};
use std::sync::Arc;

impl Battle {
    /// Create an Ability Effect with proper display name from dex.
    #[inline]
    pub fn make_ability_effect(&self, id: &ID) -> Effect {
        let name = self.dex.abilities().get(id.as_str())
            .map(|a| Arc::from(a.name.as_str()))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Ability,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create an Item Effect with proper display name from dex.
    #[inline]
    pub fn make_item_effect(&self, id: &ID) -> Effect {
        let name = self.dex.items().get(id.as_str())
            .map(|i| Arc::from(i.name.as_str()))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Item,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a Move Effect with proper display name from dex.
    #[inline]
    pub fn make_move_effect(&self, id: &ID) -> Effect {
        let name = self.dex.moves().get(id.as_str())
            .map(|m| Arc::from(m.name.as_str()))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Move,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a MoveSelf Effect with proper display name from dex.
    #[inline]
    pub fn make_move_self_effect(&self, id: &ID) -> Effect {
        let name = self.dex.moves().get(id.as_str())
            .map(|m| Arc::from(m.name.as_str()))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::MoveSelf,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a Status Effect with proper display name from dex.
    #[inline]
    pub fn make_status_effect(&self, id: &ID) -> Effect {
        let name = self.dex.conditions().get_by_id(id)
            .and_then(|c| c.name.as_ref().map(|n| Arc::from(n.as_str())))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Status,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a Condition (volatile) Effect with proper display name from dex.
    #[inline]
    pub fn make_condition_effect(&self, id: &ID) -> Effect {
        let name = self.dex.conditions().get_by_id(id)
            .and_then(|c| c.name.as_ref().map(|n| Arc::from(n.as_str())))
            .or_else(|| self.dex.moves().get(id.as_str()).map(|m| Arc::from(m.name.as_str())))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Condition,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a Weather Effect with proper display name from dex.
    #[inline]
    pub fn make_weather_effect(&self, id: &ID) -> Effect {
        let name = self.dex.conditions().get_by_id(id)
            .and_then(|c| c.name.as_ref().map(|n| Arc::from(n.as_str())))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Weather,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a Terrain Effect with proper display name from dex.
    #[inline]
    pub fn make_terrain_effect(&self, id: &ID) -> Effect {
        let name = self.dex.conditions().get_by_id(id)
            .and_then(|c| c.name.as_ref().map(|n| Arc::from(n.as_str())))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::Terrain,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Create a SideCondition Effect with proper display name from dex.
    #[inline]
    pub fn make_side_condition_effect(&self, id: &ID) -> Effect {
        let name = self.dex.conditions().get_by_id(id)
            .and_then(|c| c.name.as_ref().map(|n| Arc::from(n.as_str())))
            .unwrap_or_else(|| Arc::from(id.as_str()));
        Effect {
            id: id.clone(),
            name,
            effect_type: EffectType::SideCondition,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }
}
