//! Choice Parsing
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles parsing player choices (move, switch, team order, etc.)

use crate::dex_data::ID;

/// Parsed choice type
#[derive(Debug, Clone, PartialEq)]
pub enum Choice {
    /// Use a move: move <slot> [target] [mega/zmove/dynamax/terastallize]
    Move {
        slot: usize,
        target: Option<i8>,
        mega: bool,
        zmove: bool,
        dynamax: bool,
        terastallize: bool,
    },
    /// Switch to a Pokemon: switch <slot>
    Switch { slot: usize },
    /// Team order for team preview: team <order>
    Team { order: Vec<usize> },
    /// Pass (in doubles when one Pokemon fainted)
    Pass,
    /// Default action (auto-selected)
    Default,
    /// Undo previous choice
    Undo,
    /// Shift position (triples only)
    Shift,
}

impl Choice {
    /// Parse a choice string
    pub fn parse(input: &str) -> Result<Self, ChoiceError> {
        let input = input.trim().to_lowercase();

        if input.is_empty() || input == "default" {
            return Ok(Choice::Default);
        }

        if input == "pass" {
            return Ok(Choice::Pass);
        }

        if input == "undo" {
            return Ok(Choice::Undo);
        }

        if input == "shift" {
            return Ok(Choice::Shift);
        }

        // Parse move choice
        if input.starts_with("move ") || input.starts_with("move") && input.len() > 4 {
            return Self::parse_move(input[4..].trim());
        }

        // Parse switch choice
        if input.starts_with("switch ") || input.starts_with("switch") && input.len() > 6 {
            return Self::parse_switch(input[6..].trim());
        }

        // Parse team choice
        if input.starts_with("team ") || input.starts_with("team") && input.len() > 4 {
            return Self::parse_team(input[4..].trim());
        }

        // Try parsing as just a number (move slot)
        if let Ok(slot) = input.parse::<usize>() {
            if (1..=4).contains(&slot) {
                return Ok(Choice::Move {
                    slot,
                    target: None,
                    mega: false,
                    zmove: false,
                    dynamax: false,
                    terastallize: false,
                });
            }
        }

        Err(ChoiceError::InvalidFormat(input))
    }

    fn parse_move(input: &str) -> Result<Self, ChoiceError> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ChoiceError::MissingMoveSlot);
        }

        // Parse move slot (1-4 or move name)
        let slot = if let Ok(n) = parts[0].parse::<usize>() {
            if !(1..=4).contains(&n) {
                return Err(ChoiceError::InvalidMoveSlot(n));
            }
            n
        } else {
            // Move name - would need to look up, default to 1
            1
        };

        let mut target: Option<i8> = None;
        let mut mega = false;
        let mut zmove = false;
        let mut dynamax = false;
        let mut terastallize = false;

        for part in &parts[1..] {
            match *part {
                "mega" => mega = true,
                "zmove" | "z" => zmove = true,
                "dynamax" | "max" => dynamax = true,
                "terastallize" | "tera" => terastallize = true,
                _ => {
                    // Try to parse as target
                    if let Ok(t) = part.parse::<i8>() {
                        target = Some(t);
                    } else if part.starts_with('+') || part.starts_with('-') {
                        if let Ok(t) = part.parse::<i8>() {
                            target = Some(t);
                        }
                    }
                }
            }
        }

        Ok(Choice::Move {
            slot,
            target,
            mega,
            zmove,
            dynamax,
            terastallize,
        })
    }

    fn parse_switch(input: &str) -> Result<Self, ChoiceError> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ChoiceError::MissingSwitchSlot);
        }

        // Parse switch slot (1-6 or Pokemon name)
        let slot = if let Ok(n) = parts[0].parse::<usize>() {
            if !(1..=6).contains(&n) {
                return Err(ChoiceError::InvalidSwitchSlot(n));
            }
            n
        } else {
            // Pokemon name - would need to look up
            return Err(ChoiceError::InvalidSwitchSlot(0));
        };

        Ok(Choice::Switch { slot })
    }

    fn parse_team(input: &str) -> Result<Self, ChoiceError> {
        let input = input.replace(",", "").replace(" ", "");

        if input.is_empty() {
            return Err(ChoiceError::MissingTeamOrder);
        }

        let mut order = Vec::new();
        for c in input.chars() {
            if let Some(digit) = c.to_digit(10) {
                let slot = digit as usize;
                if (1..=6).contains(&slot) {
                    order.push(slot);
                } else {
                    return Err(ChoiceError::InvalidTeamSlot(slot));
                }
            }
        }

        if order.is_empty() {
            return Err(ChoiceError::MissingTeamOrder);
        }

        Ok(Choice::Team { order })
    }

    /// Convert choice back to string format
    pub fn to_string(&self) -> String {
        match self {
            Choice::Move { slot, target, mega, zmove, dynamax, terastallize } => {
                let mut s = format!("move {}", slot);
                if let Some(t) = target {
                    s.push_str(&format!(" {}", t));
                }
                if *mega { s.push_str(" mega"); }
                if *zmove { s.push_str(" zmove"); }
                if *dynamax { s.push_str(" dynamax"); }
                if *terastallize { s.push_str(" terastallize"); }
                s
            }
            Choice::Switch { slot } => format!("switch {}", slot),
            Choice::Team { order } => {
                let order_str: String = order.iter().map(|n| n.to_string()).collect();
                format!("team {}", order_str)
            }
            Choice::Pass => "pass".to_string(),
            Choice::Default => "default".to_string(),
            Choice::Undo => "undo".to_string(),
            Choice::Shift => "shift".to_string(),
        }
    }
}

