use crate::*;

impl<'a> BattleActions<'a> {

    /// Check critical hit
    /// Equivalent to critical hit calculation in battle-actions.ts
    pub fn is_critical_hit(
        crit_ratio: i32,
        attacker_focus_energy: bool,
        move_will_crit: Option<bool>,
        random_value: i32,
    ) -> bool {
        // Guaranteed crit or no crit
        if let Some(will_crit) = move_will_crit {
            return will_crit;
        }

        let mut crit_stages = crit_ratio;
        if attacker_focus_energy {
            crit_stages += 2;
        }

        // Crit chance by stage (Gen 7+)
        let crit_chance = match crit_stages {
            0 => 24, // 1/24
            1 => 8,  // 1/8
            2 => 2,  // 1/2
            _ => 1,  // Always crit at stage 3+
        };

        random_value % crit_chance == 0
    }
}
