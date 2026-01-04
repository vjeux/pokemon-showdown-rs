//! Battle Stream
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles the battle protocol stream, parsing and generating
//! protocol messages for communication between the battle engine and clients.

use crate::battle::{Battle};
use std::collections::VecDeque;

// Function modules

// Function modules
mod split_first;
pub use split_first::split_first;

mod new;
mod with_options;
mod with_battle;
mod start;
mod write;
mod write_line;
mod push_message;
mod read;
mod battle;
mod battle_mut;
mod ended;
mod winner;
mod destroy;
mod receive;
mod receive_line;
mod receive_error;
mod receive_request;

/// Protocol message types
/// JavaScript equivalent: Protocol messages parsed from strings (sim/battle-stream.ts)
#[derive(Debug, Clone, PartialEq)]
pub enum BattleMessage {
    // Battle initialization
    Player {
        slot: String,
        name: String,
        avatar: Option<String>,
        rating: Option<i32>,
    },
    TeamSize {
        slot: String,
        size: usize,
    },
    GameType {
        game_type: String,
    },
    Gen {
        gen: u8,
    },
    Tier {
        tier: String,
    },
    Rule {
        rule: String,
    },
    ClearPoke,
    Poke {
        slot: String,
        details: String,
        item: Option<String>,
    },
    Start,
    TeamPreview,

    // Turn management
    Turn {
        number: i32,
    },
    Upkeep,
    Request {
        request_json: String,
    },
    Win {
        winner: String,
    },
    Tie,

    // Pokemon actions
    Switch {
        pokemon: String,
        details: String,
        hp_status: String,
    },
    Drag {
        pokemon: String,
        details: String,
        hp_status: String,
    },
    DetailsChange {
        pokemon: String,
        details: String,
    },
    FormeChange {
        pokemon: String,
        species: String,
        hp_status: Option<String>,
    },
    Replace {
        pokemon: String,
        details: String,
    },
    Swap {
        pokemon: String,
        position: u8,
    },

    // Move actions
    Move {
        pokemon: String,
        move_name: String,
        target: Option<String>,
    },
    CantMove {
        pokemon: String,
        reason: String,
        move_name: Option<String>,
    },
    Faint {
        pokemon: String,
    },

    // Damage/healing
    Damage {
        pokemon: String,
        hp_status: String,
        from: Option<String>,
        of: Option<String>,
    },
    Heal {
        pokemon: String,
        hp_status: String,
        from: Option<String>,
        of: Option<String>,
    },
    SetHP {
        pokemon: String,
        hp: String,
    },
    Sethp {
        pokemon: String,
        hp: String,
    }, // Alias

    // Status
    Status {
        pokemon: String,
        status: String,
    },
    CureStatus {
        pokemon: String,
        status: String,
    },
    CureTeam {
        pokemon: String,
    },

    // Boosts
    Boost {
        pokemon: String,
        stat: String,
        amount: i8,
    },
    Unboost {
        pokemon: String,
        stat: String,
        amount: i8,
    },
    SetBoost {
        pokemon: String,
        stat: String,
        amount: i8,
    },
    ClearBoost {
        pokemon: String,
    },
    ClearAllBoost,
    ClearPositiveBoost {
        target: String,
        source: String,
        effect: String,
    },
    ClearNegativeBoost {
        pokemon: String,
    },
    CopyBoost {
        source: String,
        target: String,
    },
    InvertBoost {
        pokemon: String,
    },

    // Weather/terrain/field
    Weather {
        weather: String,
        from: Option<String>,
        of: Option<String>,
    },
    FieldStart {
        condition: String,
        from: Option<String>,
        of: Option<String>,
    },
    FieldEnd {
        condition: String,
    },
    SideStart {
        side: String,
        condition: String,
    },
    SideEnd {
        side: String,
        condition: String,
    },
    SwapSideConditions,

    // Volatile status
    VolatileStart {
        pokemon: String,
        effect: String,
        from: Option<String>,
        of: Option<String>,
    },
    VolatileEnd {
        pokemon: String,
        effect: String,
    },
    Block {
        pokemon: String,
        effect: String,
        move_name: Option<String>,
        attacker: Option<String>,
    },

