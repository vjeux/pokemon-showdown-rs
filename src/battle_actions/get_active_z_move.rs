use crate::*;
use crate::battle_actions::ActiveMove;
use super::get_z_move_name;

impl<'a> BattleActions<'a> {

    /// Get active Z-move from base move
    /// Equivalent to getActiveZMove in battle-actions.ts
    //
    // 	getActiveZMove(move: Move, pokemon: Pokemon): ActiveMove {
    // 		if (pokemon) {
    // 			const item = pokemon.getItem();
    // 			if (move.name === item.zMoveFrom) {
    // 				const zMove = this.dex.getActiveMove(item.zMove as string);
    // 				zMove.isZOrMaxPowered = true;
    // 				return zMove;
    // 			}
    // 		}
    //
    // 		if (move.category === 'Status') {
    // 			const zMove = this.dex.getActiveMove(move);
    // 			zMove.isZ = true;
    // 			zMove.isZOrMaxPowered = true;
    // 			return zMove;
    // 		}
    // 		const zMove = this.dex.getActiveMove(this.Z_MOVES[move.type]);
    // 		zMove.basePower = move.zMove!.basePower!;
    // 		zMove.category = move.category;
    // 		// copy the priority for Quick Guard
    // 		zMove.priority = move.priority;
    // 		zMove.isZOrMaxPowered = true;
    // 		return zMove;
    // 	}
    //
    pub fn get_active_z_move(
        base_move_id: &str,
        base_move_type: &str,
        base_move_category: &str,
        base_move_base_power: i32,
        z_crystal_base_power: Option<i32>,
    ) -> ActiveMove {
        let z_move_name = get_z_move_name(base_move_type);
        let z_base_power = if base_move_category == "Status" {
            0
        } else {
            z_crystal_base_power.unwrap_or_else(|| Self::z_move_power_table(base_move_base_power))
        };

        ActiveMove {
            id: ID::new(&z_move_name.to_lowercase().replace(" ", "")),
            name: z_move_name.to_string(),
            base_power: z_base_power,
            category: base_move_category.to_string(),
            move_type: base_move_type.to_string(),
            accuracy: 0, // Z-moves always hit
            priority: 0,
            target: "normal".to_string(),
            is_z: true,
            is_z_or_max_powered: true,
            always_hit: true,
            base_move: Some(ID::new(base_move_id)),
            ..Default::default()
        }
    }
}
