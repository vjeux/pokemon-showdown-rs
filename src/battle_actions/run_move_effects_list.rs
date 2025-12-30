use crate::*;
use crate::battle_actions::MoveEffects;
use crate::dex_data::BoostsTable;

impl<'a> BattleActions<'a> {

    /// Run move effects
    /// Equivalent to runMoveEffects in battle-actions.ts
    pub fn run_move_effects_list(
        move_boosts: Option<&BoostsTable>,
        move_heal: Option<(i32, i32)>,
        move_status: Option<&str>,
        move_volatile: Option<&str>,
    ) -> MoveEffects {
        MoveEffects {
            boosts: move_boosts.cloned(),
            heal: move_heal,
            status: move_status.map(|s| s.to_string()),
            volatile_status: move_volatile.map(|s| s.to_string()),
        }
    }
}
