// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_data::BoostsTable;

impl Pokemon {

    /// Clear volatile with switch flag handling
    /// Equivalent to clearVolatile in pokemon.ts
    ///
    /// Note: This method requires a mutable reference to Battle to call set_species()
    /// which is necessary to recalculate stats. This is a borrow-checker requirement
    /// that doesn't exist in TypeScript where Pokemon has a battle reference.
    pub fn clear_volatile_full(&mut self, battle: &mut Battle, include_switch_flags: bool) {
        // TODO: this should be moved to clear_volatile.rs

        // JS: this.boosts = { atk: 0, def: 0, spa: 0, spd: 0, spe: 0, accuracy: 0, evasion: 0 };
        self.boosts = BoostsTable::default();

        // JS: this.moveSlots = this.baseMoveSlots.slice();
        self.move_slots = self.base_move_slots.clone();

        // JS: this.transformed = false;
        self.transformed = false;

        // JS: this.ability = this.baseAbility;
        self.ability = self.base_ability.clone();

        // JS: this.hpType = this.baseHpType;
        // JS: this.hpPower = this.baseHpPower;
        // TODO: Implement hpType and hpPower

        // JS: for (const i in this.volatiles) { ... remove linked volatiles ... }
        // JS: this.volatiles = {};
        self.volatiles.clear();

        // JS: if (includeSwitchFlags) { ... }
        if include_switch_flags {
            self.switch_flag = false;
            self.force_switch_flag = false;
        }

        // JS: this.lastMove = null;
        // JS: if (this.battle.gen === 2) this.lastMoveEncore = null;
        // JS: this.lastMoveUsed = null;
        // JS: this.moveThisTurn = '';
        self.last_move = None;
        self.last_move_used = None;
        self.move_this_turn = None;

        // JS: this.lastDamage = 0;
        // JS: this.attackedBy = [];
        // JS: this.hurtThisTurn = null;
        self.last_damage = 0;
        // TODO: attacked_by field doesn't exist in Rust Pokemon struct
        self.hurt_this_turn = None;

        // JS: this.newlySwitched = true;
        // JS: this.beingCalledBack = false;
        self.newly_switched = true;
        self.being_called_back = false;

        // JS: this.setSpecies(this.baseSpecies);
        // This is the key step - recalculate stats for the base species
        let base_species = self.base_species.clone();
        self.set_species(battle, &base_species, None, false);
    }
}
