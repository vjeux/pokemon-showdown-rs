//! Choice Parsing
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles parsing player choices (move, switch, team order, etc.)

use crate::dex_data::ID;

/// Parsed choice type
#[derive(Debug, Clone, PartialEq)]
/// JavaScript equivalent: Choice (sim/side.ts)
/// 11 fields in JavaScript
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

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Move {
                slot,
                target,
                mega,
                zmove,
                dynamax,
                terastallize,
            } => {
                write!(f, "move {}", slot)?;
                if let Some(t) = target {
                    write!(f, " {}", t)?;
                }
                if *mega {
                    write!(f, " mega")?;
                }
                if *zmove {
                    write!(f, " zmove")?;
                }
                if *dynamax {
                    write!(f, " dynamax")?;
                }
                if *terastallize {
                    write!(f, " terastallize")?;
                }
                Ok(())
            }
            Choice::Switch { slot } => write!(f, "switch {}", slot),
            Choice::Team { order } => {
                let order_str: String = order.iter().map(|n| n.to_string()).collect();
                write!(f, "team {}", order_str)
            }
            Choice::Pass => write!(f, "pass"),
            Choice::Default => write!(f, "default"),
            Choice::Undo => write!(f, "undo"),
            Choice::Shift => write!(f, "shift"),
        }
    }
}

impl std::fmt::Display for ChoiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChoiceError::InvalidFormat(s) => write!(f, "Invalid choice format: {}", s),
            ChoiceError::MissingMoveSlot => write!(f, "Missing move slot"),
            ChoiceError::InvalidMoveSlot(n) => write!(f, "Invalid move slot: {} (must be 1-4)", n),
            ChoiceError::MissingSwitchSlot => write!(f, "Missing switch slot"),
            ChoiceError::InvalidSwitchSlot(n) => {
                write!(f, "Invalid switch slot: {} (must be 1-6)", n)
            }
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

