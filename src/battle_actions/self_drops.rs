use crate::*;
use crate::dex_data::BoostsTable;

impl<'a> BattleActions<'a> {

    /// Process self stat drops from moves
    /// Equivalent to selfDrops in battle-actions.ts
    // 	selfDrops(
    // 		targets: SpreadMoveTargets, source: Pokemon,
    // 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
    // 	) {
    // 		for (const target of targets) {
    // 			if (target === false) continue;
    // 			if (moveData.self && !move.selfDropped) {
    // 				if (!isSecondary && moveData.self.boosts) {
    // 					const secondaryRoll = this.battle.random(100);
    // 					if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
    // 						this.moveHit(source, source, move, moveData.self, isSecondary, true);
    // 					}
    // 					if (!move.multihit) move.selfDropped = true;
    // 				} else {
    // 					this.moveHit(source, source, move, moveData.self, isSecondary, true);
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn self_drops(
        move_self_boost: Option<&BoostsTable>,
        already_dropped: bool,
    ) -> Option<BoostsTable> {
        if already_dropped {
            return None;
        }
        move_self_boost.cloned()
    }
}
