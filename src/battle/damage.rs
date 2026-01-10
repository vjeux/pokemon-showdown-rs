use crate::*;
use crate::battle::Effect;

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
        effect: Option<&Effect>,
        instafaint: bool,
    ) -> Option<i32> {
        eprintln!("[BATTLE_DAMAGE] Called with damage={}, target={:?}, source={:?}, effect={:?}",
            damage, target, source, effect.map(|e| e.id.as_str()));

        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source) = if let Some(ref event) = self.event
        {
            eprintln!("[BATTLE_DAMAGE] event exists: target={:?}, source={:?}",
                event.target, event.source);
            (event.target, event.source)
        } else {
            (None, None)
        };

        // JS uses this.effect (current handler's effect), NOT this.event.effect
        let current_effect = self.effect.clone();
        eprintln!("[BATTLE_DAMAGE] current effect (self.effect): {:?}",
            current_effect.as_ref().map(|e| e.id.as_str()));

        let target = target.or(event_target);
        let source = source.or(event_source);
        eprintln!("[BATTLE_DAMAGE] After merging with event: target={:?}, source={:?}", target, source);
        let effect_owned: Option<Effect>;
        let effect = if effect.is_none() {
            // Use self.effect (current handler's effect) as fallback, not self.event.effect
            effect_owned = current_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // JavaScript: return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
        use crate::battle_actions::{DamageResult, SpreadMoveTarget};

        let target = target?; // Early return if target is None
        let damages = vec![DamageResult::Damage(damage)];
        let targets = vec![SpreadMoveTarget::Target(target)];

        let result = self.spread_damage(damages, &targets, source, effect, instafaint);

        // Extract the damage value from the result
        match result.first() {
            Some(DamageResult::Damage(n)) => Some(*n),
            _ => None,
        }
    }
}
