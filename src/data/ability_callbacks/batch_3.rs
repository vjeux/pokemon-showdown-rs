//! Ability Callback Handlers - Batch 3
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file contains ability callback implementations for batch 3.
//! Generated from data/abilities.ts
//!
//! Abilities in this batch (79):
//! - neutralizinggas
//! - noability
//! - noguard
//! - normalize
//! - oblivious
//! - opportunist
//! - orichalcumpulse
//! - overcoat
//! - overgrow
//! - owntempo
//! - parentalbond
//! - pastelveil
//! - perishbody
//! - persistent
//! - pickpocket
//! - pickup
//! - pixilate
//! - plus
//! - poisonheal
//! - poisonpoint
//! - poisonpuppeteer
//! - poisontouch
//! - powerconstruct
//! - powerofalchemy
//! - powerspot
//! - prankster
//! - pressure
//! - primordialsea
//! - prismarmor
//! - propellertail
//! - protean
//! - protosynthesis
//! - psychicsurge
//! - punkrock
//! - purepower
//! - purifyingsalt
//! - quarkdrive
//! - queenlymajesty
//! - quickdraw
//! - quickfeet
//! - raindish
//! - rattled
//! - rebound
//! - receiver
//! - reckless
//! - refrigerate
//! - regenerator
//! - ripen
//! - rivalry
//! - rkssystem
//! - rockhead
//! - rockypayload
//! - roughskin
//! - runaway
//! - sandforce
//! - sandrush
//! - sandspit
//! - sandstream
//! - sandveil
//! - sapsipper
//! - schooling
//! - scrappy
//! - screencleaner
//! - seedsower
//! - serenegrace
//! - shadowshield
//! - shadowtag
//! - sharpness
//! - shedskin
//! - sheerforce
//! - shellarmor
//! - shielddust
//! - shieldsdown
//! - simple
//! - skilllink
//! - slowstart
//! - slushrush
//! - sniper
//! - snowcloak

use crate::battle::{Battle, Arg};
use crate::data::moves::MoveDef;
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};


// -----------------------------------------------------------------------------
// NEUTRALIZINGGAS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	neutralizinggas: {
// 		// Ability suppression implemented in sim/pokemon.ts:Pokemon#ignoringAbility
// 		onSwitchInPriority: 2,
// 		onSwitchIn(pokemon) {
// 			this.add('-ability', pokemon, 'Neutralizing Gas');
// 			pokemon.abilityState.ending = false;
// 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
// 			for (const target of this.getAllActive()) {
// 				if (target.hasItem('Ability Shield')) {
// 					this.add('-block', target, 'item: Ability Shield');
// 					continue;
// 				}
// 				// Can't suppress a Tatsugiri inside of Dondozo already
// 				if (target.volatiles['commanding']) {
// 					continue;
// 				}
// 				if (target.illusion) {
// 					this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, pokemon, 'neutralizinggas');
// 				}
// 				if (target.volatiles['slowstart']) {
// 					delete target.volatiles['slowstart'];
// 					this.add('-end', target, 'Slow Start', '[silent]');
// 				}
// 				if (strongWeathers.includes(target.getAbility().id)) {
// 					this.singleEvent('End', this.dex.abilities.get(target.getAbility().id), target.abilityState, target, pokemon, 'neutralizinggas');
// 				}
// 			}
// 		},
// 		onEnd(source) {
// 			if (source.transformed) return;
// 			for (const pokemon of this.getAllActive()) {
// 				if (pokemon !== source && pokemon.hasAbility('Neutralizing Gas')) {
// 					return;
// 				}
// 			}
// 			this.add('-end', source, 'ability: Neutralizing Gas');
// 
// 			// FIXME this happens before the pokemon switches out, should be the opposite order.
// 			// Not an easy fix since we cant use a supported event. Would need some kind of special event that
// 			// gathers events to run after the switch and then runs them when the ability is no longer accessible.
// 			// (If you're tackling this, do note extreme weathers have the same issue)
// 
// 			// Mark this pokemon's ability as ending so Pokemon#ignoringAbility skips it
// 			if (source.abilityState.ending) return;
// 			source.abilityState.ending = true;
// 			const sortedActive = this.getAllActive();
// 			this.speedSort(sortedActive);
// 			for (const pokemon of sortedActive) {
// 				if (pokemon !== source) {
// 					if (pokemon.getAbility().flags['cantsuppress']) continue; // does not interact with e.g Ice Face, Zen Mode
// 					if (pokemon.hasItem('abilityshield')) continue; // don't restart abilities that weren't suppressed
// 
// 					// Will be suppressed by Pokemon#ignoringAbility if needed
// 					this.singleEvent('Start', pokemon.getAbility(), pokemon.abilityState, pokemon);
// 					if (pokemon.ability === "gluttony") {
// 						pokemon.abilityState.gluttony = false;
// 					}
// 				}
// 			}
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
// 		name: "Neutralizing Gas",
// 		rating: 3.5,
// 		num: 256,
// 	},

pub mod neutralizinggas {
    use super::*;

    /// onSwitchInPriority(...)
    pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSwitchIn(...)
    pub fn on_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// NOGUARD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	noguard: {
// 		onAnyInvulnerabilityPriority: 1,
// 		onAnyInvulnerability(target, source, move) {
// 			if (move && (source === this.effectState.target || target === this.effectState.target)) return 0;
// 		},
// 		onAnyAccuracy(accuracy, target, source, move) {
// 			if (move && (source === this.effectState.target || target === this.effectState.target)) {
// 				return true;
// 			}
// 			return accuracy;
// 		},
// 		flags: {},
// 		name: "No Guard",
// 		rating: 4,
// 		num: 99,
// 	},

pub mod noguard {
    use super::*;

