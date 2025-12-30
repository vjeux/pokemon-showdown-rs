use crate::*;

impl<'a> BattleActions<'a> {

    /// Get Z-move base power from original move's base power
    pub fn z_move_power_table(base_power: i32) -> i32 {
        match base_power {
            0..=55 => 100,
            56..=65 => 120,
            66..=75 => 140,
            76..=85 => 160,
            86..=95 => 175,
            96..=100 => 180,
            101..=110 => 185,
            111..=125 => 190,
            126..=130 => 195,
            _ => 200,
        }
    }
}
