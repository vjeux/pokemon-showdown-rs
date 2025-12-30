use crate::*;

impl<'a> BattleActions<'a> {

    /// Get Max move base power from original move's base power
    pub fn max_move_power_table(base_power: i32, move_type: &str) -> i32 {
        // Fighting and Poison type moves have different scaling
        let is_fighting_poison = move_type == "Fighting" || move_type == "Poison";

        match base_power {
            0..=40 => {
                if is_fighting_poison {
                    70
                } else {
                    90
                }
            }
            41..=50 => {
                if is_fighting_poison {
                    75
                } else {
                    100
                }
            }
            51..=60 => {
                if is_fighting_poison {
                    80
                } else {
                    110
                }
            }
            61..=70 => {
                if is_fighting_poison {
                    85
                } else {
                    120
                }
            }
            71..=100 => {
                if is_fighting_poison {
                    90
                } else {
                    130
                }
            }
            101..=140 => {
                if is_fighting_poison {
                    95
                } else {
                    140
                }
            }
            _ => {
                if is_fighting_poison {
                    100
                } else {
                    150
                }
            }
        }
    }
}