    /// onAnyInvulnerabilityPriority(...)
    pub fn on_any_invulnerability_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyInvulnerability(...)
    pub fn on_any_invulnerability(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyAccuracy(...)
    pub fn on_any_accuracy(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// NORMALIZE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	normalize: {
// 		onModifyTypePriority: 1,
// 		onModifyType(move, pokemon) {
// 			const noModifyType = [
// 				'hiddenpower', 'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'struggle', 'technoblast', 'terrainpulse', 'weatherball',
// 			];
// 			if (!(move.isZ && move.category !== 'Status') &&
// 				// TODO: Figure out actual interaction
// 				(!noModifyType.includes(move.id) || this.activeMove?.isMax) && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
// 				move.type = 'Normal';
// 				move.typeChangerBoosted = this.effect;
// 			}
// 		},
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, pokemon, target, move) {
// 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
// 		},
// 		flags: {},
// 		name: "Normalize",
// 		rating: 0,
// 		num: 96,
// 	},

pub mod normalize {
    use super::*;

    /// onModifyTypePriority(...)
    pub fn on_modify_type_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyType(...)
    pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// OBLIVIOUS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	oblivious: {
// 		onUpdate(pokemon) {
// 			if (pokemon.volatiles['attract']) {
// 				this.add('-activate', pokemon, 'ability: Oblivious');
// 				pokemon.removeVolatile('attract');
// 				this.add('-end', pokemon, 'move: Attract', '[from] ability: Oblivious');
// 			}
// 			if (pokemon.volatiles['taunt']) {
// 				this.add('-activate', pokemon, 'ability: Oblivious');
// 				pokemon.removeVolatile('taunt');
// 				// Taunt's volatile already sends the -end message when removed
// 			}
// 		},
// 		onImmunity(type, pokemon) {
// 			if (type === 'attract') return false;
// 		},
// 		onTryHit(pokemon, target, move) {
// 			if (move.id === 'attract' || move.id === 'captivate' || move.id === 'taunt') {
// 				this.add('-immune', pokemon, '[from] ability: Oblivious');
// 				return null;
// 			}
// 		},
// 		onTryBoost(boost, target, source, effect) {
// 			if (effect.name === 'Intimidate' && boost.atk) {
// 				delete boost.atk;
// 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Oblivious', `[of] ${target}`);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Oblivious",
// 		rating: 1.5,
// 		num: 12,
// 	},

pub mod oblivious {
    use super::*;

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// OPPORTUNIST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	opportunist: {
// 		onFoeAfterBoost(boost, target, source, effect) {
// 			if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
// 			if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
// 			const boostPlus = this.effectState.boosts;
// 			let i: BoostID;
// 			for (i in boost) {
// 				if (boost[i]! > 0) {
// 					boostPlus[i] = (boostPlus[i] || 0) + boost[i]!;
// 				}
// 			}
// 		},
// 		onAnySwitchInPriority: -3,
// 		onAnySwitchIn() {
// 			if (!this.effectState.boosts) return;
// 			this.boost(this.effectState.boosts, this.effectState.target);
// 			delete this.effectState.boosts;
// 		},
// 		onAnyAfterMega() {
// 			if (!this.effectState.boosts) return;
// 			this.boost(this.effectState.boosts, this.effectState.target);
// 			delete this.effectState.boosts;
// 		},
// 		onAnyAfterTerastallization() {
// 			if (!this.effectState.boosts) return;
// 			this.boost(this.effectState.boosts, this.effectState.target);
// 			delete this.effectState.boosts;
// 		},
// 		onAnyAfterMove() {
// 			if (!this.effectState.boosts) return;
// 			this.boost(this.effectState.boosts, this.effectState.target);
// 			delete this.effectState.boosts;
// 		},
// 		onResidualOrder: 29,
// 		onResidual(pokemon) {
// 			if (!this.effectState.boosts) return;
// 			this.boost(this.effectState.boosts, this.effectState.target);
// 			delete this.effectState.boosts;
// 		},
// 		onEnd() {
// 			delete this.effectState.boosts;
// 		},
// 		flags: {},
// 		name: "Opportunist",
// 		rating: 3,
// 		num: 290,
// 	},

pub mod opportunist {
    use super::*;

    /// onFoeAfterBoost(...)
    pub fn on_foe_after_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnySwitchInPriority(...)
    pub fn on_any_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnySwitchIn(...)
    pub fn on_any_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyAfterMega(...)
    pub fn on_any_after_mega(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyAfterTerastallization(...)
    pub fn on_any_after_terastallization(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyAfterMove(...)
    pub fn on_any_after_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ORICHALCUMPULSE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	orichalcumpulse: {
// 		onStart(pokemon) {
// 			if (this.field.setWeather('sunnyday')) {
// 				this.add('-activate', pokemon, 'Orichalcum Pulse', '[source]');
// 			} else if (this.field.isWeather('sunnyday')) {
// 				this.add('-activate', pokemon, 'ability: Orichalcum Pulse');
// 			}
// 		},
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk, pokemon) {
// 			if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
// 				this.debug('Orichalcum boost');
// 				return this.chainModify([5461, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Orichalcum Pulse",
// 		rating: 4.5,
// 		num: 288,
// 	},

pub mod orichalcumpulse {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// OVERCOAT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	overcoat: {
// 		onImmunity(type, pokemon) {
// 			if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
// 		},
// 		onTryHitPriority: 1,
// 		onTryHit(target, source, move) {
// 			if (move.flags['powder'] && target !== source && this.dex.getImmunity('powder', target)) {
// 				this.add('-immune', target, '[from] ability: Overcoat');
// 				return null;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Overcoat",
// 		rating: 2,
// 		num: 142,
// 	},

pub mod overcoat {
    use super::*;

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryHitPriority(...)
    pub fn on_try_hit_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// OVERGROW
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	overgrow: {
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk, attacker, defender, move) {
// 			if (move.type === 'Grass' && attacker.hp <= attacker.maxhp / 3) {
// 				this.debug('Overgrow boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		onModifySpAPriority: 5,
// 		onModifySpA(atk, attacker, defender, move) {
// 			if (move.type === 'Grass' && attacker.hp <= attacker.maxhp / 3) {
// 				this.debug('Overgrow boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Overgrow",
// 		rating: 2,
// 		num: 65,
// 	},

pub mod overgrow {
    use super::*;

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpAPriority(...)
    pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(...)
    pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// OWNTEMPO
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	owntempo: {
// 		onUpdate(pokemon) {
// 			if (pokemon.volatiles['confusion']) {
// 				this.add('-activate', pokemon, 'ability: Own Tempo');
// 				pokemon.removeVolatile('confusion');
// 			}
// 		},
// 		onTryAddVolatile(status, pokemon) {
// 			if (status.id === 'confusion') return null;
// 		},
// 		onHit(target, source, move) {
// 			if (move?.volatileStatus === 'confusion') {
// 				this.add('-immune', target, 'confusion', '[from] ability: Own Tempo');
// 			}
// 		},
// 		onTryBoost(boost, target, source, effect) {
// 			if (effect.name === 'Intimidate' && boost.atk) {
// 				delete boost.atk;
// 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Own Tempo', `[of] ${target}`);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Own Tempo",
// 		rating: 1.5,
// 		num: 20,
// 	},

pub mod owntempo {
    use super::*;

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryAddVolatile(...)
    pub fn on_try_add_volatile(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onHit(...)
    pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PARENTALBOND
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	parentalbond: {
// 		onPrepareHit(source, target, move) {
// 			if (move.category === 'Status' || move.multihit || move.flags['noparentalbond'] || move.flags['charge'] ||
// 				move.flags['futuremove'] || move.spreadHit || move.isZ || move.isMax) return;
// 			move.multihit = 2;
// 			move.multihitType = 'parentalbond';
// 		},
// 		// Damage modifier implemented in BattleActions#modifyDamage()
// 		onSourceModifySecondaries(secondaries, target, source, move) {
// 			if (move.multihitType === 'parentalbond' && move.id === 'secretpower' && move.hit < 2) {
// 				// hack to prevent accidentally suppressing King's Rock/Razor Fang
// 				return secondaries.filter(effect => effect.volatileStatus === 'flinch');
// 			}
// 		},
// 		flags: {},
// 		name: "Parental Bond",
// 		rating: 4.5,
// 		num: 185,
// 	},

pub mod parentalbond {
    use super::*;

    /// onPrepareHit(...)
    pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifySecondaries(...)
    pub fn on_source_modify_secondaries(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PASTELVEIL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	pastelveil: {
// 		onStart(pokemon) {
// 			for (const ally of pokemon.alliesAndSelf()) {
// 				if (['psn', 'tox'].includes(ally.status)) {
// 					this.add('-activate', pokemon, 'ability: Pastel Veil');
// 					ally.cureStatus();
// 				}
// 			}
// 		},
// 		onUpdate(pokemon) {
// 			if (['psn', 'tox'].includes(pokemon.status)) {
// 				this.add('-activate', pokemon, 'ability: Pastel Veil');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		onAnySwitchIn() {
// 			((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
// 		},
// 		onSetStatus(status, target, source, effect) {
// 			if (!['psn', 'tox'].includes(status.id)) return;
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Pastel Veil');
// 			}
// 			return false;
// 		},
// 		onAllySetStatus(status, target, source, effect) {
// 			if (!['psn', 'tox'].includes(status.id)) return;
// 			if ((effect as Move)?.status) {
// 				const effectHolder = this.effectState.target;
// 				this.add('-block', target, 'ability: Pastel Veil', `[of] ${effectHolder}`);
// 			}
// 			return false;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Pastel Veil",
// 		rating: 2,
// 		num: 257,
// 	},

pub mod pastelveil {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnySwitchIn(...)
    pub fn on_any_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(...)
    pub fn on_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAllySetStatus(...)
    pub fn on_ally_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PERISHBODY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	perishbody: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
// 			this.add('-ability', target, 'Perish Body');
// 			source.addVolatile('perishsong');
// 			target.addVolatile('perishsong');
// 		},
// 		flags: {},
// 		name: "Perish Body",
// 		rating: 1,
// 		num: 253,
// 	},

pub mod perishbody {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PICKPOCKET
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	pickpocket: {
// 		onAfterMoveSecondary(target, source, move) {
// 			if (source && source !== target && move?.flags['contact']) {
// 				if (target.item || target.switchFlag || target.forceSwitchFlag || source.switchFlag === true) {
// 					return;
// 				}
// 				const yourItem = source.takeItem(target);
// 				if (!yourItem) {
// 					return;
// 				}
// 				if (!target.setItem(yourItem)) {
// 					source.item = yourItem.id;
// 					return;
// 				}
// 				this.add('-enditem', source, yourItem, '[silent]', '[from] ability: Pickpocket', `[of] ${source}`);
// 				this.add('-item', target, yourItem, '[from] ability: Pickpocket', `[of] ${source}`);
// 			}
// 		},
// 		flags: {},
// 		name: "Pickpocket",
// 		rating: 1,
// 		num: 124,
// 	},

pub mod pickpocket {
    use super::*;

    /// onAfterMoveSecondary(...)
    pub fn on_after_move_secondary(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PICKUP
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	pickup: {
// 		onResidualOrder: 28,
// 		onResidualSubOrder: 2,
// 		onResidual(pokemon) {
// 			if (pokemon.item) return;
// 			const pickupTargets = this.getAllActive().filter(target => (
// 				target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
// 			));
// 			if (!pickupTargets.length) return;
// 			const randomTarget = this.sample(pickupTargets);
// 			const item = randomTarget.lastItem;
// 			randomTarget.lastItem = '';
// 			this.add('-item', pokemon, this.dex.items.get(item), '[from] ability: Pickup');
// 			pokemon.setItem(item);
// 		},
// 		flags: {},
// 		name: "Pickup",
// 		rating: 0.5,
// 		num: 53,
// 	},

pub mod pickup {
    use super::*;

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualSubOrder(...)
    pub fn on_residual_sub_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PIXILATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	pixilate: {
// 		onModifyTypePriority: -1,
// 		onModifyType(move, pokemon) {
// 			const noModifyType = [
// 				'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
// 			];
// 			if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
// 				!(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
// 				move.type = 'Fairy';
// 				move.typeChangerBoosted = this.effect;
// 			}
// 		},
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, pokemon, target, move) {
// 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
// 		},
// 		flags: {},
// 		name: "Pixilate",
// 		rating: 4,
// 		num: 182,
// 	},

pub mod pixilate {
    use super::*;

    /// onModifyTypePriority(...)
    pub fn on_modify_type_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyType(...)
    pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PLUS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	plus: {
// 		onModifySpAPriority: 5,
// 		onModifySpA(spa, pokemon) {
// 			for (const allyActive of pokemon.allies()) {
// 				if (allyActive.hasAbility(['minus', 'plus'])) {
// 					return this.chainModify(1.5);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Plus",
// 		rating: 0,
// 		num: 57,
// 	},

pub mod plus {
    use super::*;

    /// onModifySpAPriority(...)
    pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(...)
    pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POISONHEAL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	poisonheal: {
// 		onDamagePriority: 1,
// 		onDamage(damage, target, source, effect) {
// 			if (effect.id === 'psn' || effect.id === 'tox') {
// 				this.heal(target.baseMaxhp / 8);
// 				return false;
// 			}
// 		},
// 		flags: {},
// 		name: "Poison Heal",
// 		rating: 4,
// 		num: 90,
// 	},

pub mod poisonheal {
    use super::*;

    /// onDamagePriority(...)
    pub fn on_damage_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onDamage(...)
    pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POISONPOINT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	poisonpoint: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (this.checkMoveMakesContact(move, source, target)) {
// 				if (this.randomChance(3, 10)) {
// 					source.trySetStatus('psn', target);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Poison Point",
// 		rating: 1.5,
// 		num: 38,
// 	},

pub mod poisonpoint {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POISONPUPPETEER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	poisonpuppeteer: {
// 		onAnyAfterSetStatus(status, target, source, effect) {
// 			if (source.baseSpecies.name !== "Pecharunt") return;
// 			if (source !== this.effectState.target || target === source || effect.effectType !== 'Move') return;
// 			if (status.id === 'psn' || status.id === 'tox') {
// 				target.addVolatile('confusion');
// 			}
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1 },
// 		name: "Poison Puppeteer",
// 		rating: 3,
// 		num: 310,
// 	},

pub mod poisonpuppeteer {
    use super::*;

    /// onAnyAfterSetStatus(...)
    pub fn on_any_after_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POISONTOUCH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	poisontouch: {
// 		onSourceDamagingHit(damage, target, source, move) {
// 			// Despite not being a secondary, Shield Dust / Covert Cloak block Poison Touch's effect
// 			if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
// 			if (this.checkMoveMakesContact(move, target, source)) {
// 				if (this.randomChance(3, 10)) {
// 					target.trySetStatus('psn', source);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Poison Touch",
// 		rating: 2,
// 		num: 143,
// 	},

pub mod poisontouch {
    use super::*;

    /// onSourceDamagingHit(...)
    pub fn on_source_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POWERCONSTRUCT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	powerconstruct: {
// 		onResidualOrder: 29,
// 		onResidual(pokemon) {
// 			if (pokemon.baseSpecies.baseSpecies !== 'Zygarde' || pokemon.transformed || !pokemon.hp) return;
// 			if (pokemon.species.id === 'zygardecomplete' || pokemon.hp > pokemon.maxhp / 2) return;
// 			this.add('-activate', pokemon, 'ability: Power Construct');
// 			pokemon.formeChange('Zygarde-Complete', this.effect, true);
// 			pokemon.canMegaEvo = pokemon.canMegaEvo === false ? false : this.actions.canMegaEvo(pokemon);
// 			pokemon.formeRegression = true;
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
// 		name: "Power Construct",
// 		rating: 5,
// 		num: 211,
// 	},

pub mod powerconstruct {
    use super::*;

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POWEROFALCHEMY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	powerofalchemy: {
// 		onAllyFaint(target) {
// 			if (!this.effectState.target.hp) return;
// 			const ability = target.getAbility();
// 			if (ability.flags['noreceiver'] || ability.id === 'noability') return;
// 			this.effectState.target.setAbility(ability, target);
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
// 		name: "Power of Alchemy",
// 		rating: 0,
// 		num: 223,
// 	},

pub mod powerofalchemy {
    use super::*;

    /// onAllyFaint(...)
    pub fn on_ally_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POWERSPOT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	powerspot: {
// 		onAllyBasePowerPriority: 22,
// 		onAllyBasePower(basePower, attacker, defender, move) {
// 			if (attacker !== this.effectState.target) {
// 				this.debug('Power Spot boost');
// 				return this.chainModify([5325, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Power Spot",
// 		rating: 0,
// 		num: 249,
// 	},

pub mod powerspot {
    use super::*;

    /// onAllyBasePowerPriority(...)
    pub fn on_ally_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAllyBasePower(...)
    pub fn on_ally_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PRANKSTER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	prankster: {
// 		onModifyPriority(priority, pokemon, target, move) {
// 			if (move?.category === 'Status') {
// 				move.pranksterBoosted = true;
// 				return priority + 1;
// 			}
// 		},
// 		flags: {},
// 		name: "Prankster",
// 		rating: 4,
// 		num: 158,
// 	},

pub mod prankster {
    use super::*;

    /// onModifyPriority(...)
    pub fn on_modify_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PRESSURE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	pressure: {
// 		onStart(pokemon) {
// 			this.add('-ability', pokemon, 'Pressure');
// 		},
// 		onDeductPP(target, source) {
// 			if (target.isAlly(source)) return;
// 			return 1;
// 		},
// 		flags: {},
// 		name: "Pressure",
// 		rating: 2.5,
// 		num: 46,
// 	},

pub mod pressure {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onDeductPP(...)
    pub fn on_deduct_p_p(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PRIMORDIALSEA
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	primordialsea: {
// 		onStart(source) {
// 			this.field.setWeather('primordialsea');
// 		},
// 		onAnySetWeather(target, source, weather) {
// 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
// 			if (this.field.getWeather().id === 'primordialsea' && !strongWeathers.includes(weather.id)) return false;
// 		},
// 		onEnd(pokemon) {
// 			if (this.field.weatherState.source !== pokemon) return;
// 			for (const target of this.getAllActive()) {
// 				if (target === pokemon) continue;
// 				if (target.hasAbility('primordialsea')) {
// 					this.field.weatherState.source = target;
// 					return;
// 				}
// 			}
// 			this.field.clearWeather();
// 		},
// 		flags: {},
// 		name: "Primordial Sea",
// 		rating: 4.5,
// 		num: 189,
// 	},

pub mod primordialsea {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnySetWeather(...)
    pub fn on_any_set_weather(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PRISMARMOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	prismarmor: {
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (target.getMoveHitData(move).typeMod > 0) {
// 				this.debug('Prism Armor neutralize');
// 				return this.chainModify(0.75);
// 			}
// 		},
// 		flags: {},
// 		name: "Prism Armor",
// 		rating: 3,
// 		num: 232,
// 	},

pub mod prismarmor {
    use super::*;

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PROPELLERTAIL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	propellertail: {
// 		onModifyMovePriority: 1,
// 		onModifyMove(move) {
// 			// most of the implementation is in Battle#getTarget
// 			move.tracksTarget = move.target !== 'scripted';
// 		},
// 		flags: {},
// 		name: "Propeller Tail",
// 		rating: 0,
// 		num: 239,
// 	},

pub mod propellertail {
    use super::*;

    /// onModifyMovePriority(...)
    pub fn on_modify_move_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PROTEAN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	protean: {
// 		onPrepareHit(source, target, move) {
// 			if (this.effectState.protean) return;
// 			if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
// 			const type = move.type;
// 			if (type && type !== '???' && source.getTypes().join() !== type) {
// 				if (!source.setType(type)) return;
// 				this.effectState.protean = true;
// 				this.add('-start', source, 'typechange', type, '[from] ability: Protean');
// 			}
// 		},
// 		flags: {},
// 		name: "Protean",
// 		rating: 4,
// 		num: 168,
// 	},

pub mod protean {
    use super::*;

    /// onPrepareHit(...)
    pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PROTOSYNTHESIS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	protosynthesis: {
// 		onSwitchInPriority: -2,
// 		onStart(pokemon) {
// 			this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
// 		},
// 		onWeatherChange(pokemon) {
// 			// Protosynthesis is not affected by Utility Umbrella
// 			if (this.field.isWeather('sunnyday')) {
// 				pokemon.addVolatile('protosynthesis');
// 			} else if (!pokemon.volatiles['protosynthesis']?.fromBooster && !this.field.isWeather('sunnyday')) {
// 				pokemon.removeVolatile('protosynthesis');
// 			}
// 		},
// 		onEnd(pokemon) {
// 			delete pokemon.volatiles['protosynthesis'];
// 			this.add('-end', pokemon, 'Protosynthesis', '[silent]');
// 		},
// 		condition: {
// 			noCopy: true,
// 			onStart(pokemon, source, effect) {
// 				if (effect?.name === 'Booster Energy') {
// 					this.effectState.fromBooster = true;
// 					this.add('-activate', pokemon, 'ability: Protosynthesis', '[fromitem]');
// 				} else {
// 					this.add('-activate', pokemon, 'ability: Protosynthesis');
// 				}
// 				this.effectState.bestStat = pokemon.getBestStat(false, true);
// 				this.add('-start', pokemon, 'protosynthesis' + this.effectState.bestStat);
// 			},
// 			onModifyAtkPriority: 5,
// 			onModifyAtk(atk, pokemon) {
// 				if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
// 				this.debug('Protosynthesis atk boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifyDefPriority: 6,
// 			onModifyDef(def, pokemon) {
// 				if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
// 				this.debug('Protosynthesis def boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpAPriority: 5,
// 			onModifySpA(spa, pokemon) {
// 				if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
// 				this.debug('Protosynthesis spa boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpDPriority: 6,
// 			onModifySpD(spd, pokemon) {
// 				if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
// 				this.debug('Protosynthesis spd boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpe(spe, pokemon) {
// 				if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
// 				this.debug('Protosynthesis spe boost');
// 				return this.chainModify(1.5);
// 			},
// 			onEnd(pokemon) {
// 				this.add('-end', pokemon, 'Protosynthesis');
// 			},
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
// 		name: "Protosynthesis",
// 		rating: 3,
// 		num: 281,
// 	},

pub mod protosynthesis {
    use super::*;

    /// onSwitchInPriority(...)
    pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onWeatherChange(...)
    pub fn on_weather_change(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyDefPriority(...)
    pub fn on_modify_def_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyDef(...)
    pub fn on_modify_def(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpAPriority(...)
    pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(...)
    pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpDPriority(...)
    pub fn on_modify_sp_d_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpD(...)
    pub fn on_modify_sp_d(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    // Condition handlers
    pub mod condition {
        use super::*;

        // TODO: Implement condition handlers
    }
}

// -----------------------------------------------------------------------------
// PSYCHICSURGE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	psychicsurge: {
// 		onStart(source) {
// 			this.field.setTerrain('psychicterrain');
// 		},
// 		flags: {},
// 		name: "Psychic Surge",
// 		rating: 4,
// 		num: 227,
// 	},

pub mod psychicsurge {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PUNKROCK
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	punkrock: {
// 		onBasePowerPriority: 7,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (move.flags['sound']) {
// 				this.debug('Punk Rock boost');
// 				return this.chainModify([5325, 4096]);
// 			}
// 		},
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (move.flags['sound']) {
// 				this.debug('Punk Rock weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Punk Rock",
// 		rating: 3.5,
// 		num: 244,
// 	},

pub mod punkrock {
    use super::*;

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PUREPOWER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	purepower: {
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk) {
// 			return this.chainModify(2);
// 		},
// 		flags: {},
// 		name: "Pure Power",
// 		rating: 5,
// 		num: 74,
// 	},

pub mod purepower {
    use super::*;

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PURIFYINGSALT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	purifyingsalt: {
// 		onSetStatus(status, target, source, effect) {
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Purifying Salt');
// 			}
// 			return false;
// 		},
// 		onTryAddVolatile(status, target) {
// 			if (status.id === 'yawn') {
// 				this.add('-immune', target, '[from] ability: Purifying Salt');
// 				return null;
// 			}
// 		},
// 		onSourceModifyAtkPriority: 6,
// 		onSourceModifyAtk(atk, attacker, defender, move) {
// 			if (move.type === 'Ghost') {
// 				this.debug('Purifying Salt weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		onSourceModifySpAPriority: 5,
// 		onSourceModifySpA(spa, attacker, defender, move) {
// 			if (move.type === 'Ghost') {
// 				this.debug('Purifying Salt weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Purifying Salt",
// 		rating: 4,
// 		num: 272,
// 	},

pub mod purifyingsalt {
    use super::*;

    /// onSetStatus(...)
    pub fn on_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryAddVolatile(...)
    pub fn on_try_add_volatile(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyAtkPriority(...)
    pub fn on_source_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyAtk(...)
    pub fn on_source_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifySpAPriority(...)
    pub fn on_source_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifySpA(...)
    pub fn on_source_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// QUARKDRIVE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	quarkdrive: {
// 		onSwitchInPriority: -2,
// 		onStart(pokemon) {
// 			this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
// 		},
// 		onTerrainChange(pokemon) {
// 			if (this.field.isTerrain('electricterrain')) {
// 				pokemon.addVolatile('quarkdrive');
// 			} else if (!pokemon.volatiles['quarkdrive']?.fromBooster) {
// 				pokemon.removeVolatile('quarkdrive');
// 			}
// 		},
// 		onEnd(pokemon) {
// 			delete pokemon.volatiles['quarkdrive'];
// 			this.add('-end', pokemon, 'Quark Drive', '[silent]');
// 		},
// 		condition: {
// 			noCopy: true,
// 			onStart(pokemon, source, effect) {
// 				if (effect?.name === 'Booster Energy') {
// 					this.effectState.fromBooster = true;
// 					this.add('-activate', pokemon, 'ability: Quark Drive', '[fromitem]');
// 				} else {
// 					this.add('-activate', pokemon, 'ability: Quark Drive');
// 				}
// 				this.effectState.bestStat = pokemon.getBestStat(false, true);
// 				this.add('-start', pokemon, 'quarkdrive' + this.effectState.bestStat);
// 			},
// 			onModifyAtkPriority: 5,
// 			onModifyAtk(atk, pokemon) {
// 				if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
// 				this.debug('Quark Drive atk boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifyDefPriority: 6,
// 			onModifyDef(def, pokemon) {
// 				if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
// 				this.debug('Quark Drive def boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpAPriority: 5,
// 			onModifySpA(spa, pokemon) {
// 				if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
// 				this.debug('Quark Drive spa boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpDPriority: 6,
// 			onModifySpD(spd, pokemon) {
// 				if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
// 				this.debug('Quark Drive spd boost');
// 				return this.chainModify([5325, 4096]);
// 			},
// 			onModifySpe(spe, pokemon) {
// 				if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
// 				this.debug('Quark Drive spe boost');
// 				return this.chainModify(1.5);
// 			},
// 			onEnd(pokemon) {
// 				this.add('-end', pokemon, 'Quark Drive');
// 			},
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
// 		name: "Quark Drive",
// 		rating: 3,
// 		num: 282,
// 	},

pub mod quarkdrive {
    use super::*;

    /// onSwitchInPriority(...)
    pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTerrainChange(...)
    pub fn on_terrain_change(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyDefPriority(...)
    pub fn on_modify_def_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyDef(...)
    pub fn on_modify_def(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpAPriority(...)
    pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(...)
    pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpDPriority(...)
    pub fn on_modify_sp_d_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpD(...)
    pub fn on_modify_sp_d(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    // Condition handlers
    pub mod condition {
        use super::*;

        // TODO: Implement condition handlers
    }
}

// -----------------------------------------------------------------------------
// QUEENLYMAJESTY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	queenlymajesty: {
// 		onFoeTryMove(target, source, move) {
// 			const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
// 			if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
// 				return;
// 			}
// 
// 			const dazzlingHolder = this.effectState.target;
// 			if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1) {
// 				this.attrLastMove('[still]');
// 				this.add('cant', dazzlingHolder, 'ability: Queenly Majesty', move, `[of] ${target}`);
// 				return false;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Queenly Majesty",
// 		rating: 2.5,
// 		num: 214,
// 	},

pub mod queenlymajesty {
    use super::*;

    /// onFoeTryMove(...)
    pub fn on_foe_try_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// QUICKDRAW
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	quickdraw: {
// 		onFractionalPriorityPriority: -1,
// 		onFractionalPriority(priority, pokemon, target, move) {
// 			if (move.category !== "Status" && this.randomChance(3, 10)) {
// 				this.add('-activate', pokemon, 'ability: Quick Draw');
// 				return 0.1;
// 			}
// 		},
// 		flags: {},
// 		name: "Quick Draw",
// 		rating: 2.5,
// 		num: 259,
// 	},

pub mod quickdraw {
    use super::*;

    /// onFractionalPriorityPriority(...)
    pub fn on_fractional_priority_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onFractionalPriority(...)
    pub fn on_fractional_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// QUICKFEET
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	quickfeet: {
// 		onModifySpe(spe, pokemon) {
// 			if (pokemon.status) {
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Quick Feet",
// 		rating: 2.5,
// 		num: 95,
// 	},

pub mod quickfeet {
    use super::*;

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RAINDISH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	raindish: {
// 		onWeather(target, source, effect) {
// 			if (target.hasItem('utilityumbrella')) return;
// 			if (effect.id === 'raindance' || effect.id === 'primordialsea') {
// 				this.heal(target.baseMaxhp / 16);
// 			}
// 		},
// 		flags: {},
// 		name: "Rain Dish",
// 		rating: 1.5,
// 		num: 44,
// 	},

pub mod raindish {
    use super::*;

    /// onWeather(...)
    pub fn on_weather(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RATTLED
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	rattled: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (['Dark', 'Bug', 'Ghost'].includes(move.type)) {
// 				this.boost({ spe: 1 });
// 			}
// 		},
// 		onAfterBoost(boost, target, source, effect) {
// 			if (effect?.name === 'Intimidate' && boost.atk) {
// 				this.boost({ spe: 1 });
// 			}
// 		},
// 		flags: {},
// 		name: "Rattled",
// 		rating: 1,
// 		num: 155,
// 	},

pub mod rattled {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAfterBoost(...)
    pub fn on_after_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// REBOUND
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	rebound: {
// 		isNonstandard: "CAP",
// 		onTryHitPriority: 1,
// 		onTryHit(target, source, move) {
// 			if (this.effectState.target.activeTurns) return;
// 
// 			if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
// 				return;
// 			}
// 			const newMove = this.dex.getActiveMove(move.id);
// 			newMove.hasBounced = true;
// 			newMove.pranksterBoosted = false;
// 			this.actions.useMove(newMove, target, { target: source });
// 			return null;
// 		},
// 		onAllyTryHitSide(target, source, move) {
// 			if (this.effectState.target.activeTurns) return;
// 
// 			if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
// 				return;
// 			}
// 			const newMove = this.dex.getActiveMove(move.id);
// 			newMove.hasBounced = true;
// 			newMove.pranksterBoosted = false;
// 			this.actions.useMove(newMove, this.effectState.target, { target: source });
// 			move.hasBounced = true; // only bounce once in free-for-all battles
// 			return null;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Rebound",
// 		rating: 3,
// 		num: -2,
// 	},

pub mod rebound {
    use super::*;

    /// onTryHitPriority(...)
    pub fn on_try_hit_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAllyTryHitSide(...)
    pub fn on_ally_try_hit_side(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RECEIVER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	receiver: {
// 		onAllyFaint(target) {
// 			if (!this.effectState.target.hp) return;
// 			const ability = target.getAbility();
// 			if (ability.flags['noreceiver'] || ability.id === 'noability') return;
// 			this.effectState.target.setAbility(ability, target);
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
// 		name: "Receiver",
// 		rating: 0,
// 		num: 222,
// 	},

pub mod receiver {
    use super::*;

    /// onAllyFaint(...)
    pub fn on_ally_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RECKLESS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	reckless: {
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (move.recoil || move.hasCrashDamage) {
// 				this.debug('Reckless boost');
// 				return this.chainModify([4915, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Reckless",
// 		rating: 3,
// 		num: 120,
// 	},

pub mod reckless {
    use super::*;

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// REFRIGERATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	refrigerate: {
// 		onModifyTypePriority: -1,
// 		onModifyType(move, pokemon) {
// 			const noModifyType = [
// 				'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
// 			];
// 			if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
// 				!(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
// 				move.type = 'Ice';
// 				move.typeChangerBoosted = this.effect;
// 			}
// 		},
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, pokemon, target, move) {
// 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
// 		},
// 		flags: {},
// 		name: "Refrigerate",
// 		rating: 4,
// 		num: 174,
// 	},

pub mod refrigerate {
    use super::*;

    /// onModifyTypePriority(...)
    pub fn on_modify_type_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyType(...)
    pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// REGENERATOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	regenerator: {
// 		onSwitchOut(pokemon) {
// 			pokemon.heal(pokemon.baseMaxhp / 3);
// 		},
// 		flags: {},
// 		name: "Regenerator",
// 		rating: 4.5,
// 		num: 144,
// 	},

pub mod regenerator {
    use super::*;

    /// onSwitchOut(...)
    pub fn on_switch_out(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RIPEN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	ripen: {
// 		onTryHeal(damage, target, source, effect) {
// 			if (!effect) return;
// 			if (effect.name === 'Berry Juice' || effect.name === 'Leftovers') {
// 				this.add('-activate', target, 'ability: Ripen');
// 			}
// 			if ((effect as Item).isBerry) return this.chainModify(2);
// 		},
// 		onChangeBoost(boost, target, source, effect) {
// 			if (effect && (effect as Item).isBerry) {
// 				let b: BoostID;
// 				for (b in boost) {
// 					boost[b]! *= 2;
// 				}
// 			}
// 		},
// 		onSourceModifyDamagePriority: -1,
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (target.abilityState.berryWeaken) {
// 				target.abilityState.berryWeaken = false;
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		onTryEatItemPriority: -1,
// 		onTryEatItem(item, pokemon) {
// 			this.add('-activate', pokemon, 'ability: Ripen');
// 		},
// 		onEatItem(item, pokemon) {
// 			const weakenBerries = [
// 				'Babiri Berry', 'Charti Berry', 'Chilan Berry', 'Chople Berry', 'Coba Berry', 'Colbur Berry', 'Haban Berry', 'Kasib Berry', 'Kebia Berry', 'Occa Berry', 'Passho Berry', 'Payapa Berry', 'Rindo Berry', 'Roseli Berry', 'Shuca Berry', 'Tanga Berry', 'Wacan Berry', 'Yache Berry',
// 			];
// 			// Record if the pokemon ate a berry to resist the attack
// 			pokemon.abilityState.berryWeaken = weakenBerries.includes(item.name);
// 		},
// 		flags: {},
// 		name: "Ripen",
// 		rating: 2,
// 		num: 247,
// 	},

pub mod ripen {
    use super::*;

    /// onTryHeal(...)
    pub fn on_try_heal(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onChangeBoost(...)
    pub fn on_change_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyDamagePriority(...)
    pub fn on_source_modify_damage_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryEatItemPriority(...)
    pub fn on_try_eat_item_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryEatItem(...)
    pub fn on_try_eat_item(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEatItem(...)
    pub fn on_eat_item(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RIVALRY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	rivalry: {
// 		onBasePowerPriority: 24,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (attacker.gender && defender.gender) {
// 				if (attacker.gender === defender.gender) {
// 					this.debug('Rivalry boost');
// 					return this.chainModify(1.25);
// 				} else {
// 					this.debug('Rivalry weaken');
// 					return this.chainModify(0.75);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Rivalry",
// 		rating: 0,
// 		num: 79,
// 	},

pub mod rivalry {
    use super::*;

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ROCKHEAD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	rockhead: {
// 		onDamage(damage, target, source, effect) {
// 			if (effect.id === 'recoil') {
// 				if (!this.activeMove) throw new Error("Battle.activeMove is null");
// 				if (this.activeMove.id !== 'struggle') return null;
// 			}
// 		},
// 		flags: {},
// 		name: "Rock Head",
// 		rating: 3,
// 		num: 69,
// 	},

pub mod rockhead {
    use super::*;

    /// onDamage(...)
    pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ROCKYPAYLOAD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	rockypayload: {
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk, attacker, defender, move) {
// 			if (move.type === 'Rock') {
// 				this.debug('Rocky Payload boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		onModifySpAPriority: 5,
// 		onModifySpA(atk, attacker, defender, move) {
// 			if (move.type === 'Rock') {
// 				this.debug('Rocky Payload boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Rocky Payload",
// 		rating: 3.5,
// 		num: 276,
// 	},

pub mod rockypayload {
    use super::*;

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpAPriority(...)
    pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(...)
    pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ROUGHSKIN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	roughskin: {
// 		onDamagingHitOrder: 1,
// 		onDamagingHit(damage, target, source, move) {
// 			if (this.checkMoveMakesContact(move, source, target, true)) {
// 				this.damage(source.baseMaxhp / 8, source, target);
// 			}
// 		},
// 		flags: {},
// 		name: "Rough Skin",
// 		rating: 2.5,
// 		num: 24,
// 	},

pub mod roughskin {
    use super::*;

    /// onDamagingHitOrder(...)
    pub fn on_damaging_hit_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SANDFORCE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sandforce: {
// 		onBasePowerPriority: 21,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (this.field.isWeather('sandstorm')) {
// 				if (move.type === 'Rock' || move.type === 'Ground' || move.type === 'Steel') {
// 					this.debug('Sand Force boost');
// 					return this.chainModify([5325, 4096]);
// 				}
// 			}
// 		},
// 		onImmunity(type, pokemon) {
// 			if (type === 'sandstorm') return false;
// 		},
// 		flags: {},
// 		name: "Sand Force",
// 		rating: 2,
// 		num: 159,
// 	},

pub mod sandforce {
    use super::*;

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SANDRUSH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sandrush: {
// 		onModifySpe(spe, pokemon) {
// 			if (this.field.isWeather('sandstorm')) {
// 				return this.chainModify(2);
// 			}
// 		},
// 		onImmunity(type, pokemon) {
// 			if (type === 'sandstorm') return false;
// 		},
// 		flags: {},
// 		name: "Sand Rush",
// 		rating: 3,
// 		num: 146,
// 	},

pub mod sandrush {
    use super::*;

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SANDSPIT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sandspit: {
// 		onDamagingHit(damage, target, source, move) {
// 			this.field.setWeather('sandstorm');
// 		},
// 		flags: {},
// 		name: "Sand Spit",
// 		rating: 1,
// 		num: 245,
// 	},

pub mod sandspit {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SANDSTREAM
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sandstream: {
// 		onStart(source) {
// 			this.field.setWeather('sandstorm');
// 		},
// 		flags: {},
// 		name: "Sand Stream",
// 		rating: 4,
// 		num: 45,
// 	},

pub mod sandstream {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SANDVEIL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sandveil: {
// 		onImmunity(type, pokemon) {
// 			if (type === 'sandstorm') return false;
// 		},
// 		onModifyAccuracyPriority: -1,
// 		onModifyAccuracy(accuracy) {
// 			if (typeof accuracy !== 'number') return;
// 			if (this.field.isWeather('sandstorm')) {
// 				this.debug('Sand Veil - decreasing accuracy');
// 				return this.chainModify([3277, 4096]);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Sand Veil",
// 		rating: 1.5,
// 		num: 8,
// 	},

pub mod sandveil {
    use super::*;

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAccuracyPriority(...)
    pub fn on_modify_accuracy_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAccuracy(...)
    pub fn on_modify_accuracy(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SAPSIPPER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sapsipper: {
// 		onTryHitPriority: 1,
// 		onTryHit(target, source, move) {
// 			if (target !== source && move.type === 'Grass') {
// 				if (!this.boost({ atk: 1 })) {
// 					this.add('-immune', target, '[from] ability: Sap Sipper');
// 				}
// 				return null;
// 			}
// 		},
// 		onAllyTryHitSide(target, source, move) {
// 			if (source === this.effectState.target || !target.isAlly(source)) return;
// 			if (move.type === 'Grass') {
// 				this.boost({ atk: 1 }, this.effectState.target);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Sap Sipper",
// 		rating: 3,
// 		num: 157,
// 	},

pub mod sapsipper {
    use super::*;

    /// onTryHitPriority(...)
    pub fn on_try_hit_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAllyTryHitSide(...)
    pub fn on_ally_try_hit_side(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SCHOOLING
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	schooling: {
// 		onSwitchInPriority: -1,
// 		onStart(pokemon) {
// 			if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed) return;
// 			if (pokemon.hp > pokemon.maxhp / 4) {
// 				if (pokemon.species.id === 'wishiwashi') {
// 					pokemon.formeChange('Wishiwashi-School');
// 				}
// 			} else {
// 				if (pokemon.species.id === 'wishiwashischool') {
// 					pokemon.formeChange('Wishiwashi');
// 				}
// 			}
// 		},
// 		onResidualOrder: 29,
// 		onResidual(pokemon) {
// 			if (
// 				pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 ||
// 				pokemon.transformed || !pokemon.hp
// 			) return;
// 			if (pokemon.hp > pokemon.maxhp / 4) {
// 				if (pokemon.species.id === 'wishiwashi') {
// 					pokemon.formeChange('Wishiwashi-School');
// 				}
// 			} else {
// 				if (pokemon.species.id === 'wishiwashischool') {
// 					pokemon.formeChange('Wishiwashi');
// 				}
// 			}
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
// 		name: "Schooling",
// 		rating: 3,
// 		num: 208,
// 	},

pub mod schooling {
    use super::*;

    /// onSwitchInPriority(...)
    pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SCRAPPY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	scrappy: {
// 		onModifyMovePriority: -5,
// 		onModifyMove(move) {
// 			if (!move.ignoreImmunity) move.ignoreImmunity = {};
// 			if (move.ignoreImmunity !== true) {
// 				move.ignoreImmunity['Fighting'] = true;
// 				move.ignoreImmunity['Normal'] = true;
// 			}
// 		},
// 		onTryBoost(boost, target, source, effect) {
// 			if (effect.name === 'Intimidate' && boost.atk) {
// 				delete boost.atk;
// 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Scrappy', `[of] ${target}`);
// 			}
// 		},
// 		flags: {},
// 		name: "Scrappy",
// 		rating: 3,
// 		num: 113,
// 	},

pub mod scrappy {
    use super::*;

    /// onModifyMovePriority(...)
    pub fn on_modify_move_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SCREENCLEANER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	screencleaner: {
// 		onStart(pokemon) {
// 			let activated = false;
// 			for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil']) {
// 				for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()]) {
// 					if (side.getSideCondition(sideCondition)) {
// 						if (!activated) {
// 							this.add('-activate', pokemon, 'ability: Screen Cleaner');
// 							activated = true;
// 						}
// 						side.removeSideCondition(sideCondition);
// 					}
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Screen Cleaner",
// 		rating: 2,
// 		num: 251,
// 	},

pub mod screencleaner {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SEEDSOWER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	seedsower: {
// 		onDamagingHit(damage, target, source, move) {
// 			this.field.setTerrain('grassyterrain');
// 		},
// 		flags: {},
// 		name: "Seed Sower",
// 		rating: 2.5,
// 		num: 269,
// 	},

pub mod seedsower {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SERENEGRACE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	serenegrace: {
// 		onModifyMovePriority: -2,
// 		onModifyMove(move) {
// 			if (move.secondaries) {
// 				this.debug('doubling secondary chance');
// 				for (const secondary of move.secondaries) {
// 					if (secondary.chance) secondary.chance *= 2;
// 				}
// 			}
// 			if (move.self?.chance) move.self.chance *= 2;
// 		},
// 		flags: {},
// 		name: "Serene Grace",
// 		rating: 3.5,
// 		num: 32,
// 	},

pub mod serenegrace {
    use super::*;

    /// onModifyMovePriority(...)
    pub fn on_modify_move_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHADOWSHIELD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shadowshield: {
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (target.hp >= target.maxhp) {
// 				this.debug('Shadow Shield weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Shadow Shield",
// 		rating: 3.5,
// 		num: 231,
// 	},

pub mod shadowshield {
    use super::*;

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHADOWTAG
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shadowtag: {
// 		onFoeTrapPokemon(pokemon) {
// 			if (!pokemon.hasAbility('shadowtag') && pokemon.isAdjacent(this.effectState.target)) {
// 				pokemon.tryTrap(true);
// 			}
// 		},
// 		onFoeMaybeTrapPokemon(pokemon, source) {
// 			if (!source) source = this.effectState.target;
// 			if (!source || !pokemon.isAdjacent(source)) return;
// 			if (!pokemon.hasAbility('shadowtag')) {
// 				pokemon.maybeTrapped = true;
// 			}
// 		},
// 		flags: {},
// 		name: "Shadow Tag",
// 		rating: 5,
// 		num: 23,
// 	},

pub mod shadowtag {
    use super::*;

    /// onFoeTrapPokemon(...)
    pub fn on_foe_trap_pokemon(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onFoeMaybeTrapPokemon(...)
    pub fn on_foe_maybe_trap_pokemon(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHARPNESS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sharpness: {
// 		onBasePowerPriority: 19,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (move.flags['slicing']) {
// 				this.debug('Sharpness boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Sharpness",
// 		rating: 3.5,
// 		num: 292,
// 	},

pub mod sharpness {
    use super::*;

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHEDSKIN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shedskin: {
// 		onResidualOrder: 5,
// 		onResidualSubOrder: 3,
// 		onResidual(pokemon) {
// 			if (pokemon.hp && pokemon.status && this.randomChance(33, 100)) {
// 				this.debug('shed skin');
// 				this.add('-activate', pokemon, 'ability: Shed Skin');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		flags: {},
// 		name: "Shed Skin",
// 		rating: 3,
// 		num: 61,
// 	},

pub mod shedskin {
    use super::*;

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualSubOrder(...)
    pub fn on_residual_sub_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHEERFORCE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sheerforce: {
// 		onModifyMove(move, pokemon) {
// 			if (move.secondaries) {
// 				delete move.secondaries;
// 				// Technically not a secondary effect, but it is negated
// 				delete move.self;
// 				if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
// 				// Actual negation of `AfterMoveSecondary` effects implemented in scripts.js
// 				move.hasSheerForce = true;
// 			}
// 		},
// 		onBasePowerPriority: 21,
// 		onBasePower(basePower, pokemon, target, move) {
// 			if (move.hasSheerForce) return this.chainModify([5325, 4096]);
// 		},
// 		flags: {},
// 		name: "Sheer Force",
// 		rating: 3.5,
// 		num: 125,
// 	},

pub mod sheerforce {
    use super::*;

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePowerPriority(...)
    pub fn on_base_power_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBasePower(...)
    pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHELLARMOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shellarmor: {
// 		onCriticalHit: false,
// 		flags: { breakable: 1 },
// 		name: "Shell Armor",
// 		rating: 1,
// 		num: 75,
// 	},

pub mod shellarmor {
    use super::*;

    /// onCriticalHit(...)
    pub fn on_critical_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHIELDDUST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shielddust: {
// 		onModifySecondaries(secondaries) {
// 			this.debug('Shield Dust prevent secondary');
// 			return secondaries.filter(effect => !!effect.self);
// 		},
// 		flags: { breakable: 1 },
// 		name: "Shield Dust",
// 		rating: 2,
// 		num: 19,
// 	},

pub mod shielddust {
    use super::*;

    /// onModifySecondaries(...)
    pub fn on_modify_secondaries(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHIELDSDOWN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	shieldsdown: {
// 		onSwitchInPriority: -1,
// 		onStart(pokemon) {
// 			if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed) return;
// 			if (pokemon.hp > pokemon.maxhp / 2) {
// 				if (pokemon.species.forme !== 'Meteor') {
// 					pokemon.formeChange('Minior-Meteor');
// 				}
// 			} else {
// 				if (pokemon.species.forme === 'Meteor') {
// 					pokemon.formeChange(pokemon.set.species);
// 				}
// 			}
// 		},
// 		onResidualOrder: 29,
// 		onResidual(pokemon) {
// 			if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed || !pokemon.hp) return;
// 			if (pokemon.hp > pokemon.maxhp / 2) {
// 				if (pokemon.species.forme !== 'Meteor') {
// 					pokemon.formeChange('Minior-Meteor');
// 				}
// 			} else {
// 				if (pokemon.species.forme === 'Meteor') {
// 					pokemon.formeChange(pokemon.set.species);
// 				}
// 			}
// 		},
// 		onSetStatus(status, target, source, effect) {
// 			if (target.species.id !== 'miniormeteor' || target.transformed) return;
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Shields Down');
// 			}
// 			return false;
// 		},
// 		onTryAddVolatile(status, target) {
// 			if (target.species.id !== 'miniormeteor' || target.transformed) return;
// 			if (status.id !== 'yawn') return;
// 			this.add('-immune', target, '[from] ability: Shields Down');
// 			return null;
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
// 		name: "Shields Down",
// 		rating: 3,
// 		num: 197,
// 	},

pub mod shieldsdown {
    use super::*;

    /// onSwitchInPriority(...)
    pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(...)
    pub fn on_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryAddVolatile(...)
    pub fn on_try_add_volatile(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SIMPLE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	simple: {
// 		onChangeBoost(boost, target, source, effect) {
// 			if (effect && effect.id === 'zpower') return;
// 			let i: BoostID;
// 			for (i in boost) {
// 				boost[i]! *= 2;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Simple",
// 		rating: 4,
// 		num: 86,
// 	},

pub mod simple {
    use super::*;

    /// onChangeBoost(...)
    pub fn on_change_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SKILLLINK
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	skilllink: {
// 		onModifyMove(move) {
// 			if (move.multihit && Array.isArray(move.multihit) && move.multihit.length) {
// 				move.multihit = move.multihit[1];
// 			}
// 			if (move.multiaccuracy) {
// 				delete move.multiaccuracy;
// 			}
// 		},
// 		flags: {},
// 		name: "Skill Link",
// 		rating: 3,
// 		num: 92,
// 	},

pub mod skilllink {
    use super::*;

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SLOWSTART
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	slowstart: {
// 		onStart(pokemon) {
// 			this.add('-start', pokemon, 'ability: Slow Start');
// 			this.effectState.counter = 5;
// 		},
// 		onResidualOrder: 28,
// 		onResidualSubOrder: 2,
// 		onResidual(pokemon) {
// 			if (pokemon.activeTurns && this.effectState.counter) {
// 				this.effectState.counter--;
// 				if (!this.effectState.counter) {
// 					this.add('-end', pokemon, 'Slow Start');
// 					delete this.effectState.counter;
// 				}
// 			}
// 		},
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk, pokemon) {
// 			if (this.effectState.counter) {
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		onModifySpe(spe, pokemon) {
// 			if (this.effectState.counter) {
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Slow Start",
// 		rating: -1,
// 		num: 112,
// 	},

pub mod slowstart {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualOrder(...)
    pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidualSubOrder(...)
    pub fn on_residual_sub_order(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onResidual(...)
    pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtkPriority(...)
    pub fn on_modify_atk_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAtk(...)
    pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SLUSHRUSH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	slushrush: {
// 		onModifySpe(spe, pokemon) {
// 			if (this.field.isWeather(['hail', 'snowscape'])) {
// 				return this.chainModify(2);
// 			}
// 		},
// 		flags: {},
// 		name: "Slush Rush",
// 		rating: 3,
// 		num: 202,
// 	},

pub mod slushrush {
    use super::*;

    /// onModifySpe(...)
    pub fn on_modify_spe(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SNIPER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	sniper: {
// 		onModifyDamage(damage, source, target, move) {
// 			if (target.getMoveHitData(move).crit) {
// 				this.debug('Sniper boost');
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Sniper",
// 		rating: 2,
// 		num: 97,
// 	},

pub mod sniper {
    use super::*;

    /// onModifyDamage(...)
    pub fn on_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SNOWCLOAK
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	snowcloak: {
// 		onImmunity(type, pokemon) {
// 			if (type === 'hail') return false;
// 		},
// 		onModifyAccuracyPriority: -1,
// 		onModifyAccuracy(accuracy) {
// 			if (typeof accuracy !== 'number') return;
// 			if (this.field.isWeather(['hail', 'snowscape'])) {
// 				this.debug('Snow Cloak - decreasing accuracy');
// 				return this.chainModify([3277, 4096]);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Snow Cloak",
// 		rating: 1.5,
// 		num: 81,
// 	},

pub mod snowcloak {
    use super::*;

    /// onImmunity(...)
    pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAccuracyPriority(...)
    pub fn on_modify_accuracy_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyAccuracy(...)
    pub fn on_modify_accuracy(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}
