//! MoveFlags struct - move flag attributes

use serde::{Deserialize, Serialize};

/// Move flags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: MoveFlags (sim/dex-moves.ts)
/// 37 fields in JavaScript
pub struct MoveFlags {
    /// Ally-targeting animation flag
    pub allyanim: bool,
    /// Bite flag (affected by Strong Jaw)
    pub bite: bool,
    /// Bullet flag (blocked by Bulletproof)
    pub bullet: bool,
    /// Bypass Substitute flag
    pub bypasssub: bool,
    /// Can't use twice flag (e.g., Dynamax Cannon)
    pub cant_use_twice: bool,
    /// Charge flag (two-turn moves)
    pub charge: bool,
    /// Contact flag (makes physical contact)
    pub contact: bool,
    /// Dance flag (copied by Dancer)
    pub dance: bool,
    /// Defrost flag (thaws user)
    pub defrost: bool,
    /// Distance flag (blocked by Wide Guard)
    pub distance: bool,
    /// Fail if used by Copycat
    pub failcopycat: bool,
    /// Fail if used by Encore
    pub failencore: bool,
    /// Fail if used by Instruct
    pub failinstruct: bool,
    /// Fail if used by Me First
    pub failmefirst: bool,
    /// Fail if used by Mimic
    pub failmimic: bool,
    /// Future move flag (Future Sight, Doom Desire)
    pub future_move: bool,
    /// Affected by Gravity
    pub gravity: bool,
    /// Heal flag (blocked by Heal Block)
    pub heal: bool,
    /// Can be used by Metronome
    pub metronome: bool,
    /// Mirror Move flag (can be copied)
    pub mirror: bool,
    /// Must pressure flag (must deduct PP under Pressure)
    pub mustpressure: bool,
    /// Blocked by Assist
    pub noassist: bool,
    /// Cannot be used in Sky Battles
    pub nonsky: bool,
    /// Not affected by Parental Bond
    pub noparentalbond: bool,
    /// Cannot be Sketched
    pub nosketch: bool,
    /// Cannot be used by Sleep Talk
    pub nosleeptalk: bool,
    /// Pledge combo flag
    pub pledgecombo: bool,
    /// Powder flag (blocked by Grass types and Overcoat)
    pub powder: bool,
    /// Protect flag (blocked by protection moves)
    pub protect: bool,
    /// Pulse flag (boosted by Mega Launcher)
    pub pulse: bool,
    /// Punch flag (boosted by Iron Fist)
    pub punch: bool,
    /// Recharge flag (requires recharge next turn)
    pub recharge: bool,
    /// Reflectable flag (can be bounced by Magic Coat)
    pub reflectable: bool,
    /// Slicing flag (boosted by Sharpness)
    pub slicing: bool,
    /// Snatch flag (can be stolen by Snatch)
    pub snatch: bool,
    /// Sound flag (blocked by Soundproof)
    pub sound: bool,
    /// Wind flag (boosted by Wind Power)
    pub wind: bool,
}

impl MoveFlags {
    /// Check if a flag is set by name (for compatibility with HashMap-style access)
    pub fn contains_key(&self, flag: &str) -> bool {
        self.get(flag).unwrap_or(false)
    }

    /// Get flag value by name (for compatibility with HashMap-style access)
    pub fn get(&self, flag: &str) -> Option<bool> {
        Some(match flag {
            "allyanim" => self.allyanim,
            "bite" => self.bite,
            "bullet" => self.bullet,
            "bypasssub" => self.bypasssub,
            "cantusetwice" => self.cant_use_twice,
            "charge" => self.charge,
            "contact" => self.contact,
            "dance" => self.dance,
            "defrost" => self.defrost,
            "distance" => self.distance,
            "failcopycat" => self.failcopycat,
            "failencore" => self.failencore,
            "failinstruct" => self.failinstruct,
            "failmefirst" => self.failmefirst,
            "failmimic" => self.failmimic,
            "futuremove" => self.future_move,
            "gravity" => self.gravity,
            "heal" => self.heal,
            "metronome" => self.metronome,
            "mirror" => self.mirror,
            "mustpressure" => self.mustpressure,
            "noassist" => self.noassist,
            "nonsky" => self.nonsky,
            "noparentalbond" => self.noparentalbond,
            "nosketch" => self.nosketch,
            "nosleeptalk" => self.nosleeptalk,
            "pledgecombo" => self.pledgecombo,
            "powder" => self.powder,
            "protect" => self.protect,
            "pulse" => self.pulse,
            "punch" => self.punch,
            "recharge" => self.recharge,
            "reflectable" => self.reflectable,
            "slicing" => self.slicing,
            "snatch" => self.snatch,
            "sound" => self.sound,
            "wind" => self.wind,
            _ => return None,
        })
    }
}
