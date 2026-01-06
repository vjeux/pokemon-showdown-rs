///! Battle clear_terrain method
///!
///! JavaScript source (field.ts clearTerrain):
// 	clearTerrain() {
// 		if (!this.terrain) return false;
// 		const prevTerrain = this.getTerrain();
// 		this.battle.singleEvent('FieldEnd', prevTerrain, this.terrainState, this);
// 		this.terrain = '';
// 		this.battle.clearEffectState(this.terrainState);
// 		this.battle.eachEvent('TerrainChange');
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event_system::EffectState;

impl Battle {
    pub fn clear_terrain(&mut self) -> bool {
        // if (!this.terrain) return false;
        if self.field.terrain.is_empty() {
            return false;
        }

        // const prevTerrain = this.getTerrain();
        let prev_terrain = self.field.terrain.clone();

        // this.battle.singleEvent('FieldEnd', prevTerrain, this.terrainState, this);
        self.single_event(
            "FieldEnd",
            &prev_terrain,
            None,  // field as target
            None,
            None,
            None,
        );

        // this.terrain = '';
        self.field.terrain = ID::empty();

        // this.battle.clearEffectState(this.terrainState);
        self.field.terrain_state = EffectState::new(ID::empty());

        // this.battle.eachEvent('TerrainChange');
        self.each_event("TerrainChange", None, None);

        true
    }
}