/// Choice parsing error
#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceError {
    InvalidFormat(String),
    MissingMoveSlot,
    InvalidMoveSlot(usize),
    MissingSwitchSlot,
    InvalidSwitchSlot(usize),
    MissingTeamOrder,
    InvalidTeamSlot(usize),
}

impl std::fmt::Display for ChoiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceError::InvalidFormat(s) => write!(f, "Invalid choice format: {}", s),
            ChoiceError::MissingMoveSlot => write!(f, "Missing move slot"),
            ChoiceError::InvalidMoveSlot(n) => write!(f, "Invalid move slot: {} (must be 1-4)", n),
            ChoiceError::MissingSwitchSlot => write!(f, "Missing switch slot"),
            ChoiceError::InvalidSwitchSlot(n) => write!(f, "Invalid switch slot: {} (must be 1-6)", n),
            ChoiceError::MissingTeamOrder => write!(f, "Missing team order"),
            ChoiceError::InvalidTeamSlot(n) => write!(f, "Invalid team slot: {} (must be 1-6)", n),
        }
    }
}

/// Parse multiple choices (for doubles/triples)
pub fn parse_choices(input: &str) -> Result<Vec<Choice>, ChoiceError> {
    let mut choices = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if !part.is_empty() {
            choices.push(Choice::parse(part)?);
        }
    }

    if choices.is_empty() {
        choices.push(Choice::Default);
    }

    Ok(choices)
}

/// Request data sent to players
#[derive(Debug, Clone)]
pub struct BattleRequest {
    /// Request type
    pub request_type: RequestType,
    /// Active Pokemon info
    pub active: Vec<ActiveRequest>,
    /// Side info (team)
    pub side: SideRequest,
    /// Can undo?
    pub can_undo: bool,
    /// No cancel?
    pub no_cancel: bool,
}

/// Type of request
#[derive(Debug, Clone, PartialEq)]
pub enum RequestType {
    /// Normal turn - choose moves/switches
    Move,
    /// Force switch (fainted Pokemon)
    Switch,
    /// Team preview
    TeamPreview,
    /// Waiting for opponent
    Wait,
}

/// Active Pokemon request data
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ActiveRequest {
    /// Available moves
    pub moves: Vec<MoveRequest>,
    /// Can mega evolve?
    pub can_mega: bool,
    /// Can use Z-move?
    pub can_zmove: Option<Vec<Option<ZMoveRequest>>>,
    /// Can Dynamax?
    pub can_dynamax: bool,
    /// Can Terastallize?
    pub can_terastallize: Option<String>,
    /// Trapped?
    pub trapped: bool,
    /// Maybe trapped?
    pub maybe_trapped: bool,
}

/// Move request data
#[derive(Debug, Clone)]
pub struct MoveRequest {
    /// Move ID
    pub id: ID,
    /// Move name
    pub name: String,
    /// Current PP
    pub pp: u8,
    /// Max PP
    pub maxpp: u8,
    /// Move target type
    pub target: String,
    /// Is move disabled?
    pub disabled: bool,
}

/// Z-Move request data
#[derive(Debug, Clone)]
pub struct ZMoveRequest {
    /// Z-Move name
    pub name: String,
    /// Z-Move target
    pub target: String,
}

/// Side request data (team info)
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SideRequest {
    /// Player name
    pub name: String,
    /// Player ID
    pub id: String,
    /// Pokemon on team
    pub pokemon: Vec<PokemonRequest>,
}

