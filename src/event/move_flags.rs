//! Move Flags

use serde::{Deserialize, Serialize};

/// Flags that can be set on moves
/// JavaScript equivalent: MoveFlags (sim/dex-moves.ts)
/// 27 fields in JavaScript
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveFlags {
    /// Makes contact
    pub contact: bool,
    /// Can be protected against
    pub protect: bool,
    /// Can be reflected by Magic Bounce
    pub reflectable: bool,
    /// Can be copied by Mirror Move
    pub mirror: bool,
    /// Uses sound
    pub sound: bool,
    /// Affected by Gravity
    pub gravity: bool,
    /// Is a punch move (Iron Fist)
    pub punch: bool,
    /// Is a biting move (Strong Jaw)
    pub bite: bool,
    /// Is a bullet/ball move (Bulletproof)
    pub bullet: bool,
    /// Is a slicing move
    pub slicing: bool,
    /// Is a wind move
    pub wind: bool,
    /// Is a powder move
    pub powder: bool,
    /// Bypasses Substitute
    pub bypasssub: bool,
    /// Cannot be Encored
    pub failencore: bool,
    /// High crit ratio
    pub high_crit_ratio: bool,
    /// Cannot be copied by Mimic/Sketch
    pub no_mimic: bool,
    /// Cannot be copied by Metronome
    pub no_metronome: bool,
    /// Defrosts user
    pub defrost: bool,
    /// Uses recharge
    pub recharge: bool,
    /// Is a heal move
    pub heal: bool,
    /// Is a dance move
    pub dance: bool,
    /// Can target distant Pokemon
    pub distance: bool,
    /// Is a pulse move (Mega Launcher)
    pub pulse: bool,
    /// Is a charge move (turn 1)
    pub charge: bool,
    // Stolen by Snatch
    pub snatch: bool,
    // Cannot be used in Sky Battles
    pub nonsky: bool,
}

impl MoveFlags {
    /// Check if a move makes contact
    pub fn makes_contact(&self) -> bool {
        self.contact
    }
}