    // Items/abilities
    Item {
        pokemon: String,
        item: String,
        from: Option<String>,
        of: Option<String>,
    },
    EndItem {
        pokemon: String,
        item: String,
        from: Option<String>,
        eat: bool,
    },
    Ability {
        pokemon: String,
        ability: String,
        from: Option<String>,
        of: Option<String>,
    },
    EndAbility {
        pokemon: String,
        ability: String,
    },

    // Transform/mega/etc
    Transform {
        pokemon: String,
        target: String,
    },
    Mega {
        pokemon: String,
        mega_stone: String,
    },
    Primal {
        pokemon: String,
    },
    Burst {
        pokemon: String,
        species: String,
        item: String,
    },
    ZMove {
        pokemon: String,
    },
    ZBroken {
        pokemon: String,
    },
    Terastallize {
        pokemon: String,
        tera_type: String,
    },

    // Battle mechanics
    Activate {
        effect: String,
        pokemon: Option<String>,
        target: Option<String>,
    },
    Hint {
        message: String,
    },
    Center,
    Message {
        message: String,
    },

    // Effectiveness
    SuperEffective {
        pokemon: String,
    },
    Resisted {
        pokemon: String,
    },
    Immune {
        pokemon: String,
    },
    Crit {
        pokemon: String,
    },
    Miss {
        source: String,
        target: String,
    },

    // Misc
    Fail {
        pokemon: String,
        effect: Option<String>,
    },
    NoTarget {
        pokemon: String,
    },
    PrepareMove {
        pokemon: String,
        move_name: String,
        target: Option<String>,
    },
    MustRecharge {
        pokemon: String,
    },
    Nothing,
    HitCount {
        pokemon: String,
        count: u8,
    },
    SingleMove {
        pokemon: String,
        move_name: String,
    },
    SingleTurn {
        pokemon: String,
        move_name: String,
    },

    // Raw/unknown
    Raw {
        line: String,
    },
}

impl BattleMessage {
    /// Parse a protocol line into a message
    /// Note: Rust-specific infrastructure. JavaScript handles protocol messages as raw strings.
    /// This method provides type-safe parsing for the Rust implementation.
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        // Lines starting with | are protocol messages
        if !line.starts_with('|') {
            return Some(BattleMessage::Raw {
                line: line.to_string(),
            });
        }

        let parts: Vec<&str> = line[1..].splitn(2, '|').collect();
        let cmd = parts.first().copied().unwrap_or("");
        let args = parts.get(1).copied().unwrap_or("");

