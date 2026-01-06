///! Battle is_terrain method
///!
///! JavaScript source (field.ts isTerrain):
// 	isTerrain(terrain: string | string[], target?: Pokemon | Side | Battle) {
// 		const ourTerrain = this.effectiveTerrain(target);
// 		if (!Array.isArray(terrain)) {
// 			return ourTerrain === toID(terrain);
// 		}
// 		return terrain.map(toID).includes(ourTerrain);
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;

impl Battle {
    pub fn is_terrain(&self, terrain: &str) -> bool {
        // const ourTerrain = this.effectiveTerrain(target);
        // NOTE: target parameter not implemented yet
        let our_terrain = &self.field.terrain;
        // TODO: Implement effective_terrain with TryTerrain event

        // if (!Array.isArray(terrain)) {
        // 	return ourTerrain === toID(terrain);
        // }
        let terrain_id = ID::new(terrain);
        *our_terrain == terrain_id
        // NOTE: Array case handled at call sites
    }
}
