use crate::*;
use crate::battle_actions::AfterMoveResult;

impl<'a> BattleActions<'a> {

    /// After move secondary event
    /// Equivalent to afterMoveSecondaryEvent in battle-actions.ts
    // 	afterMoveSecondaryEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		// console.log(`${targets}, ${pokemon}, ${move}`)
    // 		if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce'))) {
    // 			this.battle.singleEvent('AfterMoveSecondary', move, null, targets[0], pokemon, move);
    // 			this.battle.runEvent('AfterMoveSecondary', targets, pokemon, move);
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn after_move_secondary_event(
        move_self_switch: Option<&str>,
        move_force_switch: bool,
    ) -> AfterMoveResult {
        AfterMoveResult {
            self_switch: move_self_switch.map(|s| s.to_string()),
            force_switch: move_force_switch,
        }
    }
}