        match cmd {
            "player" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::Player {
                    slot: parts.first().copied().unwrap_or("").to_string(),
                    name: parts.get(1).copied().unwrap_or("").to_string(),
                    avatar: parts.get(2).map(|s| s.to_string()),
                    rating: parts.get(3).and_then(|s| s.parse().ok()),
                })
            }
            "teamsize" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::TeamSize {
                    slot: parts.first().copied().unwrap_or("").to_string(),
                    size: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                })
            }
            "gametype" => Some(BattleMessage::GameType {
                game_type: args.to_string(),
            }),
            "gen" => Some(BattleMessage::Gen {
                gen: args.parse().unwrap_or(9),
            }),
            "tier" => Some(BattleMessage::Tier {
                tier: args.to_string(),
            }),
            "rule" => Some(BattleMessage::Rule {
                rule: args.to_string(),
            }),
            "clearpoke" => Some(BattleMessage::ClearPoke),
            "poke" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Poke {
                    slot: parts.first().copied().unwrap_or("").to_string(),
                    details: parts.get(1).copied().unwrap_or("").to_string(),
                    item: parts.get(2).map(|s| s.to_string()),
                })
            }
            "start" => Some(BattleMessage::Start),
            "teampreview" => Some(BattleMessage::TeamPreview),
            "turn" => Some(BattleMessage::Turn {
                number: args.parse().unwrap_or(0),
            }),
            "upkeep" => Some(BattleMessage::Upkeep),
            "request" => Some(BattleMessage::Request {
                request_json: args.to_string(),
            }),
            "win" => Some(BattleMessage::Win {
                winner: args.to_string(),
            }),
            "tie" => Some(BattleMessage::Tie),
            "switch" | "drag" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                let msg = if cmd == "switch" {
                    BattleMessage::Switch {
                        pokemon: parts.first().copied().unwrap_or("").to_string(),
                        details: parts.get(1).copied().unwrap_or("").to_string(),
                        hp_status: parts.get(2).copied().unwrap_or("").to_string(),
                    }
                } else {
                    BattleMessage::Drag {
                        pokemon: parts.first().copied().unwrap_or("").to_string(),
                        details: parts.get(1).copied().unwrap_or("").to_string(),
                        hp_status: parts.get(2).copied().unwrap_or("").to_string(),
                    }
                };
                Some(msg)
            }
            "move" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Move {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    move_name: parts.get(1).copied().unwrap_or("").to_string(),
                    target: parts.get(2).map(|s| s.to_string()),
                })
            }
            "faint" => Some(BattleMessage::Faint {
                pokemon: args.to_string(),
            }),
            "-damage" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::Damage {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    hp_status: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    of: parts.get(3).map(|s| s.to_string()),
                })
            }
            "-heal" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::Heal {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    hp_status: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    of: parts.get(3).map(|s| s.to_string()),
                })
            }
            "-status" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Status {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    status: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-curestatus" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::CureStatus {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    status: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-boost" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Boost {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    stat: parts.get(1).copied().unwrap_or("").to_string(),
                    amount: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1),
                })
            }
            "-unboost" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Unboost {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    stat: parts.get(1).copied().unwrap_or("").to_string(),
                    amount: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1),
                })
            }
            "-weather" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Weather {
                    weather: parts.first().copied().unwrap_or("").to_string(),
                    from: parts.get(1).map(|s| s.to_string()),
                    of: parts.get(2).map(|s| s.to_string()),
                })
            }
            "-fieldstart" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::FieldStart {
                    condition: parts.first().copied().unwrap_or("").to_string(),
                    from: parts.get(1).map(|s| s.to_string()),
                    of: parts.get(2).map(|s| s.to_string()),
                })
            }
            "-fieldend" => Some(BattleMessage::FieldEnd {
                condition: args.to_string(),
            }),
            "-sidestart" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::SideStart {
                    side: parts.first().copied().unwrap_or("").to_string(),
                    condition: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-sideend" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::SideEnd {
                    side: parts.first().copied().unwrap_or("").to_string(),
                    condition: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-start" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::VolatileStart {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    effect: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    of: parts.get(3).map(|s| s.to_string()),
                })
            }
            "-end" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::VolatileEnd {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    effect: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-item" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::Item {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    item: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    of: parts.get(3).map(|s| s.to_string()),
                })
            }
            "-enditem" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::EndItem {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    item: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    eat: args.contains("[eat]"),
                })
            }
            "-ability" => {
                let parts: Vec<&str> = args.splitn(4, '|').collect();
                Some(BattleMessage::Ability {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    ability: parts.get(1).copied().unwrap_or("").to_string(),
                    from: parts.get(2).map(|s| s.to_string()),
                    of: parts.get(3).map(|s| s.to_string()),
                })
            }
            "-transform" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Transform {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    target: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-mega" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Mega {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    mega_stone: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-terastallize" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Terastallize {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    tera_type: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-supereffective" => Some(BattleMessage::SuperEffective {
                pokemon: args.to_string(),
            }),
            "-resisted" => Some(BattleMessage::Resisted {
                pokemon: args.to_string(),
            }),
            "-immune" => Some(BattleMessage::Immune {
                pokemon: args.to_string(),
            }),
            "-crit" => Some(BattleMessage::Crit {
                pokemon: args.to_string(),
            }),
            "-miss" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Miss {
                    source: parts.first().copied().unwrap_or("").to_string(),
                    target: parts.get(1).copied().unwrap_or("").to_string(),
                })
            }
            "-fail" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::Fail {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    effect: parts.get(1).map(|s| s.to_string()),
                })
            }
            "-hitcount" => {
                let parts: Vec<&str> = args.splitn(2, '|').collect();
                Some(BattleMessage::HitCount {
                    pokemon: parts.first().copied().unwrap_or("").to_string(),
                    count: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1),
                })
            }
            "-activate" => {
                let parts: Vec<&str> = args.splitn(3, '|').collect();
                Some(BattleMessage::Activate {
                    effect: parts.first().copied().unwrap_or("").to_string(),
                    pokemon: parts.get(1).map(|s| s.to_string()),
                    target: parts.get(2).map(|s| s.to_string()),
                })
            }
            "-hint" => Some(BattleMessage::Hint {
                message: args.to_string(),
            }),
            "-message" | "" => Some(BattleMessage::Message {
                message: args.to_string(),
            }),
            _ => Some(BattleMessage::Raw {
                line: line.to_string(),
            }),
        }
    }

    /// Convert message to protocol string
    /// Note: Rust-specific infrastructure. JavaScript works with protocol strings directly.
    /// This method provides type-safe serialization for the Rust implementation.
    pub fn to_protocol(&self) -> String {
        match self {
            BattleMessage::Player {
                slot,
                name,
                avatar,
                rating,
            } => {
                let mut parts = vec![format!("|player|{}|{}", slot, name)];
                if let Some(av) = avatar {
                    parts.push(av.clone());
                }
                if let Some(r) = rating {
                    parts.push(r.to_string());
                }
                parts.join("|")
            }
            BattleMessage::TeamSize { slot, size } => format!("|teamsize|{}|{}", slot, size),
            BattleMessage::GameType { game_type } => format!("|gametype|{}", game_type),
            BattleMessage::Gen { gen } => format!("|gen|{}", gen),
            BattleMessage::Tier { tier } => format!("|tier|{}", tier),
            BattleMessage::Rule { rule } => format!("|rule|{}", rule),
            BattleMessage::ClearPoke => "|clearpoke".to_string(),
            BattleMessage::Start => "|start".to_string(),
            BattleMessage::TeamPreview => "|teampreview".to_string(),
            BattleMessage::Turn { number } => format!("|turn|{}", number),
            BattleMessage::Upkeep => "|upkeep".to_string(),
            BattleMessage::Request { request_json } => format!("|request|{}", request_json),
            BattleMessage::Win { winner } => format!("|win|{}", winner),
            BattleMessage::Tie => "|tie".to_string(),
            BattleMessage::Switch {
                pokemon,
                details,
                hp_status,
            } => format!("|switch|{}|{}|{}", pokemon, details, hp_status),
            BattleMessage::Move {
                pokemon,
                move_name,
                target,
            } => {
                if let Some(t) = target {
                    format!("|move|{}|{}|{}", pokemon, move_name, t)
                } else {
                    format!("|move|{}|{}", pokemon, move_name)
                }
            }
            BattleMessage::Faint { pokemon } => format!("|faint|{}", pokemon),
            BattleMessage::Damage {
                pokemon,
                hp_status,
                from,
                of,
            } => {
                let mut s = format!("|-damage|{}|{}", pokemon, hp_status);
                if let Some(f) = from {
                    s.push_str(&format!("|{}", f));
                }
                if let Some(o) = of {
                    s.push_str(&format!("|{}", o));
                }
                s
            }
            BattleMessage::Heal {
                pokemon,
                hp_status,
                from,
                of,
            } => {
                let mut s = format!("|-heal|{}|{}", pokemon, hp_status);
                if let Some(f) = from {
                    s.push_str(&format!("|{}", f));
                }
                if let Some(o) = of {
                    s.push_str(&format!("|{}", o));
                }
                s
            }
            BattleMessage::Status { pokemon, status } => format!("|-status|{}|{}", pokemon, status),
            BattleMessage::CureStatus { pokemon, status } => {
                format!("|-curestatus|{}|{}", pokemon, status)
            }
            BattleMessage::Boost {
                pokemon,
                stat,
                amount,
            } => format!("|-boost|{}|{}|{}", pokemon, stat, amount),
            BattleMessage::Unboost {
                pokemon,
                stat,
                amount,
            } => format!("|-unboost|{}|{}|{}", pokemon, stat, amount),
            BattleMessage::Weather { weather, from, of } => {
                let mut s = format!("|-weather|{}", weather);
                if let Some(f) = from {
                    s.push_str(&format!("|{}", f));
                }
                if let Some(o) = of {
                    s.push_str(&format!("|{}", o));
                }
                s
            }
            BattleMessage::FieldStart {
                condition,
                from,
                of,
            } => {
                let mut s = format!("|-fieldstart|{}", condition);
                if let Some(f) = from {
                    s.push_str(&format!("|{}", f));
                }
                if let Some(o) = of {
                    s.push_str(&format!("|{}", o));
                }
                s
            }
            BattleMessage::FieldEnd { condition } => format!("|-fieldend|{}", condition),
            BattleMessage::SideStart { side, condition } => {
                format!("|-sidestart|{}|{}", side, condition)
            }
            BattleMessage::SideEnd { side, condition } => {
                format!("|-sideend|{}|{}", side, condition)
            }
            BattleMessage::SuperEffective { pokemon } => format!("|-supereffective|{}", pokemon),
            BattleMessage::Resisted { pokemon } => format!("|-resisted|{}", pokemon),
            BattleMessage::Immune { pokemon } => format!("|-immune|{}", pokemon),
            BattleMessage::Crit { pokemon } => format!("|-crit|{}", pokemon),
            BattleMessage::Fail { pokemon, effect } => {
                if let Some(e) = effect {
                    format!("|-fail|{}|{}", pokemon, e)
                } else {
                    format!("|-fail|{}", pokemon)
                }
            }
            BattleMessage::Hint { message } => format!("|-hint|{}", message),
            BattleMessage::Message { message } => format!("||{}", message),
            BattleMessage::Raw { line } => line.clone(),
            // Add more cases as needed
            _ => format!("{:?}", self),
        }
    }
}

