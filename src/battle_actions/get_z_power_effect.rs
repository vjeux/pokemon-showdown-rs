use crate::*;
use crate::battle_actions::ZPowerEffect;

impl<'a> BattleActions<'a> {

    /// Run Z-Power effect (status Z-moves)
    /// Equivalent to runZPower in battle-actions.ts
    pub fn get_z_power_effect(z_move_effect: Option<&str>) -> Option<ZPowerEffect> {
        match z_move_effect {
            Some("heal") => Some(ZPowerEffect::Heal),
            Some("clearnegativeboost") => Some(ZPowerEffect::ClearNegativeBoost),
            Some("crit2") => Some(ZPowerEffect::Crit2),
            Some("redirect") => Some(ZPowerEffect::Redirect),
            Some("healreplacement") => Some(ZPowerEffect::HealReplacement),
            Some("curse") => Some(ZPowerEffect::Curse),
            _ => None,
        }
    }
}
