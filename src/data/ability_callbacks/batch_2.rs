//! Ability Callback Handlers - Batch 2
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file contains ability callback implementations for batch 2.
//! Generated from data/abilities.ts
//!
//! Abilities in this batch (79):
//! - frisk
//! - fullmetalbody
//! - furcoat
//! - galewings
//! - galvanize
//! - gluttony
//! - goodasgold
//! - gooey
//! - gorillatactics
//! - grasspelt
//! - grassysurge
//! - grimneigh
//! - guarddog
//! - gulpmissile
//! - guts
//! - hadronengine
//! - harvest
//! - healer
//! - heatproof
//! - heavymetal
//! - honeygather
//! - hospitality
//! - hugepower
//! - hungerswitch
//! - hustle
//! - hydration
//! - hypercutter
//! - icebody
//! - iceface
//! - icescales
//! - illuminate
//! - illusion
//! - immunity
//! - imposter
//! - infiltrator
//! - innardsout
//! - innerfocus
//! - insomnia
//! - intimidate
//! - intrepidsword
//! - ironbarbs
//! - ironfist
//! - justified
//! - keeneye
//! - klutz
//! - leafguard
//! - levitate
//! - libero
//! - lightmetal
//! - lightningrod
//! - limber
//! - lingeringaroma
//! - liquidooze
//! - liquidvoice
//! - longreach
//! - magicbounce
//! - magicguard
//! - magician
//! - magmaarmor
//! - magnetpull
//! - marvelscale
//! - megalauncher
//! - merciless
//! - mimicry
//! - mindseye
//! - minus
//! - mirrorarmor
//! - mistysurge
//! - moldbreaker
//! - moody
//! - motordrive
//! - mountaineer
//! - moxie
//! - multiscale
//! - multitype
//! - mummy
//! - myceliummight
//! - naturalcure
//! - neuroforce

use crate::battle::{Battle, Arg};
use crate::data::moves::MoveDef;
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};