/// Battle stream for managing a battle with protocol I/O
/// Equivalent to BattleStream class in battle-stream.ts
/// JavaScript equivalent: BattleStream (sim/battle-stream.ts)
/// 8 fields in JavaScript
pub struct BattleStream {
    /// The underlying battle
    /// JavaScript: battle: Battle
    battle: Option<Battle>,
    /// Output message queue
    /// JavaScript: (not explicitly stored, generated on-demand via battle.log)
    output_queue: VecDeque<String>,
    /// Debug mode
    /// JavaScript: debug: boolean
    pub debug: bool,
    /// No-catch mode (propagate errors instead of catching)
    /// JavaScript: noCatch: boolean
    pub no_catch: bool,
    /// Replay mode
    /// JavaScript: replay: 'spectator' | true | false
    /// TODO: Rust uses enum, JavaScript uses union type
    pub replay: ReplayMode,
    /// Keep alive (don't end stream on battle end)
    /// JavaScript: keepAlive: boolean
    pub keep_alive: bool,
    /// Last request received from battle
    /// JavaScript: Not stored in base class (abstract receiveRequest)
    /// Rust stores it for potential use by concrete implementations
    pub last_request: Option<serde_json::Value>,
}

/// Replay mode options
/// Equivalent to replay option in battle-stream.ts
/// JavaScript equivalent: 'spectator' | true | false (sim/battle-stream.ts)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ReplayMode {
    #[default]
    Off,
    Spectator,
    Full,
}

