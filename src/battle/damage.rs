use crate::*;

impl Battle {

    /// Deal damage to a Pokemon
    /// Matches JavaScript battle.ts:2165-2176 damage()
    /// This is a wrapper around spreadDamage for a single target
    /// Deal damage to Pokemon
    /// Equivalent to battle.ts damage() (battle.ts:2165-2175)
    ///
    //
    // 	damage(
    // 		damage: number, target: Pokemon | null = null, source: Pokemon | null = null,
    // 		effect: 'drain' | 'recoil' | Effect | null = null, instafaint = false
    // 	) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
    // 	}
    //
    pub fn damage(
        &mut self,
        damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        instafaint: bool,
    ) -> Option<i32> {
        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source, event_effect) = if let Some(ref event) = self.current_event
        {
            (event.target, event.source, event.effect.clone())
        } else {
            (None, None, None)
        };

        let target = target.or(event_target);
        let source = source.or(event_source);
        let effect_owned: Option<ID>;
        let effect = if effect.is_none() {
            effect_owned = event_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // JavaScript: return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
        let result = self.spread_damage(&[Some(damage)], &[target], source, effect, instafaint);
        result.first().copied().flatten()
    }
}