/// Battle request sent to players
#[derive(Debug, Clone)]
/// JavaScript equivalent: BattleRequest (sim/side.ts)
/// 5 fields in JavaScript
pub struct BattleRequest {
    /// Request type
    /// JavaScript: requestType: 'move' | 'switch' | 'teamPreview' | 'wait'
    pub request_type: RequestType,
    /// Active Pokemon info
    /// JavaScript: active: ActiveRequest[]
    pub active: Vec<ActiveRequest>,
    /// Side info (team)
    /// JavaScript: side: SideRequestData
    pub side: SideRequest,
    /// Can undo?
    /// JavaScript: canUndo: boolean
    pub can_undo: bool,
    /// No cancel?
    /// JavaScript: noCancel: boolean
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
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: ActiveRequest (sim/side.ts)
/// 7 fields in JavaScript
pub struct ActiveRequest {
    /// Available moves
    /// JavaScript: moves: MoveRequest[]
    pub moves: Vec<MoveRequest>,
    /// Can mega evolve?
    /// JavaScript: canMega: boolean
    pub can_mega: bool,
    /// Can use Z-move?
    /// JavaScript: canZMove?: (ZMoveRequest | null)[]
    pub can_zmove: Option<Vec<Option<ZMoveRequest>>>,
    /// Can Dynamax?
    /// JavaScript: canDynamax: boolean
    pub can_dynamax: bool,
    /// Can Terastallize?
    /// JavaScript: canTerastallize?: string
    pub can_terastallize: Option<String>,
    /// Trapped?
    /// JavaScript: trapped: boolean
    pub trapped: bool,
    /// Maybe trapped?
    /// JavaScript: maybeTrapped: boolean
    pub maybe_trapped: bool,
}

/// Move request data
#[derive(Debug, Clone)]
/// JavaScript equivalent: MoveRequest (sim/side.ts)
/// 8 fields in JavaScript
pub struct MoveRequest {
    /// Move ID
    /// JavaScript: id: ID
    pub id: ID,
    /// Move name
    /// JavaScript: move: string
    pub name: String,
    /// Current PP
    /// JavaScript: pp: number
    pub pp: u8,
    /// Max PP
    /// JavaScript: maxpp: number
    pub maxpp: u8,
    /// Move target type
    /// JavaScript: target: string
    pub target: String,
    /// Is move disabled?
    /// JavaScript: disabled: boolean
    pub disabled: bool,
}

/// Z-Move request data
#[derive(Debug, Clone)]
/// JavaScript equivalent: ZMoveRequest (sim/side.ts)
/// 2 fields in JavaScript
pub struct ZMoveRequest {
    /// Z-Move name
    /// JavaScript: move: string
    pub name: String,
    /// Z-Move target
    /// JavaScript: target: string
    pub target: String,
}

/// Side request data (team info)
/// JavaScript equivalent: SideRequestData (sim/side.ts)
/// 4 fields in JavaScript
#[derive(Debug, Clone, Default)]
pub struct SideRequest {
    /// Player name
    /// JavaScript: name: string
    pub name: String,
    /// Player ID
    /// JavaScript: id: string
    pub id: String,
    /// Pokemon on team
    /// JavaScript: pokemon: PokemonRequestData[]
    pub pokemon: Vec<PokemonSwitchRequestData>,
    /// No cancel allowed?
    /// JavaScript: noCancel?: boolean
    pub no_cancel: Option<bool>,
}

/// Pokemon switch request data (team preview and force switch)
/// JavaScript equivalent: PokemonSwitchRequestData (sim/side.ts)
/// 14 fields in JavaScript
#[derive(Debug, Clone)]
pub struct PokemonSwitchRequestData {
    /// Pokemon ident (e.g., "p1: Pikachu")
    /// JavaScript: ident: string
    pub ident: String,
    /// Details string
    /// JavaScript: details: string
    pub details: String,
    /// HP and status condition
    /// JavaScript: condition: string
    pub condition: String,
    /// Is this Pokemon active?
    /// JavaScript: active: boolean
    pub active: bool,
    /// Stats (except HP)
    /// JavaScript: stats: StatsExceptHP
    pub stats: RequestStatsExceptHP,
    /// Moves
    /// JavaScript: moves: ID[]
    pub moves: Vec<ID>,
    /// Base ability
    /// JavaScript: baseAbility: ID
    pub base_ability: ID,
    /// Item
    /// JavaScript: item: ID
    pub item: ID,
    /// Pokeball
    /// JavaScript: pokeball: ID
    pub pokeball: ID,
    /// Current ability (if different from base)
    /// JavaScript: ability?: ID
    pub ability: Option<ID>,
    /// Is commanding another Pokemon (Dondozo/Tatsugiri)
    /// JavaScript: commanding?: boolean
    pub commanding: Option<bool>,
    /// Is reviving (Revival Blessing)
    /// JavaScript: reviving?: boolean
    pub reviving: Option<bool>,
    /// Tera type
    /// JavaScript: teraType?: string
    pub tera_type: Option<String>,
    /// Has terastallized
    /// JavaScript: terastallized?: string
    pub terastallized: Option<String>,
}

/// Pokemon move request data (normal turn move selection)
/// JavaScript equivalent: PokemonMoveRequestData (sim/side.ts)
/// 13 fields in JavaScript
#[derive(Debug, Clone, Default)]
pub struct PokemonMoveRequestData {
    /// Available moves
    /// JavaScript: moves: MoveOption[]
    pub moves: Vec<MoveRequestOption>,
    /// Maybe disabled by Imprison, etc.
    /// JavaScript: maybeDisabled?: boolean
    pub maybe_disabled: Option<bool>,
    /// Maybe locked by Choice item, etc.
    /// JavaScript: maybeLocked?: boolean
    pub maybe_locked: Option<bool>,
    /// Trapped by ability/move
    /// JavaScript: trapped?: boolean
    pub trapped: Option<bool>,
    /// Maybe trapped
    /// JavaScript: maybeTrapped?: boolean
    pub maybe_trapped: Option<bool>,
    /// Can mega evolve
    /// JavaScript: canMegaEvo?: boolean
    pub can_mega_evo: Option<bool>,
    /// Can mega evolve X
    /// JavaScript: canMegaEvoX?: boolean
    pub can_mega_evo_x: Option<bool>,
    /// Can mega evolve Y
    /// JavaScript: canMegaEvoY?: boolean
    pub can_mega_evo_y: Option<bool>,
    /// Can ultra burst
    /// JavaScript: canUltraBurst?: boolean
    pub can_ultra_burst: Option<bool>,
    /// Can use Z-move
    /// JavaScript: canZMove?: (ZMoveOption | null)[]
    pub can_z_move: Option<Vec<Option<ZMoveOption>>>,
    /// Can Dynamax
    /// JavaScript: canDynamax?: boolean
    pub can_dynamax: Option<bool>,
    /// Max moves available
    /// JavaScript: maxMoves?: DynamaxOptions
    pub max_moves: Option<DynamaxOptions>,
    /// Can Terastallize (tera type if available)
    /// JavaScript: canTerastallize?: string
    pub can_terastallize: Option<String>,
}

/// Move option in request
#[derive(Debug, Clone)]
/// JavaScript equivalent: MoveOption (sim/side.ts)
/// 5 fields in JavaScript
pub struct MoveRequestOption {
    /// Move name
    /// JavaScript: move: string
    pub move_name: String,
    /// Move ID
    /// JavaScript: id: ID
    pub id: ID,
    /// Move target type
    /// JavaScript: target?: string
    pub target: Option<String>,
    /// Is move disabled?
    /// JavaScript: disabled?: boolean | string
    /// TODO: Rust uses MoveDisabled enum to represent boolean | string union
    pub disabled: Option<MoveDisabled>,
    /// What disabled the move
    /// JavaScript: disabledSource?: string
    pub disabled_source: Option<String>,
}

/// Move disabled state
#[derive(Debug, Clone)]
pub enum MoveDisabled {
    /// Disabled by name (e.g., "locked")
    Named(String),
    /// Disabled boolean
    Bool(bool),
}

/// Dynamax options
/// JavaScript equivalent: DynamaxOptions (sim/side.ts)
/// 2 fields in JavaScript
#[derive(Debug, Clone)]
pub struct DynamaxOptions {
    /// Max moves available
    /// JavaScript: maxMoves: MaxMoveOption[]
    pub max_moves: Vec<MaxMoveOption>,
    /// Gigantamax forme if available
    /// JavaScript: gigantamax?: string
    pub gigantamax: Option<String>,
}

/// Max move option
#[derive(Debug, Clone)]
/// JavaScript equivalent: MaxMoveOption (sim/side.ts)
/// 3 fields in JavaScript
pub struct MaxMoveOption {
    /// Move name
    /// JavaScript: move: string
    pub move_name: String,
    /// Move target type
    /// JavaScript: target: string
    pub target: String,
    /// Is move disabled?
    /// JavaScript: disabled?: boolean
    pub disabled: Option<bool>,
}

/// Z-Move option
#[derive(Debug, Clone)]
/// JavaScript equivalent: ZMoveOption (sim/side.ts)
/// 2 fields in JavaScript
pub struct ZMoveOption {
    /// Z-Move name
    /// JavaScript: move: string
    pub move_name: String,
    /// Z-Move target type
    /// JavaScript: target: string
    pub target: String,
}

/// Stats except HP (for requests)
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: StatsExceptHP (sim/global-types.ts)
/// 5 fields in JavaScript
pub struct RequestStatsExceptHP {
    /// Attack stat
    /// JavaScript: atk: number
    pub atk: i32,
    /// Defense stat
    /// JavaScript: def: number
    pub def: i32,
    /// Special Attack stat
    /// JavaScript: spa: number
    pub spa: i32,
    /// Special Defense stat
    /// JavaScript: spd: number
    pub spd: i32,
    /// Speed stat
    /// JavaScript: spe: number
    pub spe: i32,
}

/// Deprecated - use PokemonSwitchRequestData instead
#[deprecated(note = "Use PokemonSwitchRequestData instead")]
pub type PokemonRequest = PokemonSwitchRequestData;

/// Deprecated - use RequestStatsExceptHP instead
#[deprecated(note = "Use RequestStatsExceptHP instead")]
pub type RequestStats = RequestStatsExceptHP;

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
        assert_eq!(
            choice,
            Choice::Move {
                slot: 1,
                target: None,
                mega: false,
                zmove: false,
                dynamax: false,
                terastallize: false,
            }
        );
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
        assert_eq!(
            choice,
            Choice::Team {
                order: vec![3, 1, 2, 4, 5, 6]
            }
        );
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
        assert_eq!(
            choice,
            Choice::Move {
                slot: 2,
                target: None,
                mega: false,
                zmove: false,
                dynamax: false,
                terastallize: false,
            }
        );
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

        let choice = Choice::Team {
            order: vec![3, 1, 2],
        };
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
