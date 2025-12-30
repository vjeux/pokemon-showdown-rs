use crate::*;

impl<'a> BattleActions<'a> {

    /// Check if move breaks protect
    /// Equivalent to hitStepBreakProtect in battle-actions.ts
    // 	hitStepBreakProtect(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		if (move.breaksProtect) {
    // 			for (const target of targets) {
    // 				let broke = false;
    // 				for (const effectid of [
    // 					'banefulbunker', 'burningbulwark', 'kingsshield', 'obstruct', 'protect', 'silktrap', 'spikyshield',
    // 				]) {
    // 					if (target.removeVolatile(effectid)) broke = true;
    // 				}
    // 				if (this.battle.gen >= 6 || !target.isAlly(pokemon)) {
    // 					for (const effectid of ['craftyshield', 'matblock', 'quickguard', 'wideguard']) {
    // 						if (target.side.removeSideCondition(effectid)) broke = true;
    // 					}
    // 				}
    // 				if (broke) {
    // 					if (move.id === 'feint') {
    // 						this.battle.add('-activate', target, 'move: Feint');
    // 					} else {
    // 						this.battle.add('-activate', target, `move: ${move.name}`, '[broken]');
    // 					}
    // 					if (this.battle.gen >= 6) delete target.volatiles['stall'];
    // 				}
    // 			}
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn hit_step_break_protect(
        move_breaks_protect: bool,
        move_is_z: bool,
        target_protected: bool,
    ) -> bool {
        if !target_protected {
            return true; // Not protected, can hit
        }
        if move_breaks_protect {
            return true; // Move breaks protect
        }
        if move_is_z {
            return true; // Z-moves break protect (with reduced damage)
        }
        false
    }
}