// -----------------------------------------------------------------------------
// FRISK
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	frisk: {
// 		onStart(pokemon) {
// 			for (const target of pokemon.foes()) {
// 				if (target.item) {
// 					this.add('-item', target, target.getItem().name, '[from] ability: Frisk', `[of] ${pokemon}`);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Frisk",
// 		rating: 1.5,
// 		num: 119,
// 	},

pub mod frisk {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FULLMETALBODY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	fullmetalbody: {
// 		onTryBoost(boost, target, source, effect) {
// 			if (source && target === source) return;
// 			let showMsg = false;
// 			let i: BoostID;
// 			for (i in boost) {
// 				if (boost[i]! < 0) {
// 					delete boost[i];
// 					showMsg = true;
// 				}
// 			}
// 			if (showMsg && !(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
// 				this.add("-fail", target, "unboost", "[from] ability: Full Metal Body", `[of] ${target}`);
// 			}
// 		},
// 		flags: {},
// 		name: "Full Metal Body",
// 		rating: 2,
// 		num: 230,
// 	},

pub mod fullmetalbody {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FURCOAT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	furcoat: {
// 		onModifyDefPriority: 6,
// 		onModifyDef(def) {
// 			return this.chainModify(2);
// 		},
// 		flags: { breakable: 1 },
// 		name: "Fur Coat",
// 		rating: 4,
// 		num: 169,
// 	},

pub mod furcoat {
    use super::*;

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
}

// -----------------------------------------------------------------------------
// GALEWINGS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	galewings: {
// 		onModifyPriority(priority, pokemon, target, move) {
// 			if (move?.type === 'Flying' && pokemon.hp === pokemon.maxhp) return priority + 1;
// 		},
// 		flags: {},
// 		name: "Gale Wings",
// 		rating: 1.5,
// 		num: 177,
// 	},

pub mod galewings {
    use super::*;

    /// onModifyPriority(...)
    pub fn on_modify_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GALVANIZE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	galvanize: {
// 		onModifyTypePriority: -1,
// 		onModifyType(move, pokemon) {
// 			const noModifyType = [
// 				'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
// 			];
// 			if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
// 				!(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
// 				move.type = 'Electric';
// 				move.typeChangerBoosted = this.effect;
// 			}
// 		},
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, pokemon, target, move) {
// 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
// 		},
// 		flags: {},
// 		name: "Galvanize",
// 		rating: 4,
// 		num: 206,
// 	},

pub mod galvanize {
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
// GLUTTONY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	gluttony: {
// 		onStart(pokemon) {
// 			pokemon.abilityState.gluttony = true;
// 		},
// 		onDamage(item, pokemon) {
// 			pokemon.abilityState.gluttony = true;
// 		},
// 		flags: {},
// 		name: "Gluttony",
// 		rating: 1.5,
// 		num: 82,
// 	},

pub mod gluttony {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// GOODASGOLD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	goodasgold: {
// 		onTryHit(target, source, move) {
// 			if (move.category === 'Status' && target !== source) {
// 				this.add('-immune', target, '[from] ability: Good as Gold');
// 				return null;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Good as Gold",
// 		rating: 5,
// 		num: 283,
// 	},

pub mod goodasgold {
    use super::*;

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GOOEY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	gooey: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (this.checkMoveMakesContact(move, source, target, true)) {
// 				this.add('-ability', target, 'Gooey');
// 				this.boost({ spe: -1 }, source, target, null, true);
// 			}
// 		},
// 		flags: {},
// 		name: "Gooey",
// 		rating: 2,
// 		num: 183,
// 	},

pub mod gooey {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GORILLATACTICS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	gorillatactics: {
// 		onStart(pokemon) {
// 			pokemon.abilityState.choiceLock = "";
// 		},
// 		onBeforeMove(pokemon, target, move) {
// 			if (move.isZOrMaxPowered || move.id === 'struggle') return;
// 			if (pokemon.abilityState.choiceLock && pokemon.abilityState.choiceLock !== move.id) {
// 				// Fails unless ability is being ignored (these events will not run), no PP lost.
// 				this.addMove('move', pokemon, move.name);
// 				this.attrLastMove('[still]');
// 				this.debug("Disabled by Gorilla Tactics");
// 				this.add('-fail', pokemon);
// 				return false;
// 			}
// 		},
// 		onModifyMove(move, pokemon) {
// 			if (pokemon.abilityState.choiceLock || move.isZOrMaxPowered || move.id === 'struggle') return;
// 			pokemon.abilityState.choiceLock = move.id;
// 		},
// 		onModifyAtkPriority: 1,
// 		onModifyAtk(atk, pokemon) {
// 			if (pokemon.volatiles['dynamax']) return;
// 			// PLACEHOLDER
// 			this.debug('Gorilla Tactics Atk Boost');
// 			return this.chainModify(1.5);
// 		},
// 		onDisableMove(pokemon) {
// 			if (!pokemon.abilityState.choiceLock) return;
// 			if (pokemon.volatiles['dynamax']) return;
// 			for (const moveSlot of pokemon.moveSlots) {
// 				if (moveSlot.id !== pokemon.abilityState.choiceLock) {
// 					pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
// 				}
// 			}
// 		},
// 		onEnd(pokemon) {
// 			pokemon.abilityState.choiceLock = "";
// 		},
// 		flags: {},
// 		name: "Gorilla Tactics",
// 		rating: 4.5,
// 		num: 255,
// 	},

pub mod gorillatactics {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onBeforeMove(...)
    pub fn on_before_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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

    /// onDisableMove(...)
    pub fn on_disable_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// GRASSPELT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	grasspelt: {
// 		onModifyDefPriority: 6,
// 		onModifyDef(pokemon) {
// 			if (this.field.isTerrain('grassyterrain')) return this.chainModify(1.5);
// 		},
// 		flags: { breakable: 1 },
// 		name: "Grass Pelt",
// 		rating: 0.5,
// 		num: 179,
// 	},

pub mod grasspelt {
    use super::*;

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
}

// -----------------------------------------------------------------------------
// GRASSYSURGE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	grassysurge: {
// 		onStart(source) {
// 			this.field.setTerrain('grassyterrain');
// 		},
// 		flags: {},
// 		name: "Grassy Surge",
// 		rating: 4,
// 		num: 229,
// 	},

pub mod grassysurge {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GRIMNEIGH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	grimneigh: {
// 		onSourceAfterFaint(length, target, source, effect) {
// 			if (effect && effect.effectType === 'Move') {
// 				this.boost({ spa: length }, source);
// 			}
// 		},
// 		flags: {},
// 		name: "Grim Neigh",
// 		rating: 3,
// 		num: 265,
// 	},

pub mod grimneigh {
    use super::*;

    /// onSourceAfterFaint(...)
    pub fn on_source_after_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GUARDDOG
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	guarddog: {
// 		onDragOutPriority: 1,
// 		onDragOut(pokemon) {
// 			this.add('-activate', pokemon, 'ability: Guard Dog');
// 			return null;
// 		},
// 		onTryBoostPriority: 2,
// 		onTryBoost(boost, target, source, effect) {
// 			if (effect.name === 'Intimidate' && boost.atk) {
// 				delete boost.atk;
// 				this.boost({ atk: 1 }, target, target, null, false, true);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Guard Dog",
// 		rating: 2,
// 		num: 275,
// 	},

pub mod guarddog {
    use super::*;

    /// onDragOutPriority(...)
    pub fn on_drag_out_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onDragOut(...)
    pub fn on_drag_out(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onTryBoostPriority(...)
    pub fn on_try_boost_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// GULPMISSILE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	gulpmissile: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (!source.hp || !source.isActive || target.isSemiInvulnerable()) return;
// 			if (['cramorantgulping', 'cramorantgorging'].includes(target.species.id)) {
// 				this.damage(source.baseMaxhp / 4, source, target);
// 				if (target.species.id === 'cramorantgulping') {
// 					this.boost({ def: -1 }, source, target, null, true);
// 				} else {
// 					source.trySetStatus('par', target, move);
// 				}
// 				target.formeChange('cramorant', move);
// 			}
// 		},
// 		// The Dive part of this mechanic is implemented in Dive's `onTryMove` in moves.ts
// 		onSourceTryPrimaryHit(target, source, effect) {
// 			if (effect?.id === 'surf' && source.hasAbility('gulpmissile') && source.species.name === 'Cramorant') {
// 				const forme = source.hp <= source.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
// 				source.formeChange(forme, effect);
// 			}
// 		},
// 		flags: { cantsuppress: 1, notransform: 1 },
// 		name: "Gulp Missile",
// 		rating: 2.5,
// 		num: 241,
// 	},

pub mod gulpmissile {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceTryPrimaryHit(...)
    pub fn on_source_try_primary_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GUTS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	guts: {
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk, pokemon) {
// 			if (pokemon.status) {
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Guts",
// 		rating: 3.5,
// 		num: 62,
// 	},

pub mod guts {
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
// HADRONENGINE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hadronengine: {
// 		onStart(pokemon) {
// 			if (!this.field.setTerrain('electricterrain') && this.field.isTerrain('electricterrain')) {
// 				this.add('-activate', pokemon, 'ability: Hadron Engine');
// 			}
// 		},
// 		onModifySpAPriority: 5,
// 		onModifySpA(atk, attacker, defender, move) {
// 			if (this.field.isTerrain('electricterrain')) {
// 				this.debug('Hadron Engine boost');
// 				return this.chainModify([5461, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Hadron Engine",
// 		rating: 4.5,
// 		num: 289,
// 	},

pub mod hadronengine {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// HARVEST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	harvest: {
// 		onResidualOrder: 28,
// 		onResidualSubOrder: 2,
// 		onResidual(pokemon) {
// 			if (this.field.isWeather(['sunnyday', 'desolateland']) || this.randomChance(1, 2)) {
// 				if (pokemon.hp && !pokemon.item && this.dex.items.get(pokemon.lastItem).isBerry) {
// 					pokemon.setItem(pokemon.lastItem);
// 					pokemon.lastItem = '';
// 					this.add('-item', pokemon, pokemon.getItem(), '[from] ability: Harvest');
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Harvest",
// 		rating: 2.5,
// 		num: 139,
// 	},

pub mod harvest {
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
// HEALER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	healer: {
// 		onResidualOrder: 5,
// 		onResidualSubOrder: 3,
// 		onResidual(pokemon) {
// 			for (const allyActive of pokemon.adjacentAllies()) {
// 				if (allyActive.status && this.randomChance(3, 10)) {
// 					this.add('-activate', pokemon, 'ability: Healer');
// 					allyActive.cureStatus();
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Healer",
// 		rating: 0,
// 		num: 131,
// 	},

pub mod healer {
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
// HEATPROOF
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	heatproof: {
// 		onSourceModifyAtkPriority: 6,
// 		onSourceModifyAtk(atk, attacker, defender, move) {
// 			if (move.type === 'Fire') {
// 				this.debug('Heatproof Atk weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		onSourceModifySpAPriority: 5,
// 		onSourceModifySpA(atk, attacker, defender, move) {
// 			if (move.type === 'Fire') {
// 				this.debug('Heatproof SpA weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		onDamage(damage, target, source, effect) {
// 			if (effect && effect.id === 'brn') {
// 				return damage / 2;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Heatproof",
// 		rating: 2,
// 		num: 85,
// 	},

pub mod heatproof {
    use super::*;

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

    /// onDamage(...)
    pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// HEAVYMETAL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	heavymetal: {
// 		onModifyWeightPriority: 1,
// 		onModifyWeight(weighthg) {
// 			return weighthg * 2;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Heavy Metal",
// 		rating: 0,
// 		num: 134,
// 	},

pub mod heavymetal {
    use super::*;

    /// onModifyWeightPriority(...)
    pub fn on_modify_weight_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onModifyWeight(...)
    pub fn on_modify_weight(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// HOSPITALITY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hospitality: {
// 		onSwitchInPriority: -2,
// 		onStart(pokemon) {
// 			for (const ally of pokemon.adjacentAllies()) {
// 				this.heal(ally.baseMaxhp / 4, ally, pokemon);
// 			}
// 		},
// 		flags: {},
// 		name: "Hospitality",
// 		rating: 0,
// 		num: 299,
// 	},

pub mod hospitality {
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
}

// -----------------------------------------------------------------------------
// HUGEPOWER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hugepower: {
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk) {
// 			return this.chainModify(2);
// 		},
// 		flags: {},
// 		name: "Huge Power",
// 		rating: 5,
// 		num: 37,
// 	},

pub mod hugepower {
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
// HUNGERSWITCH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hungerswitch: {
// 		onResidualOrder: 29,
// 		onResidual(pokemon) {
// 			if (pokemon.species.baseSpecies !== 'Morpeko' || pokemon.terastallized) return;
// 			const targetForme = pokemon.species.name === 'Morpeko' ? 'Morpeko-Hangry' : 'Morpeko';
// 			pokemon.formeChange(targetForme);
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
// 		name: "Hunger Switch",
// 		rating: 1,
// 		num: 258,
// 	},

pub mod hungerswitch {
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
// HUSTLE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hustle: {
// 		// This should be applied directly to the stat as opposed to chaining with the others
// 		onModifyAtkPriority: 5,
// 		onModifyAtk(atk) {
// 			return this.modify(atk, 1.5);
// 		},
// 		onSourceModifyAccuracyPriority: -1,
// 		onSourceModifyAccuracy(accuracy, target, source, move) {
// 			if (move.category === 'Physical' && typeof accuracy === 'number') {
// 				return this.chainModify([3277, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Hustle",
// 		rating: 3.5,
// 		num: 55,
// 	},

pub mod hustle {
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

    /// onSourceModifyAccuracyPriority(...)
    pub fn on_source_modify_accuracy_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSourceModifyAccuracy(...)
    pub fn on_source_modify_accuracy(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// HYDRATION
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hydration: {
// 		onResidualOrder: 5,
// 		onResidualSubOrder: 3,
// 		onResidual(pokemon) {
// 			if (pokemon.status && ['raindance', 'primordialsea'].includes(pokemon.effectiveWeather())) {
// 				this.debug('hydration');
// 				this.add('-activate', pokemon, 'ability: Hydration');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		flags: {},
// 		name: "Hydration",
// 		rating: 1.5,
// 		num: 93,
// 	},

pub mod hydration {
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
// HYPERCUTTER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	hypercutter: {
// 		onTryBoost(boost, target, source, effect) {
// 			if (source && target === source) return;
// 			if (boost.atk && boost.atk < 0) {
// 				delete boost.atk;
// 				if (!(effect as ActiveMove).secondaries) {
// 					this.add("-fail", target, "unboost", "Attack", "[from] ability: Hyper Cutter", `[of] ${target}`);
// 				}
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Hyper Cutter",
// 		rating: 1.5,
// 		num: 52,
// 	},

pub mod hypercutter {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ICEBODY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	icebody: {
// 		onWeather(target, source, effect) {
// 			if (effect.id === 'hail' || effect.id === 'snowscape') {
// 				this.heal(target.baseMaxhp / 16);
// 			}
// 		},
// 		onImmunity(type, pokemon) {
// 			if (type === 'hail') return false;
// 		},
// 		flags: {},
// 		name: "Ice Body",
// 		rating: 1,
// 		num: 115,
// 	},

pub mod icebody {
    use super::*;

    /// onWeather(...)
    pub fn on_weather(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// ICEFACE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	iceface: {
// 		onSwitchInPriority: -2,
// 		onStart(pokemon) {
// 			if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
// 				this.add('-activate', pokemon, 'ability: Ice Face');
// 				this.effectState.busted = false;
// 				pokemon.formeChange('Eiscue', this.effect, true);
// 			}
// 		},
// 		onDamagePriority: 1,
// 		onDamage(damage, target, source, effect) {
// 			if (effect?.effectType === 'Move' && effect.category === 'Physical' && target.species.id === 'eiscue') {
// 				this.add('-activate', target, 'ability: Ice Face');
// 				this.effectState.busted = true;
// 				return 0;
// 			}
// 		},
// 		onCriticalHit(target, type, move) {
// 			if (!target) return;
// 			if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
// 			if (target.volatiles['substitute'] && !(move.flags['bypasssub'] || move.infiltrates)) return;
// 			if (!target.runImmunity(move)) return;
// 			return false;
// 		},
// 		onEffectiveness(typeMod, target, type, move) {
// 			if (!target) return;
// 			if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
// 
// 			const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
// 			if (hitSub) return;
// 
// 			if (!target.runImmunity(move)) return;
// 			return 0;
// 		},
// 		onUpdate(pokemon) {
// 			if (pokemon.species.id === 'eiscue' && this.effectState.busted) {
// 				pokemon.formeChange('Eiscue-Noice', this.effect, true);
// 			}
// 		},
// 		onWeatherChange(pokemon, source, sourceEffect) {
// 			// snow/hail resuming because Cloud Nine/Air Lock ended does not trigger Ice Face
// 			if ((sourceEffect as Ability)?.suppressWeather) return;
// 			if (!pokemon.hp) return;
// 			if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
// 				this.add('-activate', pokemon, 'ability: Ice Face');
// 				this.effectState.busted = false;
// 				pokemon.formeChange('Eiscue', this.effect, true);
// 			}
// 		},
// 		flags: {
// 			failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1,
// 			breakable: 1, notransform: 1,
// 		},
// 		name: "Ice Face",
// 		rating: 3,
// 		num: 248,
// 	},

pub mod iceface {
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

    /// onCriticalHit(...)
    pub fn on_critical_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEffectiveness(...)
    pub fn on_effectiveness(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onWeatherChange(...)
    pub fn on_weather_change(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ICESCALES
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	icescales: {
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (move.category === 'Special') {
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Ice Scales",
// 		rating: 4,
// 		num: 246,
// 	},

pub mod icescales {
    use super::*;

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ILLUMINATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	illuminate: {
// 		onTryBoost(boost, target, source, effect) {
// 			if (source && target === source) return;
// 			if (boost.accuracy && boost.accuracy < 0) {
// 				delete boost.accuracy;
// 				if (!(effect as ActiveMove).secondaries) {
// 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Illuminate", `[of] ${target}`);
// 				}
// 			}
// 		},
// 		onModifyMove(move) {
// 			move.ignoreEvasion = true;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Illuminate",
// 		rating: 0.5,
// 		num: 35,
// 	},

pub mod illuminate {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// ILLUSION
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	illusion: {
// 		onBeforeSwitchIn(pokemon) {
// 			pokemon.illusion = null;
// 			// yes, you can Illusion an active pokemon but only if it's to your right
// 			for (let i = pokemon.side.pokemon.length - 1; i > pokemon.position; i--) {
// 				const possibleTarget = pokemon.side.pokemon[i];
// 				if (!possibleTarget.fainted) {
// 					// If Ogerpon is in the last slot while the Illusion Pokemon is Terastallized
// 					// Illusion will not disguise as anything
// 					if (!pokemon.terastallized || !['Ogerpon', 'Terapagos'].includes(possibleTarget.species.baseSpecies)) {
// 						pokemon.illusion = possibleTarget;
// 					}
// 					break;
// 				}
// 			}
// 		},
// 		onDamagingHit(damage, target, source, move) {
// 			if (target.illusion) {
// 				this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, source, move);
// 			}
// 		},
// 		onEnd(pokemon) {
// 			if (pokemon.illusion) {
// 				this.debug('illusion cleared');
// 				pokemon.illusion = null;
// 				const details = pokemon.getUpdatedDetails();
// 				this.add('replace', pokemon, details);
// 				this.add('-end', pokemon, 'Illusion');
// 				if (this.ruleTable.has('illusionlevelmod')) {
// 					this.hint("Illusion Level Mod is active, so this Pok\u00e9mon's true level was hidden.", true);
// 				}
// 			}
// 		},
// 		onFaint(pokemon) {
// 			pokemon.illusion = null;
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1 },
// 		name: "Illusion",
// 		rating: 4.5,
// 		num: 149,
// 	},

pub mod illusion {
    use super::*;

    /// onBeforeSwitchIn(...)
    pub fn on_before_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onEnd(...)
    pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onFaint(...)
    pub fn on_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IMMUNITY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	immunity: {
// 		onUpdate(pokemon) {
// 			if (pokemon.status === 'psn' || pokemon.status === 'tox') {
// 				this.add('-activate', pokemon, 'ability: Immunity');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		onSetStatus(status, target, source, effect) {
// 			if (status.id !== 'psn' && status.id !== 'tox') return;
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Immunity');
// 			}
// 			return false;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Immunity",
// 		rating: 2,
// 		num: 17,
// 	},

pub mod immunity {
    use super::*;

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(...)
    pub fn on_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IMPOSTER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	imposter: {
// 		onSwitchIn(pokemon) {
// 			// Imposter does not activate when Skill Swapped or when Neutralizing Gas leaves the field
// 			// Imposter copies across in doubles/triples
// 			// (also copies across in multibattle and diagonally in free-for-all,
// 			// but side.foe already takes care of those)
// 			const target = pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
// 			if (target) {
// 				pokemon.transformInto(target, this.dex.abilities.get('imposter'));
// 			}
// 		},
// 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
// 		name: "Imposter",
// 		rating: 5,
// 		num: 150,
// 	},

pub mod imposter {
    use super::*;

    /// onSwitchIn(...)
    pub fn on_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// INFILTRATOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	infiltrator: {
// 		onModifyMove(move) {
// 			move.infiltrates = true;
// 		},
// 		flags: {},
// 		name: "Infiltrator",
// 		rating: 2.5,
// 		num: 151,
// 	},

pub mod infiltrator {
    use super::*;

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// INNARDSOUT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	innardsout: {
// 		onDamagingHitOrder: 1,
// 		onDamagingHit(damage, target, source, move) {
// 			if (!target.hp) {
// 				this.damage(target.getUndynamaxedHP(damage), source, target);
// 			}
// 		},
// 		flags: {},
// 		name: "Innards Out",
// 		rating: 4,
// 		num: 215,
// 	},

pub mod innardsout {
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
// INNERFOCUS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	innerfocus: {
// 		onTryAddVolatile(status, pokemon) {
// 			if (status.id === 'flinch') return null;
// 		},
// 		onTryBoost(boost, target, source, effect) {
// 			if (effect.name === 'Intimidate' && boost.atk) {
// 				delete boost.atk;
// 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Inner Focus', `[of] ${target}`);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Inner Focus",
// 		rating: 1,
// 		num: 39,
// 	},

pub mod innerfocus {
    use super::*;

    /// onTryAddVolatile(...)
    pub fn on_try_add_volatile(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// INSOMNIA
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	insomnia: {
// 		onUpdate(pokemon) {
// 			if (pokemon.status === 'slp') {
// 				this.add('-activate', pokemon, 'ability: Insomnia');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		onSetStatus(status, target, source, effect) {
// 			if (status.id !== 'slp') return;
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Insomnia');
// 			}
// 			return false;
// 		},
// 		onTryAddVolatile(status, target) {
// 			if (status.id === 'yawn') {
// 				this.add('-immune', target, '[from] ability: Insomnia');
// 				return null;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Insomnia",
// 		rating: 1.5,
// 		num: 15,
// 	},

pub mod insomnia {
    use super::*;

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// INTIMIDATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	intimidate: {
// 		onStart(pokemon) {
// 			let activated = false;
// 			for (const target of pokemon.adjacentFoes()) {
// 				if (!activated) {
// 					this.add('-ability', pokemon, 'Intimidate', 'boost');
// 					activated = true;
// 				}
// 				if (target.volatiles['substitute']) {
// 					this.add('-immune', target);
// 				} else {
// 					this.boost({ atk: -1 }, target, pokemon, null, true);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Intimidate",
// 		rating: 3.5,
// 		num: 22,
// 	},

pub mod intimidate {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// INTREPIDSWORD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	intrepidsword: {
// 		onStart(pokemon) {
// 			if (pokemon.swordBoost) return;
// 			pokemon.swordBoost = true;
// 			this.boost({ atk: 1 }, pokemon);
// 		},
// 		flags: {},
// 		name: "Intrepid Sword",
// 		rating: 4,
// 		num: 234,
// 	},

pub mod intrepidsword {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IRONBARBS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	ironbarbs: {
// 		onDamagingHitOrder: 1,
// 		onDamagingHit(damage, target, source, move) {
// 			if (this.checkMoveMakesContact(move, source, target, true)) {
// 				this.damage(source.baseMaxhp / 8, source, target);
// 			}
// 		},
// 		flags: {},
// 		name: "Iron Barbs",
// 		rating: 2.5,
// 		num: 160,
// 	},

pub mod ironbarbs {
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
// IRONFIST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	ironfist: {
// 		onBasePowerPriority: 23,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (move.flags['punch']) {
// 				this.debug('Iron Fist boost');
// 				return this.chainModify([4915, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Iron Fist",
// 		rating: 3,
// 		num: 89,
// 	},

pub mod ironfist {
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
// JUSTIFIED
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	justified: {
// 		onDamagingHit(damage, target, source, move) {
// 			if (move.type === 'Dark') {
// 				this.boost({ atk: 1 });
// 			}
// 		},
// 		flags: {},
// 		name: "Justified",
// 		rating: 2.5,
// 		num: 154,
// 	},

pub mod justified {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// KEENEYE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	keeneye: {
// 		onTryBoost(boost, target, source, effect) {
// 			if (source && target === source) return;
// 			if (boost.accuracy && boost.accuracy < 0) {
// 				delete boost.accuracy;
// 				if (!(effect as ActiveMove).secondaries) {
// 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Keen Eye", `[of] ${target}`);
// 				}
// 			}
// 		},
// 		onModifyMove(move) {
// 			move.ignoreEvasion = true;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Keen Eye",
// 		rating: 0.5,
// 		num: 51,
// 	},

pub mod keeneye {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// KLUTZ
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	klutz: {
// 		// Klutz isn't technically active immediately in-game, but it activates early enough to beat all items
// 		// we should keep an eye out in future gens for items that activate on switch-in before Unnerve
// 		onSwitchInPriority: 1,
// 		// Item suppression implemented in Pokemon.ignoringItem() within sim/pokemon.js
// 		onStart(pokemon) {
// 			this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
// 		},
// 		flags: {},
// 		name: "Klutz",
// 		rating: -1,
// 		num: 103,
// 	},

pub mod klutz {
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
}

// -----------------------------------------------------------------------------
// LEAFGUARD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	leafguard: {
// 		onSetStatus(status, target, source, effect) {
// 			if (['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
// 				if ((effect as Move)?.status) {
// 					this.add('-immune', target, '[from] ability: Leaf Guard');
// 				}
// 				return false;
// 			}
// 		},
// 		onTryAddVolatile(status, target) {
// 			if (status.id === 'yawn' && ['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
// 				this.add('-immune', target, '[from] ability: Leaf Guard');
// 				return null;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Leaf Guard",
// 		rating: 0.5,
// 		num: 102,
// 	},

pub mod leafguard {
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
}

// -----------------------------------------------------------------------------
// LIBERO
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	libero: {
// 		onPrepareHit(source, target, move) {
// 			if (this.effectState.libero) return;
// 			if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
// 			const type = move.type;
// 			if (type && type !== '???' && source.getTypes().join() !== type) {
// 				if (!source.setType(type)) return;
// 				this.effectState.libero = true;
// 				this.add('-start', source, 'typechange', type, '[from] ability: Libero');
// 			}
// 		},
// 		flags: {},
// 		name: "Libero",
// 		rating: 4,
// 		num: 236,
// 	},

pub mod libero {
    use super::*;

    /// onPrepareHit(...)
    pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIGHTMETAL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	lightmetal: {
// 		onModifyWeight(weighthg) {
// 			return this.trunc(weighthg / 2);
// 		},
// 		flags: { breakable: 1 },
// 		name: "Light Metal",
// 		rating: 1,
// 		num: 135,
// 	},

pub mod lightmetal {
    use super::*;

    /// onModifyWeight(...)
    pub fn on_modify_weight(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIGHTNINGROD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	lightningrod: {
// 		onTryHit(target, source, move) {
// 			if (target !== source && move.type === 'Electric') {
// 				if (!this.boost({ spa: 1 })) {
// 					this.add('-immune', target, '[from] ability: Lightning Rod');
// 				}
// 				return null;
// 			}
// 		},
// 		onAnyRedirectTarget(target, source, source2, move) {
// 			if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
// 			const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
// 			if (this.validTarget(this.effectState.target, source, redirectTarget)) {
// 				if (move.smartTarget) move.smartTarget = false;
// 				if (this.effectState.target !== target) {
// 					this.add('-activate', this.effectState.target, 'ability: Lightning Rod');
// 				}
// 				return this.effectState.target;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Lightning Rod",
// 		rating: 3,
// 		num: 31,
// 	},

pub mod lightningrod {
    use super::*;

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onAnyRedirectTarget(...)
    pub fn on_any_redirect_target(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIMBER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	limber: {
// 		onUpdate(pokemon) {
// 			if (pokemon.status === 'par') {
// 				this.add('-activate', pokemon, 'ability: Limber');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		onSetStatus(status, target, source, effect) {
// 			if (status.id !== 'par') return;
// 			if ((effect as Move)?.status) {
// 				this.add('-immune', target, '[from] ability: Limber');
// 			}
// 			return false;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Limber",
// 		rating: 2,
// 		num: 7,
// 	},

pub mod limber {
    use super::*;

    /// onUpdate(...)
    pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(...)
    pub fn on_set_status(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LINGERINGAROMA
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	lingeringaroma: {
// 		onDamagingHit(damage, target, source, move) {
// 			const sourceAbility = source.getAbility();
// 			if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'lingeringaroma') {
// 				return;
// 			}
// 			if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
// 				const oldAbility = source.setAbility('lingeringaroma', target);
// 				if (oldAbility) {
// 					this.add('-activate', target, 'ability: Lingering Aroma', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Lingering Aroma",
// 		rating: 2,
// 		num: 268,
// 	},

pub mod lingeringaroma {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIQUIDOOZE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	liquidooze: {
// 		onSourceTryHeal(damage, target, source, effect) {
// 			this.debug(`Heal is occurring: ${target} <- ${source} :: ${effect.id}`);
// 			const canOoze = ['drain', 'leechseed', 'strengthsap'];
// 			if (canOoze.includes(effect.id)) {
// 				this.damage(damage);
// 				return 0;
// 			}
// 		},
// 		flags: {},
// 		name: "Liquid Ooze",
// 		rating: 2.5,
// 		num: 64,
// 	},

pub mod liquidooze {
    use super::*;

    /// onSourceTryHeal(...)
    pub fn on_source_try_heal(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIQUIDVOICE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	liquidvoice: {
// 		onModifyTypePriority: -1,
// 		onModifyType(move, pokemon) {
// 			if (move.flags['sound'] && !pokemon.volatiles['dynamax']) { // hardcode
// 				move.type = 'Water';
// 			}
// 		},
// 		flags: {},
// 		name: "Liquid Voice",
// 		rating: 1.5,
// 		num: 204,
// 	},

pub mod liquidvoice {
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
}

// -----------------------------------------------------------------------------
// LONGREACH
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	longreach: {
// 		onModifyMove(move) {
// 			delete move.flags['contact'];
// 		},
// 		flags: {},
// 		name: "Long Reach",
// 		rating: 1,
// 		num: 203,
// 	},

pub mod longreach {
    use super::*;

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MAGICBOUNCE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	magicbounce: {
// 		onTryHitPriority: 1,
// 		onTryHit(target, source, move) {
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
// 		name: "Magic Bounce",
// 		rating: 4,
// 		num: 156,
// 	},

pub mod magicbounce {
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
// MAGICGUARD
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	magicguard: {
// 		onDamage(damage, target, source, effect) {
// 			if (effect.effectType !== 'Move') {
// 				if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
// 				return false;
// 			}
// 		},
// 		flags: {},
// 		name: "Magic Guard",
// 		rating: 4,
// 		num: 98,
// 	},

pub mod magicguard {
    use super::*;

    /// onDamage(...)
    pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MAGICIAN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	magician: {
// 		onAfterMoveSecondarySelf(source, target, move) {
// 			if (!move || source.switchFlag === true || !move.hitTargets || source.item || source.volatiles['gem'] ||
// 				move.id === 'fling' || move.category === 'Status') return;
// 			const hitTargets = move.hitTargets;
// 			this.speedSort(hitTargets);
// 			for (const pokemon of hitTargets) {
// 				if (pokemon !== source) {
// 					const yourItem = pokemon.takeItem(source);
// 					if (!yourItem) continue;
// 					if (!source.setItem(yourItem)) {
// 						pokemon.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
// 						continue;
// 					}
// 					this.add('-item', source, yourItem, '[from] ability: Magician', `[of] ${pokemon}`);
// 					return;
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Magician",
// 		rating: 1,
// 		num: 170,
// 	},

pub mod magician {
    use super::*;

    /// onAfterMoveSecondarySelf(...)
    pub fn on_after_move_secondary_self(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MAGMAARMOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	magmaarmor: {
// 		onUpdate(pokemon) {
// 			if (pokemon.status === 'frz') {
// 				this.add('-activate', pokemon, 'ability: Magma Armor');
// 				pokemon.cureStatus();
// 			}
// 		},
// 		onImmunity(type, pokemon) {
// 			if (type === 'frz') return false;
// 		},
// 		flags: { breakable: 1 },
// 		name: "Magma Armor",
// 		rating: 0.5,
// 		num: 40,
// 	},

pub mod magmaarmor {
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
}

// -----------------------------------------------------------------------------
// MAGNETPULL
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	magnetpull: {
// 		onFoeTrapPokemon(pokemon) {
// 			if (pokemon.hasType('Steel') && pokemon.isAdjacent(this.effectState.target)) {
// 				pokemon.tryTrap(true);
// 			}
// 		},
// 		onFoeMaybeTrapPokemon(pokemon, source) {
// 			if (!source) source = this.effectState.target;
// 			if (!source || !pokemon.isAdjacent(source)) return;
// 			if (!pokemon.knownType || pokemon.hasType('Steel')) {
// 				pokemon.maybeTrapped = true;
// 			}
// 		},
// 		flags: {},
// 		name: "Magnet Pull",
// 		rating: 4,
// 		num: 42,
// 	},

pub mod magnetpull {
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
// MARVELSCALE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	marvelscale: {
// 		onModifyDefPriority: 6,
// 		onModifyDef(def, pokemon) {
// 			if (pokemon.status) {
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Marvel Scale",
// 		rating: 2.5,
// 		num: 63,
// 	},

pub mod marvelscale {
    use super::*;

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
}

// -----------------------------------------------------------------------------
// MEGALAUNCHER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	megalauncher: {
// 		onBasePowerPriority: 19,
// 		onBasePower(basePower, attacker, defender, move) {
// 			if (move.flags['pulse']) {
// 				return this.chainModify(1.5);
// 			}
// 		},
// 		flags: {},
// 		name: "Mega Launcher",
// 		rating: 3,
// 		num: 178,
// 	},

pub mod megalauncher {
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
// MERCILESS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	merciless: {
// 		onModifyCritRatio(critRatio, source, target) {
// 			if (target && ['psn', 'tox'].includes(target.status)) return 5;
// 		},
// 		flags: {},
// 		name: "Merciless",
// 		rating: 1.5,
// 		num: 196,
// 	},

pub mod merciless {
    use super::*;

    /// onModifyCritRatio(...)
    pub fn on_modify_crit_ratio(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MIMICRY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mimicry: {
// 		onSwitchInPriority: -1,
// 		onStart(pokemon) {
// 			this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
// 		},
// 		onTerrainChange(pokemon) {
// 			let types;
// 			switch (this.field.terrain) {
// 			case 'electricterrain':
// 				types = ['Electric'];
// 				break;
// 			case 'grassyterrain':
// 				types = ['Grass'];
// 				break;
// 			case 'mistyterrain':
// 				types = ['Fairy'];
// 				break;
// 			case 'psychicterrain':
// 				types = ['Psychic'];
// 				break;
// 			default:
// 				types = pokemon.baseSpecies.types;
// 			}
// 			const oldTypes = pokemon.getTypes();
// 			if (oldTypes.join() === types.join() || !pokemon.setType(types)) return;
// 			if (this.field.terrain || pokemon.transformed) {
// 				this.add('-start', pokemon, 'typechange', types.join('/'), '[from] ability: Mimicry');
// 				if (!this.field.terrain) this.hint("Transform Mimicry changes you to your original un-transformed types.");
// 			} else {
// 				this.add('-activate', pokemon, 'ability: Mimicry');
// 				this.add('-end', pokemon, 'typechange', '[silent]');
// 			}
// 		},
// 		flags: {},
// 		name: "Mimicry",
// 		rating: 0,
// 		num: 250,
// 	},

pub mod mimicry {
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
}

// -----------------------------------------------------------------------------
// MINDSEYE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mindseye: {
// 		onTryBoost(boost, target, source, effect) {
// 			if (source && target === source) return;
// 			if (boost.accuracy && boost.accuracy < 0) {
// 				delete boost.accuracy;
// 				if (!(effect as ActiveMove).secondaries) {
// 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Mind's Eye", `[of] ${target}`);
// 				}
// 			}
// 		},
// 		onModifyMovePriority: -5,
// 		onModifyMove(move) {
// 			move.ignoreEvasion = true;
// 			if (!move.ignoreImmunity) move.ignoreImmunity = {};
// 			if (move.ignoreImmunity !== true) {
// 				move.ignoreImmunity['Fighting'] = true;
// 				move.ignoreImmunity['Normal'] = true;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Mind's Eye",
// 		rating: 0,
// 		num: 300,
// 	},

pub mod mindseye {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

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
// MINUS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	minus: {
// 		onModifySpAPriority: 5,
// 		onModifySpA(spa, pokemon) {
// 			for (const allyActive of pokemon.allies()) {
// 				if (allyActive.hasAbility(['minus', 'plus'])) {
// 					return this.chainModify(1.5);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Minus",
// 		rating: 0,
// 		num: 58,
// 	},

pub mod minus {
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
// MIRRORARMOR
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mirrorarmor: {
// 		onTryBoost(boost, target, source, effect) {
// 			// Don't bounce self stat changes, or boosts that have already bounced
// 			if (!source || target === source || !boost || effect.name === 'Mirror Armor') return;
// 			let b: BoostID;
// 			for (b in boost) {
// 				if (boost[b]! < 0) {
// 					if (target.boosts[b] === -6) continue;
// 					const negativeBoost: SparseBoostsTable = {};
// 					negativeBoost[b] = boost[b];
// 					delete boost[b];
// 					if (source.hp) {
// 						this.add('-ability', target, 'Mirror Armor');
// 						this.boost(negativeBoost, source, target, null, true);
// 					}
// 				}
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Mirror Armor",
// 		rating: 2,
// 		num: 240,
// 	},

pub mod mirrorarmor {
    use super::*;

    /// onTryBoost(...)
    pub fn on_try_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MISTYSURGE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mistysurge: {
// 		onStart(source) {
// 			this.field.setTerrain('mistyterrain');
// 		},
// 		flags: {},
// 		name: "Misty Surge",
// 		rating: 3.5,
// 		num: 228,
// 	},

pub mod mistysurge {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MOLDBREAKER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	moldbreaker: {
// 		onStart(pokemon) {
// 			this.add('-ability', pokemon, 'Mold Breaker');
// 		},
// 		onModifyMove(move) {
// 			move.ignoreAbility = true;
// 		},
// 		flags: {},
// 		name: "Mold Breaker",
// 		rating: 3,
// 		num: 104,
// 	},

pub mod moldbreaker {
    use super::*;

    /// onStart(...)
    pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// MOODY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	moody: {
// 		onResidualOrder: 28,
// 		onResidualSubOrder: 2,
// 		onResidual(pokemon) {
// 			let stats: BoostID[] = [];
// 			const boost: SparseBoostsTable = {};
// 			let statPlus: BoostID;
// 			for (statPlus in pokemon.boosts) {
// 				if (statPlus === 'accuracy' || statPlus === 'evasion') continue;
// 				if (pokemon.boosts[statPlus] < 6) {
// 					stats.push(statPlus);
// 				}
// 			}
// 			let randomStat: BoostID | undefined = stats.length ? this.sample(stats) : undefined;
// 			if (randomStat) boost[randomStat] = 2;
// 
// 			stats = [];
// 			let statMinus: BoostID;
// 			for (statMinus in pokemon.boosts) {
// 				if (statMinus === 'accuracy' || statMinus === 'evasion') continue;
// 				if (pokemon.boosts[statMinus] > -6 && statMinus !== randomStat) {
// 					stats.push(statMinus);
// 				}
// 			}
// 			randomStat = stats.length ? this.sample(stats) : undefined;
// 			if (randomStat) boost[randomStat] = -1;
// 
// 			this.boost(boost, pokemon, pokemon);
// 		},
// 		flags: {},
// 		name: "Moody",
// 		rating: 5,
// 		num: 141,
// 	},

pub mod moody {
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
// MOTORDRIVE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	motordrive: {
// 		onTryHit(target, source, move) {
// 			if (target !== source && move.type === 'Electric') {
// 				if (!this.boost({ spe: 1 })) {
// 					this.add('-immune', target, '[from] ability: Motor Drive');
// 				}
// 				return null;
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Motor Drive",
// 		rating: 3,
// 		num: 78,
// 	},

pub mod motordrive {
    use super::*;

    /// onTryHit(...)
    pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MOUNTAINEER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mountaineer: {
// 		onDamage(damage, target, source, effect) {
// 			if (effect && effect.id === 'stealthrock') {
// 				return false;
// 			}
// 		},
// 		onTryHit(target, source, move) {
// 			if (move.type === 'Rock' && !target.activeTurns) {
// 				this.add('-immune', target, '[from] ability: Mountaineer');
// 				return null;
// 			}
// 		},
// 		isNonstandard: "CAP",
// 		flags: { breakable: 1 },
// 		name: "Mountaineer",
// 		rating: 3,
// 		num: -1,
// 	},

pub mod mountaineer {
    use super::*;

    /// onDamage(...)
    pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
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
// MOXIE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	moxie: {
// 		onSourceAfterFaint(length, target, source, effect) {
// 			if (effect && effect.effectType === 'Move') {
// 				this.boost({ atk: length }, source);
// 			}
// 		},
// 		flags: {},
// 		name: "Moxie",
// 		rating: 3,
// 		num: 153,
// 	},

pub mod moxie {
    use super::*;

    /// onSourceAfterFaint(...)
    pub fn on_source_after_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MULTISCALE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	multiscale: {
// 		onSourceModifyDamage(damage, source, target, move) {
// 			if (target.hp >= target.maxhp) {
// 				this.debug('Multiscale weaken');
// 				return this.chainModify(0.5);
// 			}
// 		},
// 		flags: { breakable: 1 },
// 		name: "Multiscale",
// 		rating: 3.5,
// 		num: 136,
// 	},

pub mod multiscale {
    use super::*;

    /// onSourceModifyDamage(...)
    pub fn on_source_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MUMMY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	mummy: {
// 		onDamagingHit(damage, target, source, move) {
// 			const sourceAbility = source.getAbility();
// 			if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'mummy') {
// 				return;
// 			}
// 			if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
// 				const oldAbility = source.setAbility('mummy', target);
// 				if (oldAbility) {
// 					this.add('-activate', target, 'ability: Mummy', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
// 				}
// 			}
// 		},
// 		flags: {},
// 		name: "Mummy",
// 		rating: 2,
// 		num: 152,
// 	},

pub mod mummy {
    use super::*;

    /// onDamagingHit(...)
    pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MYCELIUMMIGHT
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	myceliummight: {
// 		onFractionalPriorityPriority: -1,
// 		onFractionalPriority(priority, pokemon, target, move) {
// 			if (move.category === 'Status') {
// 				return -0.1;
// 			}
// 		},
// 		onModifyMove(move) {
// 			if (move.category === 'Status') {
// 				move.ignoreAbility = true;
// 			}
// 		},
// 		flags: {},
// 		name: "Mycelium Might",
// 		rating: 2,
// 		num: 298,
// 	},

pub mod myceliummight {
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

    /// onModifyMove(...)
    pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// NATURALCURE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	naturalcure: {
// 		onCheckShow(pokemon) {
// 			// This is complicated
// 			// For the most part, in-game, it's obvious whether or not Natural Cure activated,
// 			// since you can see how many of your opponent's pokemon are statused.
// 			// The only ambiguous situation happens in Doubles/Triples, where multiple pokemon
// 			// that could have Natural Cure switch out, but only some of them get cured.
// 			if (pokemon.side.active.length === 1) return;
// 			if (pokemon.showCure === true || pokemon.showCure === false) return;
// 
// 			const cureList = [];
// 			let noCureCount = 0;
// 			for (const curPoke of pokemon.side.active) {
// 				// pokemon not statused
// 				if (!curPoke?.status) {
// 					// this.add('-message', "" + curPoke + " skipped: not statused or doesn't exist");
// 					continue;
// 				}
// 				if (curPoke.showCure) {
// 					// this.add('-message', "" + curPoke + " skipped: Natural Cure already known");
// 					continue;
// 				}
// 				const species = curPoke.species;
// 				// pokemon can't get Natural Cure
// 				if (!Object.values(species.abilities).includes('Natural Cure')) {
// 					// this.add('-message', "" + curPoke + " skipped: no Natural Cure");
// 					continue;
// 				}
// 				// pokemon's ability is known to be Natural Cure
// 				if (!species.abilities['1'] && !species.abilities['H']) {
// 					// this.add('-message', "" + curPoke + " skipped: only one ability");
// 					continue;
// 				}
// 				// pokemon isn't switching this turn
// 				if (curPoke !== pokemon && !this.queue.willSwitch(curPoke)) {
// 					// this.add('-message', "" + curPoke + " skipped: not switching");
// 					continue;
// 				}
// 
// 				if (curPoke.hasAbility('naturalcure')) {
// 					// this.add('-message', "" + curPoke + " confirmed: could be Natural Cure (and is)");
// 					cureList.push(curPoke);
// 				} else {
// 					// this.add('-message', "" + curPoke + " confirmed: could be Natural Cure (but isn't)");
// 					noCureCount++;
// 				}
// 			}
// 
// 			if (!cureList.length || !noCureCount) {
// 				// It's possible to know what pokemon were cured
// 				for (const pkmn of cureList) {
// 					pkmn.showCure = true;
// 				}
// 			} else {
// 				// It's not possible to know what pokemon were cured
// 
// 				// Unlike a -hint, this is real information that battlers need, so we use a -message
// 				this.add('-message', `(${cureList.length} of ${pokemon.side.name}'s pokemon ${cureList.length === 1 ? "was" : "were"} cured by Natural Cure.)`);
// 
// 				for (const pkmn of cureList) {
// 					pkmn.showCure = false;
// 				}
// 			}
// 		},
// 		onSwitchOut(pokemon) {
// 			if (!pokemon.status) return;
// 
// 			// if pokemon.showCure is undefined, it was skipped because its ability
// 			// is known
// 			if (pokemon.showCure === undefined) pokemon.showCure = true;
// 
// 			if (pokemon.showCure) this.add('-curestatus', pokemon, pokemon.status, '[from] ability: Natural Cure');
// 			pokemon.clearStatus();
// 
// 			// only reset .showCure if it's false
// 			// (once you know a Pokemon has Natural Cure, its cures are always known)
// 			if (!pokemon.showCure) pokemon.showCure = undefined;
// 		},
// 		flags: {},
// 		name: "Natural Cure",
// 		rating: 2.5,
// 		num: 30,
// 	},

pub mod naturalcure {
    use super::*;

    /// onCheckShow(...)
    pub fn on_check_show(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }

    /// onSwitchOut(...)
    pub fn on_switch_out(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// NEUROFORCE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// 	neuroforce: {
// 		onModifyDamage(damage, source, target, move) {
// 			if (move && target.getMoveHitData(move).typeMod > 0) {
// 				return this.chainModify([5120, 4096]);
// 			}
// 		},
// 		flags: {},
// 		name: "Neuroforce",
// 		rating: 2.5,
// 		num: 233,
// 	},

pub mod neuroforce {
    use super::*;

    /// onModifyDamage(...)
    pub fn on_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
        // TODO: Implement 1-to-1 from JS
        AbilityHandlerResult::Undefined
    }
}
