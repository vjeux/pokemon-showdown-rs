// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use std::collections::HashMap;

impl Dex {

    /// Convert HashMap flags to MoveFlags struct
    pub fn convert_move_flags(flags: &HashMap<String, i32>) -> crate::battle_actions::MoveFlags {
        crate::battle_actions::MoveFlags {
            allyanim: flags.get("allyanim").map(|&v| v != 0).unwrap_or(false),
            bite: flags.get("bite").map(|&v| v != 0).unwrap_or(false),
            bullet: flags.get("bullet").map(|&v| v != 0).unwrap_or(false),
            bypasssub: flags.get("bypasssub").map(|&v| v != 0).unwrap_or(false),
            cant_use_twice: flags.get("cantusetwice").map(|&v| v != 0).unwrap_or(false),
            charge: flags.get("charge").map(|&v| v != 0).unwrap_or(false),
            contact: flags.get("contact").map(|&v| v != 0).unwrap_or(false),
            dance: flags.get("dance").map(|&v| v != 0).unwrap_or(false),
            defrost: flags.get("defrost").map(|&v| v != 0).unwrap_or(false),
            distance: flags.get("distance").map(|&v| v != 0).unwrap_or(false),
            failcopycat: flags.get("failcopycat").map(|&v| v != 0).unwrap_or(false),
            failencore: flags.get("failencore").map(|&v| v != 0).unwrap_or(false),
            failinstruct: flags.get("failinstruct").map(|&v| v != 0).unwrap_or(false),
            failmefirst: flags.get("failmefirst").map(|&v| v != 0).unwrap_or(false),
            failmimic: flags.get("failmimic").map(|&v| v != 0).unwrap_or(false),
            future_move: flags.get("futuremove").map(|&v| v != 0).unwrap_or(false),
            gravity: flags.get("gravity").map(|&v| v != 0).unwrap_or(false),
            heal: flags.get("heal").map(|&v| v != 0).unwrap_or(false),
            metronome: flags.get("metronome").map(|&v| v != 0).unwrap_or(false),
            mirror: flags.get("mirror").map(|&v| v != 0).unwrap_or(false),
            mustpressure: flags.get("mustpressure").map(|&v| v != 0).unwrap_or(false),
            noassist: flags.get("noassist").map(|&v| v != 0).unwrap_or(false),
            nonsky: flags.get("nonsky").map(|&v| v != 0).unwrap_or(false),
            noparentalbond: flags.get("noparentalbond").map(|&v| v != 0).unwrap_or(false),
            nosketch: flags.get("nosketch").map(|&v| v != 0).unwrap_or(false),
            nosleeptalk: flags.get("nosleeptalk").map(|&v| v != 0).unwrap_or(false),
            pledgecombo: flags.get("pledgecombo").map(|&v| v != 0).unwrap_or(false),
            powder: flags.get("powder").map(|&v| v != 0).unwrap_or(false),
            protect: flags.get("protect").map(|&v| v != 0).unwrap_or(false),
            pulse: flags.get("pulse").map(|&v| v != 0).unwrap_or(false),
            punch: flags.get("punch").map(|&v| v != 0).unwrap_or(false),
            recharge: flags.get("recharge").map(|&v| v != 0).unwrap_or(false),
            reflectable: flags.get("reflectable").map(|&v| v != 0).unwrap_or(false),
            slicing: flags.get("slicing").map(|&v| v != 0).unwrap_or(false),
            snatch: flags.get("snatch").map(|&v| v != 0).unwrap_or(false),
            sound: flags.get("sound").map(|&v| v != 0).unwrap_or(false),
            wind: flags.get("wind").map(|&v| v != 0).unwrap_or(false),
        }
    }
}
