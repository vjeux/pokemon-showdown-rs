use crate::*;
use crate::battle_actions::ActiveMove;
use super::get_max_move_name;

impl<'a> BattleActions<'a> {

    /// Get active Max move from base move
    /// Equivalent to getActiveMaxMove in battle-actions.ts
    //
    // 	getActiveMaxMove(move: Move, pokemon: Pokemon) {
    // 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
    // 		if (move.name === 'Struggle') return this.dex.getActiveMove(move);
    // 		let maxMove = this.dex.getActiveMove(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
    // 		if (move.category !== 'Status') {
    // 			if (pokemon.gigantamax && pokemon.canGigantamax) {
    // 				const gMaxMove = this.dex.getActiveMove(pokemon.canGigantamax);
    // 				if (gMaxMove.exists && gMaxMove.type === move.type) maxMove = gMaxMove;
    // 			}
    // 			if (!move.maxMove?.basePower) throw new Error(`${move.name} doesn't have a maxMove basePower`);
    // 			if (!['gmaxdrumsolo', 'gmaxfireball', 'gmaxhydrosnipe'].includes(maxMove.id)) {
    // 				maxMove.basePower = move.maxMove.basePower;
    // 			}
    // 			maxMove.category = move.category;
    // 		}
    // 		maxMove.baseMove = move.id;
    // 		// copy the priority for Psychic Terrain, Quick Guard
    // 		maxMove.priority = move.priority;
    // 		maxMove.isZOrMaxPowered = true;
    // 		return maxMove;
    // 	}
    //
    pub fn get_active_max_move(
        base_move_id: &str,
        base_move_type: &str,
        base_move_category: &str,
        base_move_base_power: i32,
        gmax_move: Option<&str>,
    ) -> ActiveMove {
        let max_move_name = if let Some(gmax) = gmax_move {
            gmax.to_string()
        } else {
            get_max_move_name(if base_move_category == "Status" {
                "Status"
            } else {
                base_move_type
            })
            .to_string()
        };

        let max_base_power = if base_move_category == "Status" {
            0
        } else {
            Self::max_move_power_table(base_move_base_power, base_move_type)
        };

        ActiveMove {
            id: ID::new(
                &max_move_name
                    .to_lowercase()
                    .replace(" ", "")
                    .replace("-", ""),
            ),
            name: max_move_name,
            base_power: max_base_power,
            category: base_move_category.to_string(),
            move_type: base_move_type.to_string(),
            accuracy: 0, // Max moves always hit
            priority: 0,
            target: if base_move_category == "Status" {
                "self".to_string()
            } else {
                "adjacentFoe".to_string()
            },
            is_max: true,
            is_z_or_max_powered: true,
            always_hit: true,
            base_move: Some(ID::new(base_move_id)),
            ..Default::default()
        }
    }
}
