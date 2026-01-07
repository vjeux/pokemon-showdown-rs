///! Battle set_terrain method
///!
///! JavaScript source (field.ts setTerrain):
// 	setTerrain(status: string | Effect, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null) {
// 		status = this.battle.dex.conditions.get(status);
// 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
// 		if (!source && this.battle.event?.target) source = this.battle.event.target;
// 		if (source === 'debug') source = this.battle.sides[0].active[0];
// 		if (!source) throw new Error(`setting terrain without a source`);
//
// 		if (this.terrain === status.id) return false;
// 		const prevTerrain = this.terrain;
// 		const prevTerrainState = this.terrainState;
// 		this.terrain = status.id;
// 		this.terrainState = this.battle.initEffectState({
// 			id: status.id,
// 			source,
// 			sourceSlot: source.getSlot(),
// 			duration: status.duration,
// 		});
// 		if (status.durationCallback) {
// 			this.terrainState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
// 		}
// 		if (!this.battle.singleEvent('FieldStart', status, this.terrainState, this, source, sourceEffect)) {
// 			this.terrain = prevTerrain;
// 			this.terrainState = prevTerrainState;
// 			return false;
// 		}
// 		this.battle.eachEvent('TerrainChange', sourceEffect);
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event_system::EffectState;

impl Battle {
    pub fn set_terrain(&mut self, terrain_id: ID, source_pos: Option<(usize, usize)>) -> bool {
        // if (this.terrain === status.id) return false;
        if self.field.terrain == terrain_id {
            return false;
        }

        // const prevTerrain = this.terrain;
        // const prevTerrainState = this.terrainState;
        let prev_terrain = self.field.terrain.clone();
        let prev_terrain_state = self.field.terrain_state.clone();

        // this.terrain = status.id;
        self.field.terrain = terrain_id.clone();

        // this.terrainState = this.battle.initEffectState({
        //     id: status.id,
        //     source,
        //     sourceSlot: source.getSlot(),
        //     duration: status.duration,
        // });
        let mut terrain_state = EffectState::new(terrain_id.clone());
        terrain_state.source = source_pos;

        // Set duration from condition data
        // JavaScript: if (status.duration) this.terrainState.duration = status.duration;
        if let Some(condition) = self.dex.conditions().get_by_id(&terrain_id) {
            terrain_state.duration = condition.duration;
        }

        // if (status.durationCallback) {
        //     this.terrainState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
        // }
        if self.has_callback(&terrain_id, "durationCallback") {
            let duration_result = self.call_duration_callback(
                &terrain_id,
                None,  // target_pos
                source_pos,  // source_pos
                None,  // effect_id
            );

            if let crate::event::EventResult::Number(dur) = duration_result {
                terrain_state.duration = Some(dur);
            }
        }

        self.field.terrain_state = terrain_state;

        // if (!this.battle.singleEvent('FieldStart', status, this.terrainState, this, source, sourceEffect)) {
        //     this.terrain = prevTerrain;
        //     this.terrainState = prevTerrainState;
        //     return false;
        // }
        let field_start_result = self.single_event(
            "FieldStart",
            &crate::battle::Effect::terrain(terrain_id.clone()),
            None,  // field as target
            source_pos,
            None,
            None,
        );

        // Check if event returned false (event system returns Boolean(false) on failure)
        if matches!(field_start_result, crate::event::EventResult::Boolean(false)) {
            // Restore previous terrain
            self.field.terrain = prev_terrain;
            self.field.terrain_state = prev_terrain_state;
            return false;
        }

        // this.battle.eachEvent('TerrainChange', sourceEffect);
        self.each_event("TerrainChange", None, None);

        true
    }
}
