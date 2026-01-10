//! Action enum combining all action types

use serde::{Deserialize, Serialize};

use super::{
    FieldAction, FieldActionType, MoveAction, PokemonAction, PokemonActionType, SwitchAction,
    TeamAction,
};

/// All possible actions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Action (sim/global-types.ts)
pub enum Action {
    Move(MoveAction),
    Switch(SwitchAction),
    Team(TeamAction),
    Field(FieldAction),
    Pokemon(PokemonAction),
}

impl Action {
    /// Get the order value for sorting
    pub fn order(&self) -> i32 {
        match self {
            Action::Move(a) => a.order,
            Action::Switch(a) => a.order,
            Action::Team(_) => 1,
            Action::Field(a) => match a.choice {
                FieldActionType::Start => 2,
                FieldActionType::BeforeTurn => 4,
                FieldActionType::Pass => 200,
                FieldActionType::Residual => 300,
            },
            Action::Pokemon(a) => {
                if a.order != 0 {
                    a.order
                } else {
                    match a.choice {
                        PokemonActionType::Start => 2,
                        PokemonActionType::BeforeTurn => 4,
                        PokemonActionType::RunSwitch => 101,
                        PokemonActionType::MegaEvo
                        | PokemonActionType::MegaEvoX
                        | PokemonActionType::MegaEvoY => 104,
                        PokemonActionType::RunDynamax => 105,
                        PokemonActionType::Terastallize => 106,
                        PokemonActionType::Shift => 200,
                        PokemonActionType::Event => 200,
                        PokemonActionType::Residual => 300,
                    }
                }
            }
        }
    }

    /// Get the priority value for sorting
    pub fn priority(&self) -> i8 {
        match self {
            Action::Move(a) => a.priority,
            Action::Switch(a) => a.priority,
            Action::Team(a) => a.priority,
            Action::Field(a) => a.priority,
            Action::Pokemon(a) => a.priority,
        }
    }

    /// Get the speed value for sorting
    pub fn speed(&self) -> f64 {
        match self {
            Action::Move(a) => a.speed,
            Action::Switch(a) => a.speed,
            Action::Team(a) => a.speed,
            Action::Field(_) => 1.0,
            Action::Pokemon(a) => a.speed,
        }
    }

    /// Get the fractional priority
    pub fn fractional_priority(&self) -> f64 {
        match self {
            Action::Move(a) => a.fractional_priority,
            _ => 0.0,
        }
    }

    /// Get the sub-order for tie-breaking
    pub fn sub_order(&self) -> i32 {
        match self {
            Action::Move(a) => a.sub_order,
            Action::Switch(a) => a.sub_order,
            Action::Team(a) => a.sub_order,
            Action::Field(a) => a.sub_order,
            Action::Pokemon(a) => a.sub_order,
        }
    }

    /// Get the effect order for tie-breaking
    pub fn effect_order(&self) -> i32 {
        match self {
            Action::Move(a) => a.effect_order,
            Action::Switch(a) => a.effect_order,
            Action::Team(a) => a.effect_order,
            Action::Field(a) => a.effect_order,
            Action::Pokemon(a) => a.effect_order,
        }
    }

    /// Get pokemon index if applicable
    pub fn pokemon_index(&self) -> Option<usize> {
        match self {
            Action::Move(a) => Some(a.pokemon_index),
            Action::Switch(a) => Some(a.pokemon_index),
            Action::Team(a) => Some(a.pokemon_index),
            Action::Field(_) => None,
            Action::Pokemon(a) => Some(a.pokemon_index),
        }
    }

    /// Get side index if applicable
    pub fn side_index(&self) -> Option<usize> {
        match self {
            Action::Move(a) => Some(a.side_index),
            Action::Switch(a) => Some(a.side_index),
            Action::Team(a) => Some(a.side_index),
            Action::Field(_) => None,
            Action::Pokemon(a) => Some(a.side_index),
        }
    }

    /// Check if this is a move action
    pub fn is_move(&self) -> bool {
        matches!(self, Action::Move(_))
    }

    /// Check if this is a switch action
    pub fn is_switch(&self) -> bool {
        matches!(self, Action::Switch(_))
    }

    /// Check if this is a runSwitch action
    pub fn is_run_switch(&self) -> bool {
        matches!(self, Action::Pokemon(p) if p.choice == PokemonActionType::RunSwitch)
    }

    /// Get the switch target (side_idx, pokemon_idx) for Pokemon actions
    pub fn get_switch_target(&self) -> Option<(usize, usize)> {
        match self {
            Action::Pokemon(a) => Some((a.side_index, a.pokemon_index)),
            _ => None,
        }
    }

    /// Check if this action has a pokemon (equivalent to JavaScript's `if (action.pokemon)`)
    /// Field actions don't have a pokemon, all other actions do
    pub fn has_pokemon(&self) -> bool {
        !matches!(self, Action::Field(_))
    }
}