/// Stream options
/// Equivalent to BattleStream constructor options in battle-stream.ts
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: BattleStream constructor options (sim/battle-stream.ts)
/// 4 fields in JavaScript
pub struct BattleStreamOptions {
    /// Debug mode
    /// JavaScript: debug?: boolean
    pub debug: bool,
    /// No-catch mode
    /// JavaScript: noCatch?: boolean
    pub no_catch: bool,
    /// Keep alive mode
    /// JavaScript: keepAlive?: boolean
    pub keep_alive: bool,
    /// Replay mode
    /// JavaScript: replay?: 'spectator' | true | false
    pub replay: ReplayMode,
}

impl BattleStream {
}

// =========================================================================
// Player streams - split a BattleStream into per-player streams
// Equivalent to getPlayerStreams() in battle-stream.ts
// =========================================================================

/// Player streams container
/// Equivalent to the return type of getPlayerStreams() in battle-stream.ts
/// JavaScript equivalent: ObjectReadWriteStream[] (indexed by channel)
/// 6 fields in JavaScript (indexed streams)
pub struct PlayerStreams {
    /// Omniscient stream (sees all)
    /// JavaScript: streams[-1]
    pub omniscient: VecDeque<String>,
    /// Spectator stream (public info only)
    /// JavaScript: streams[0]
    pub spectator: VecDeque<String>,
    /// Player 1 stream
    /// JavaScript: streams[1]
    pub p1: VecDeque<String>,
    /// Player 2 stream
    /// JavaScript: streams[2]
    pub p2: VecDeque<String>,
    /// Player 3 stream (for multi battles)
    /// JavaScript: streams[3]
    pub p3: VecDeque<String>,
    /// Player 4 stream (for multi battles)
    /// JavaScript: streams[4]
    pub p4: VecDeque<String>,
}

