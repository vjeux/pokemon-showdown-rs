// NOTE: This method is NOT in JavaScript - Rust-specific implementation
//
// Deserializes move flags from JSON HashMap<String, i32> to MoveFlags struct

use crate::battle_actions::MoveFlags;
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

pub fn deserialize_move_flags<'de, D>(deserializer: D) -> Result<MoveFlags, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct MoveFlagsVisitor;

    impl<'de> Visitor<'de> for MoveFlagsVisitor {
        type Value = MoveFlags;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of flag names to integers")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut flags = MoveFlags::default();

            while let Some(key) = map.next_key::<String>()? {
                let value: i32 = map.next_value()?;
                let is_set = value != 0;

                match key.as_str() {
                    "allyanim" => flags.allyanim = is_set,
                    "bite" => flags.bite = is_set,
                    "bullet" => flags.bullet = is_set,
                    "bypasssub" => flags.bypasssub = is_set,
                    "cantusetwice" => flags.cant_use_twice = is_set,
                    "charge" => flags.charge = is_set,
                    "contact" => flags.contact = is_set,
                    "dance" => flags.dance = is_set,
                    "defrost" => flags.defrost = is_set,
                    "distance" => flags.distance = is_set,
                    "failcopycat" => flags.failcopycat = is_set,
                    "failencore" => flags.failencore = is_set,
                    "failinstruct" => flags.failinstruct = is_set,
                    "failmefirst" => flags.failmefirst = is_set,
                    "failmimic" => flags.failmimic = is_set,
                    "futuremove" => flags.future_move = is_set,
                    "gravity" => flags.gravity = is_set,
                    "heal" => flags.heal = is_set,
                    "metronome" => flags.metronome = is_set,
                    "mirror" => flags.mirror = is_set,
                    "mustpressure" => flags.mustpressure = is_set,
                    "noassist" => flags.noassist = is_set,
                    "nonsky" => flags.nonsky = is_set,
                    "noparentalbond" => flags.noparentalbond = is_set,
                    "nosketch" => flags.nosketch = is_set,
                    "nosleeptalk" => flags.nosleeptalk = is_set,
                    "pledgecombo" => flags.pledgecombo = is_set,
                    "powder" => flags.powder = is_set,
                    "protect" => flags.protect = is_set,
                    "pulse" => flags.pulse = is_set,
                    "punch" => flags.punch = is_set,
                    "recharge" => flags.recharge = is_set,
                    "reflectable" => flags.reflectable = is_set,
                    "slicing" => flags.slicing = is_set,
                    "snatch" => flags.snatch = is_set,
                    "sound" => flags.sound = is_set,
                    "wind" => flags.wind = is_set,
                    _ => {
                        // Unknown flag - ignore for forward compatibility
                    }
                }
            }

            Ok(flags)
        }
    }

    deserializer.deserialize_map(MoveFlagsVisitor)
}
