use crate::*;
use crate::pokemon::MoveSlot;

impl Pokemon {

    /// Transform into another Pokemon
    //
    // 	transformInto(pokemon: Pokemon, effect?: Effect) {
    // 		const species = pokemon.species;
    // 		if (
    // 			pokemon.fainted || this.illusion || pokemon.illusion || (pokemon.volatiles['substitute'] && this.battle.gen >= 5) ||
    // 			(pokemon.transformed && this.battle.gen >= 2) || (this.transformed && this.battle.gen >= 5) ||
    // 			species.name === 'Eternatus-Eternamax' ||
    // 			(['Ogerpon', 'Terapagos'].includes(species.baseSpecies) && (this.terastallized || pokemon.terastallized)) ||
    // 			this.terastallized === 'Stellar'
    // 		) {
    // 			return false;
    // 		}
    //
    // 		if (this.battle.dex.currentMod === 'gen1stadium' && (
    // 			species.name === 'Ditto' ||
    // 			(this.species.name === 'Ditto' && pokemon.moves.includes('transform'))
    // 		)) {
    // 			return false;
    // 		}
    //
    // 		if (!this.setSpecies(species, effect, true)) return false;
    //
    // 		this.transformed = true;
    // 		this.weighthg = pokemon.weighthg;
    //
    // 		const types = pokemon.getTypes(true, true);
    // 		this.setType(pokemon.volatiles['roost'] ? pokemon.volatiles['roost'].typeWas : types, true);
    // 		this.addedType = pokemon.addedType;
    // 		this.knownType = this.isAlly(pokemon) && pokemon.knownType;
    // 		this.apparentType = pokemon.apparentType;
    //
    // 		let statName: StatIDExceptHP;
    // 		for (statName in this.storedStats) {
    // 			this.storedStats[statName] = pokemon.storedStats[statName];
    // 			if (this.modifiedStats) this.modifiedStats[statName] = pokemon.modifiedStats![statName]; // Gen 1: Copy modified stats.
    // 		}
    // 		this.moveSlots = [];
    // 		this.hpType = (this.battle.gen >= 5 ? this.hpType : pokemon.hpType);
    // 		this.hpPower = (this.battle.gen >= 5 ? this.hpPower : pokemon.hpPower);
    // 		this.timesAttacked = pokemon.timesAttacked;
    // 		for (const moveSlot of pokemon.moveSlots) {
    // 			let moveName = moveSlot.move;
    // 			if (moveSlot.id === 'hiddenpower') {
    // 				moveName = 'Hidden Power ' + this.hpType;
    // 			}
    // 			this.moveSlots.push({
    // 				move: moveName,
    // 				id: moveSlot.id,
    // 				pp: moveSlot.maxpp === 1 ? 1 : 5,
    // 				maxpp: this.battle.gen >= 5 ? (moveSlot.maxpp === 1 ? 1 : 5) : moveSlot.maxpp,
    // 				target: moveSlot.target,
    // 				disabled: false,
    // 				used: false,
    // 				virtual: true,
    // 			});
    // 		}
    // 		let boostName: BoostID;
    // 		for (boostName in pokemon.boosts) {
    // 			this.boosts[boostName] = pokemon.boosts[boostName];
    // 		}
    // 		if (this.battle.gen >= 6) {
    // 			// we need to remove all of the overlapping crit volatiles before adding any of them
    // 			const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    // 			for (const volatile of volatilesToCopy) this.removeVolatile(volatile);
    // 			for (const volatile of volatilesToCopy) {
    // 				if (pokemon.volatiles[volatile]) {
    // 					this.addVolatile(volatile);
    // 					if (volatile === 'gmaxchistrike') this.volatiles[volatile].layers = pokemon.volatiles[volatile].layers;
    // 					if (volatile === 'dragoncheer') this.volatiles[volatile].hasDragonType = pokemon.volatiles[volatile].hasDragonType;
    // 				}
    // 			}
    // 		}
    // 		if (effect) {
    // 			this.battle.add('-transform', this, pokemon, '[from] ' + effect.fullname);
    // 		} else {
    // 			this.battle.add('-transform', this, pokemon);
    // 		}
    // 		if (this.terastallized) {
    // 			this.knownType = true;
    // 			this.apparentType = this.terastallized;
    // 		}
    // 		if (this.battle.gen > 2) this.setAbility(pokemon.ability, this, null, true, true);
    //
    // 		// Change formes based on held items (for Transform)
    // 		// Only ever relevant in Generation 4 since Generation 3 didn't have item-based forme changes
    // 		if (this.battle.gen === 4) {
    // 			if (this.species.num === 487) {
    // 				// Giratina formes
    // 				if (this.species.name === 'Giratina' && this.item === 'griseousorb') {
    // 					this.formeChange('Giratina-Origin');
    // 				} else if (this.species.name === 'Giratina-Origin' && this.item !== 'griseousorb') {
    // 					this.formeChange('Giratina');
    // 				}
    // 			}
    // 			if (this.species.num === 493) {
    // 				// Arceus formes
    // 				const item = this.getItem();
    // 				const targetForme = (item?.onPlate ? 'Arceus-' + item.onPlate : 'Arceus');
    // 				if (this.species.name !== targetForme) {
    // 					this.formeChange(targetForme);
    // 				}
    // 			}
    // 		}
    //
    // 		// Pokemon transformed into Ogerpon cannot Terastallize
    // 		// restoring their ability to tera after they untransform is handled ELSEWHERE
    // 		if (['Ogerpon', 'Terapagos'].includes(this.species.baseSpecies) && this.canTerastallize) this.canTerastallize = false;
    //
    // 		return true;
    // 	}
    //
    pub fn transform_into(&mut self, target: &Pokemon) -> bool {
        // TODO: implement the same logic as JavaScript

        if self.fainted || target.fainted || self.transformed {
            return false;
        }
        if target.has_volatile(&ID::new("substitute")) {
            return false;
        }
        if target.transformed {
            return false;
        }

        // Copy species
        self.species_id = target.species_id.clone();
        self.transformed = true;
        self.weight_hg = target.weight_hg;

        // Copy types
        self.types = target.types.clone();
        self.added_type = target.added_type.clone();

        // Copy stats
        self.stored_stats = target.stored_stats;

        // Copy moves with reduced PP
        self.move_slots = target
            .move_slots
            .iter()
            .map(|slot| MoveSlot {
                id: slot.id.clone(),
                move_name: slot.move_name.clone(),
                pp: 5.min(slot.maxpp),
                maxpp: 5.min(slot.maxpp),
                target: slot.target.clone(),
                disabled: false,
                disabled_source: None,
                used: false,
                virtual_move: true,
                is_z: slot.is_z,
            })
            .collect();

        // Copy boosts
        self.boosts = target.boosts;

        // Copy ability
        self.ability = target.ability.clone();

        true
    }
}