impl PlayerStreams {
    /// Create new player streams
    /// Note: Rust-specific constructor. Part of getPlayerStreams() infrastructure.
    pub fn new() -> Self {
        Self {
            omniscient: VecDeque::new(),
            spectator: VecDeque::new(),
            p1: VecDeque::new(),
            p2: VecDeque::new(),
            p3: VecDeque::new(),
            p4: VecDeque::new(),
        }
    }

    /// Push update to appropriate channels
    /// Equivalent to the stream processing in getPlayerStreams()
    pub fn push_update(&mut self, data: &str, channels: &[i8]) {
        for &channel in channels {
            match channel {
                -1 => self.omniscient.push_back(data.to_string()),
                0 => self.spectator.push_back(data.to_string()),
                1 => self.p1.push_back(data.to_string()),
                2 => self.p2.push_back(data.to_string()),
                3 => self.p3.push_back(data.to_string()),
                4 => self.p4.push_back(data.to_string()),
                _ => {}
            }
        }
    }
}

impl Default for PlayerStreams {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BattleStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_turn() {
        let msg = BattleMessage::parse("|turn|5");
        assert_eq!(msg, Some(BattleMessage::Turn { number: 5 }));
    }

    #[test]
    fn test_parse_switch() {
        let msg = BattleMessage::parse("|switch|p1a: Pikachu|Pikachu, L50, M|100/100");
        assert!(matches!(msg, Some(BattleMessage::Switch { .. })));
        if let Some(BattleMessage::Switch {
            pokemon,
            details,
            hp_status,
        }) = msg
        {
            assert_eq!(pokemon, "p1a: Pikachu");
            assert_eq!(details, "Pikachu, L50, M");
            assert_eq!(hp_status, "100/100");
        }
    }

    #[test]
    fn test_parse_move() {
        let msg = BattleMessage::parse("|move|p1a: Pikachu|Thunderbolt|p2a: Charizard");
        assert!(matches!(msg, Some(BattleMessage::Move { .. })));
        if let Some(BattleMessage::Move {
            pokemon,
            move_name,
            target,
        }) = msg
        {
            assert_eq!(pokemon, "p1a: Pikachu");
            assert_eq!(move_name, "Thunderbolt");
            assert_eq!(target, Some("p2a: Charizard".to_string()));
        }
    }

    #[test]
    fn test_parse_damage() {
        let msg = BattleMessage::parse("|-damage|p2a: Charizard|75/100");
        assert!(matches!(msg, Some(BattleMessage::Damage { .. })));
    }

    #[test]
    fn test_parse_boost() {
        let msg = BattleMessage::parse("|-boost|p1a: Garchomp|atk|2");
        if let Some(BattleMessage::Boost {
            pokemon,
            stat,
            amount,
        }) = msg
        {
            assert_eq!(pokemon, "p1a: Garchomp");
            assert_eq!(stat, "atk");
            assert_eq!(amount, 2);
        }
    }

    #[test]
    fn test_parse_weather() {
        let msg =
            BattleMessage::parse("|-weather|RainDance|[from] ability: Drizzle|[of] p1a: Kyogre");
        assert!(matches!(msg, Some(BattleMessage::Weather { .. })));
    }

    #[test]
    fn test_parse_status() {
        let msg = BattleMessage::parse("|-status|p2a: Ferrothorn|brn");
        if let Some(BattleMessage::Status { pokemon, status }) = msg {
            assert_eq!(pokemon, "p2a: Ferrothorn");
            assert_eq!(status, "brn");
        }
    }

    #[test]
    fn test_to_protocol() {
        let msg = BattleMessage::Turn { number: 10 };
        assert_eq!(msg.to_protocol(), "|turn|10");

        let msg = BattleMessage::Boost {
            pokemon: "p1a: Garchomp".to_string(),
            stat: "atk".to_string(),
            amount: 2,
        };
        assert_eq!(msg.to_protocol(), "|-boost|p1a: Garchomp|atk|2");
    }

    #[test]
    fn test_battle_stream_creation() {
        let stream = BattleStream::new();
        assert!(stream.battle().is_none());
        assert!(stream.ended());
    }

    #[test]
    fn test_parse_terastallize() {
        let msg = BattleMessage::parse("|-terastallize|p1a: Dragonite|Normal");
        if let Some(BattleMessage::Terastallize { pokemon, tera_type }) = msg {
            assert_eq!(pokemon, "p1a: Dragonite");
            assert_eq!(tera_type, "Normal");
        }
    }
}
