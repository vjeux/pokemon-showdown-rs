use crate::*;

impl Battle {

    /// Heal a Pokemon
    /// Matches JavaScript battle.ts:2231-2274 heal()
    /// Heal HP
    /// Equivalent to battle.ts heal() (battle.ts:2231-2273)
    ///
    //
    // 	heal(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: 'drain' | Effect | null = null) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (effect === 'drain') effect = this.dex.conditions.getByID(effect as ID);
    // 		if (damage && damage <= 1) damage = 1;
    // 		damage = this.trunc(damage);
    // 		// for things like Liquid Ooze, the Heal event still happens when nothing is healed.
    // 		damage = this.runEvent('TryHeal', target, source, effect, damage);
    // 		if (!damage) return damage;
    // 		if (!target?.hp) return false;
    // 		if (!target.isActive) return false;
    // 		if (target.hp >= target.maxhp) return false;
    // 		const finalDamage = target.heal(damage, source, effect);
    // 		switch (effect?.id) {
    // 		case 'leechseed':
    // 		case 'rest':
    // 			this.add('-heal', target, target.getHealth, '[silent]');
    // 			break;
    // 		case 'drain':
    // 			this.add('-heal', target, target.getHealth, '[from] drain', `[of] ${source}`);
    // 			break;
    // 		case 'wish':
    // 			break;
    // 		case 'zpower':
    // 			this.add('-heal', target, target.getHealth, '[zeffect]');
    // 			break;
    // 		default:
    // 			if (!effect) break;
    // 			if (effect.effectType === 'Move') {
    // 				this.add('-heal', target, target.getHealth);
    // 			} else if (source && source !== target) {
    // 				this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`, `[of] ${source}`);
    // 			} else {
    // 				this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`);
    // 			}
    // 			break;
    // 		}
    // 		this.runEvent('Heal', target, source, effect, finalDamage);
    // 		return finalDamage;
    // 	}
    //
    pub fn heal(
        &mut self,
        mut damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
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

        // Clamp damage to at least 1
        if damage > 0 && damage <= 1 {
            damage = 1;
        }
        // JavaScript: damage = this.trunc(damage);
        // Already an integer in Rust

        // Get target, handle None case
        let target_pos = match target {
            Some(pos) => pos,
            None => return Some(0),
        };

        // Fire TryHeal event
        // JavaScript: damage = this.runEvent('TryHeal', target, source, effect, damage);
        let event_result =
            self.run_event("TryHeal", Some(target_pos), source, effect, Some(damage));
        damage = event_result?;

        if damage == 0 {
            return Some(0);
        }

        // Check target validity
        let (side_idx, poke_idx) = target_pos;
        let (has_hp, is_active, at_max_hp) = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                (
                    pokemon.hp > 0,
                    pokemon.is_active,
                    pokemon.hp >= pokemon.maxhp,
                )
            } else {
                return None; // JavaScript returns false
            }
        } else {
            return None;
        };

        if !has_hp {
            return None;
        }
        if !is_active {
            return None;
        }
        if at_max_hp {
            return None;
        }

        // Apply healing using Pokemon's heal method
        let final_damage = if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                let old_hp = pokemon.hp;
                pokemon.hp = (pokemon.hp + damage).min(pokemon.maxhp);
                pokemon.hp - old_hp
            } else {
                0
            }
        } else {
            0
        };

        // Add heal log message
        self.add_heal_log(target_pos, source, effect);

        // Fire Heal event
        // JavaScript: this.runEvent('Heal', target, source, effect, finalDamage);
        self.run_event("Heal", Some(target_pos), source, effect, Some(final_damage));

        Some(final_damage)
    }
}
