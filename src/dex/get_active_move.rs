use crate::*;
use crate::battle_actions::ActiveMove;
use crate::dex::Accuracy;

impl Dex {

    /// Get an ActiveMove from a move name or ID
    /// Equivalent to Dex.getActiveMove() in dex.ts
    ///
    /// TypeScript source:
    /// ```typescript
    /// getActiveMove(move: Move | string): ActiveMove {
    ///     if (move && typeof (move as ActiveMove).hit === 'number') return move as ActiveMove;
    ///     move = this.moves.get(move);
    ///     const moveCopy: ActiveMove = this.deepClone(move);
    ///     moveCopy.hit = 0;
    ///     return moveCopy;
    /// }
    /// ```
    ///
    /// Creates a mutable copy of move data suitable for use in battle
    pub fn get_active_move(&self, name: &str) -> Option<ActiveMove> {
        let move_data = self.moves().get(name)?;

        // Convert MoveData to ActiveMove
        let active_move = ActiveMove {
            // From BasicEffect
            id: move_data.id.clone(),
            name: move_data.name.clone(),
            fullname: String::new(), // Will be set during battle
            num: move_data.num,
            exists: true,
            gen: self.gen,
            short_desc: String::new(),
            desc: String::new(),
            is_nonstandard: None,
            duration: None,
            no_copy: false,
            affects_fainted: false,
            source_effect_name: None,

            // From MoveData
            // Embedded conditions (like gmaxvolcalith) don't have names, store None
            condition: move_data.condition.as_ref().and_then(|c| c.name.clone()),
            base_power: move_data.base_power,
            accuracy: move_data.accuracy.clone(),
            pp: move_data.pp as u8,
            category: move_data.category.clone(),
            move_type: move_data.move_type.clone(),
            priority: move_data.priority,
            target: move_data.target.clone(),
            flags: Self::convert_move_flags(&move_data.flags),
            real_move: None,
            damage: None,
            contest_type: None,
            no_pp_boosts: None,
            is_z: false,
            z_move: None,
            is_max: false,
            max_move: None,
            ohko: None,
            thaws_target: None,
            heal: move_data.heal,
            drain: None,
            force_switch: false,
            self_switch: None,
            self_boost: None,
            self_destruct: move_data.selfdestruct.clone(),
            breaks_protect: false,
            recoil: move_data.recoil,
            mindblown_recoil: false,
            steals_boosts: false,
            struggle_recoil: false,
            secondary: None,
            secondaries: {
                let mut secs = Vec::new();
                // Add singular secondary if it exists
                if let Some(ref sec) = move_data.secondary {
                    secs.push(Self::convert_secondary(sec));
                }
                // Add plural secondaries if they exist
                if let Some(ref sec_vec) = move_data.secondaries {
                    for sec in sec_vec {
                        secs.push(Self::convert_secondary(sec));
                    }
                }
                secs
            },
            self_effect: move_data.self_effect.as_ref().map(|se| Self::convert_self_effect(se)),
            has_sheer_force: false,
            always_hit: matches!(move_data.accuracy, Accuracy::AlwaysHits),
            base_move_type: None,
            base_power_modifier: None,
            crit_modifier: None,
            crit_ratio: 0,
            override_offensive_pokemon: None,
            override_offensive_stat: None,
            override_defensive_pokemon: None,
            override_defensive_stat: None,
            force_stab: false,
            ignore_ability: false,
            ignore_accuracy: false,
            ignore_evasion: false,
            ignore_positive_evasion: None,
            ignore_immunity: None,
            ignore_defensive: false,
            ignore_offensive: false,
            ignore_negative_offensive: false,
            ignore_positive_defensive: false,
            infiltrates: false,
            will_crit: move_data.will_crit,
            multi_accuracy: false,
            multi_hit: move_data.multihit.clone(),
            multi_hit_type: None,
            no_damage_variance: None,
            non_ghost_target: None,
            spread_modifier: None,
            sleep_usable: false,
            smart_target: move_data.smart_target,
            tracks_target: move_data.tracks_target.unwrap_or(false),
            calls_move: None,
            has_crash_damage: None,
            is_confusion_self_hit: None,
            stalling_move: None,
            base_move: None,

            // From HitEffect
            boosts: move_data
                .boosts
                .as_ref()
                .map(Self::convert_boosts_hash_to_table),
            status: move_data.status.clone(),
            volatile_status: move_data.volatile_status.clone(),
            side_condition: move_data.side_condition.clone(),
            slot_condition: move_data.slot_condition.clone(),
            pseudo_weather: move_data.pseudo_weather.clone(),
            terrain: move_data.terrain.clone(),
            weather: move_data.weather.clone(),

            // ActiveMove-specific
            hit: 0,
            total_damage: 0,
            move_hit_data: std::collections::HashMap::new(),
            spread_hit: false,
            last_hit: None,
            is_external: false,
            is_z_or_max_powered: false,
            prankster_boosted: false,
            has_bounced: false,
            source_effect: None,
            has_aura_break: None,
            aura_booster: None,
            caused_crash_damage: None,
            self_dropped: false,
            stellar_boosted: false,
            type_changer_boosted: None,
            magnitude: None,
            will_change_forme: false,
            status_roll: None,
            force_status: None,
            ruined_atk: None,
            ruined_spa: None,
            ruined_def: None,
            ruined_spd: None,
            allies: None,
            ability: None,
            hit_targets: Vec::new(),
        };

        Some(active_move)
    }
}