/// Pokemon request data
#[derive(Debug, Clone)]
pub struct PokemonRequest {
    /// Pokemon ident (e.g., "p1: Pikachu")
    pub ident: String,
    /// Details string
    pub details: String,
    /// HP and status condition
    pub condition: String,
    /// Is this Pokemon active?
    pub active: bool,
    /// Stats
    pub stats: RequestStats,
    /// Moves
    pub moves: Vec<String>,
    /// Base ability
    pub base_ability: String,
    /// Current ability (if different)
    pub ability: Option<String>,
    /// Item
    pub item: String,
    /// Pokeball
    pub pokeball: String,
}

/// Stats in request
#[derive(Debug, Clone, Default)]
pub struct RequestStats {
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}



impl Default for BattleRequest {
    fn default() -> Self {
        Self {
            request_type: RequestType::Wait,
            active: Vec::new(),
            side: SideRequest::default(),
            can_undo: false,
            no_cancel: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        let choice = Choice::parse("move 1").unwrap();
        assert_eq!(choice, Choice::Move {
            slot: 1,
            target: None,
            mega: false,
            zmove: false,
            dynamax: false,
            terastallize: false,
        });
    }

    #[test]
    fn test_parse_move_with_target() {
        let choice = Choice::parse("move 2 -1").unwrap();
        if let Choice::Move { slot, target, .. } = choice {
            assert_eq!(slot, 2);
            assert_eq!(target, Some(-1));
        } else {
            panic!("Expected Move choice");
        }
    }

    #[test]
    fn test_parse_move_mega() {
        let choice = Choice::parse("move 1 mega").unwrap();
        if let Choice::Move { mega, .. } = choice {
            assert!(mega);
        } else {
            panic!("Expected Move choice");
        }
    }

    #[test]
    fn test_parse_move_dynamax() {
        let choice = Choice::parse("move 3 dynamax").unwrap();
        if let Choice::Move { slot, dynamax, .. } = choice {
            assert_eq!(slot, 3);
            assert!(dynamax);
        } else {
            panic!("Expected Move choice");
        }
    }

    #[test]
    fn test_parse_move_terastallize() {
        let choice = Choice::parse("move 1 tera").unwrap();
        if let Choice::Move { terastallize, .. } = choice {
            assert!(terastallize);
        } else {
            panic!("Expected Move choice");
        }
    }

    #[test]
    fn test_parse_switch() {
        let choice = Choice::parse("switch 3").unwrap();
        assert_eq!(choice, Choice::Switch { slot: 3 });
    }

    #[test]
    fn test_parse_team() {
        let choice = Choice::parse("team 312456").unwrap();
        assert_eq!(choice, Choice::Team { order: vec![3, 1, 2, 4, 5, 6] });
    }

    #[test]
    fn test_parse_pass() {
        let choice = Choice::parse("pass").unwrap();
        assert_eq!(choice, Choice::Pass);
    }

    #[test]
    fn test_parse_default() {
        let choice = Choice::parse("").unwrap();
        assert_eq!(choice, Choice::Default);

        let choice = Choice::parse("default").unwrap();
        assert_eq!(choice, Choice::Default);
    }

    #[test]
    fn test_parse_number_only() {
        let choice = Choice::parse("2").unwrap();
        assert_eq!(choice, Choice::Move {
            slot: 2,
            target: None,
            mega: false,
            zmove: false,
            dynamax: false,
            terastallize: false,
        });
    }

    #[test]
    fn test_parse_multiple_choices() {
        let choices = parse_choices("move 1, switch 3").unwrap();
        assert_eq!(choices.len(), 2);
        assert!(matches!(choices[0], Choice::Move { slot: 1, .. }));
        assert_eq!(choices[1], Choice::Switch { slot: 3 });
    }

    #[test]
    fn test_choice_to_string() {
        let choice = Choice::Move {
            slot: 1,
            target: Some(-2),
            mega: true,
            zmove: false,
            dynamax: false,
            terastallize: false,
        };
        assert_eq!(choice.to_string(), "move 1 -2 mega");

        let choice = Choice::Switch { slot: 4 };
        assert_eq!(choice.to_string(), "switch 4");

        let choice = Choice::Team { order: vec![3, 1, 2] };
        assert_eq!(choice.to_string(), "team 312");
    }

    #[test]
    fn test_invalid_move_slot() {
        let result = Choice::parse("move 5");
        assert!(matches!(result, Err(ChoiceError::InvalidMoveSlot(5))));
    }

    #[test]
    fn test_invalid_switch_slot() {
        let result = Choice::parse("switch 7");
        assert!(matches!(result, Err(ChoiceError::InvalidSwitchSlot(7))));
    }
}
