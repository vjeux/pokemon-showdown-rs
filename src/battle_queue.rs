//! Simulator Battle Action Queue
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! The action queue is the core of the battle simulation. A rough overview of
//! the core battle loop:
//!
//! - chosen moves/switches are added to the action queue
//! - the action queue is sorted in speed/priority order
//! - we go through the action queue
//! - repeat

// Type modules
mod move_action;
mod switch_action;
mod team_action;
mod field_action;
mod pokemon_action;
mod action;
mod battle_queue_struct;

// Function modules
mod new;
mod shift;
mod peek;
mod peek_end;
mod push;
mod unshift;
mod len;
mod is_empty;
mod clear;
mod cancel_action;
mod cancel_move;
mod will_move;
mod will_move_mut;
mod will_move_pokemon;
mod will_switch;
mod will_act;
mod sort;
mod prioritize_action;
mod iter;
mod iter_mut;
mod change_action;
mod insert_in_order;
mod insert_choice;
mod add_choice;
mod add_choice_raw;
mod debug;
mod entries;
mod entries_mut;
mod find;
mod splice;
mod resolve_action;

// Re-export types from submodules
pub use move_action::{MoveAction, MoveActionType};
pub use switch_action::{SwitchAction, SwitchActionType};
pub use team_action::{TeamAction, TeamActionType};
pub use field_action::{FieldAction, FieldActionType};
pub use pokemon_action::{PokemonAction, PokemonActionType};
pub use action::Action;
pub use battle_queue_struct::BattleQueue;
