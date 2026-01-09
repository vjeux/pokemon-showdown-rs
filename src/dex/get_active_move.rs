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
            condition: move_data.condition.clone(),
            base_power: move_data.base_power,
            accuracy: move_data.accuracy.clone(),
            pp: move_data.pp,
            category: move_data.category.clone(),
            move_type: move_data.move_type.clone(),
            priority: move_data.priority,
            target: move_data.target.clone(),
            flags: move_data.flags.clone(),
            real_move: None,
            damage: None,
            contest_type: move_data.contest_type.clone(),
            no_pp_boosts: move_data.no_pp_boosts,
            is_z: move_data.is_z.clone(),
            z_move: None,
            is_max: move_data.is_max.clone(),
            max_move: None,
            ohko: move_data.ohko.clone(),
            thaws_target: move_data.thaws_target,
            heal: move_data.heal,
            drain: move_data.drain,
            force_switch: move_data.force_switch,
            self_switch: move_data.self_switch.as_ref().and_then(|v| {
                // Handle both boolean and string values
                // JavaScript: selfSwitch?: boolean | string
                // - true means normal switch
                // - 'copyvolatile' or 'shedtail' means special switch behavior
                if let Some(s) = v.as_str() {
                    Some(s.to_string())
                } else if v.as_bool() == Some(true) {
                    Some("true".to_string())
                } else {
                    None
                }
            }),
            self_boost: move_data.self_boost.as_ref().and_then(|sb| {
                // selfBoost has structure: { boosts: { atk: 1, def: 1, ... } }
                let boosts_val = sb.get("boosts")?;
                let boosts_obj = boosts_val.as_object()?;
                let mut boosts_map = std::collections::HashMap::new();
                for (k, v) in boosts_obj {
                    if let Some(n) = v.as_i64() {
                        boosts_map.insert(k.clone(), n as i32);
                    }
                }
                Some(Self::convert_boosts_hash_to_table(&boosts_map))
            }),
            self_destruct: move_data.selfdestruct.clone(),
            breaks_protect: move_data.breaks_protect,
            recoil: move_data.recoil,
            mindblown_recoil: move_data.mind_blown_recoil,
            steals_boosts: move_data.steals_boosts,
            struggle_recoil: move_data.struggle_recoil,
            secondary: move_data.secondary.as_ref().map(|s| Self::convert_secondary(s)),
            secondaries: move_data.secondaries.as_ref()
                .map(|secs| secs.iter().map(|s| Self::convert_secondary(s)).collect())
                .unwrap_or_default(),
            self_effect: move_data.self_effect.as_ref().map(|s| Self::convert_self_effect(s)),
            has_sheer_force: move_data.has_sheer_force,
            always_hit: matches!(move_data.accuracy, Accuracy::AlwaysHits),
            base_move_type: None,
            base_power_modifier: None,
            crit_modifier: None,
            crit_ratio: move_data.crit_ratio,
            override_offensive_pokemon: move_data.override_offensive_pokemon.clone(),
            override_offensive_stat: move_data.override_offensive_stat.clone(),
            override_defensive_pokemon: None,
            override_defensive_stat: move_data.override_defensive_stat.clone(),
            force_stab: false,
            ignore_ability: move_data.ignore_ability,
            ignore_accuracy: move_data.ignore_accuracy,
            ignore_evasion: move_data.ignore_evasion,
            ignore_positive_evasion: None,
            ignore_immunity: move_data.ignore_immunity.as_ref().map(|v| {
                // Handle both boolean and object values
                // JavaScript: ignoreImmunity: boolean | { Type: true, ... }
                if v.as_bool() == Some(true) {
                    crate::battle_actions::IgnoreImmunity::All
                } else if let Some(obj) = v.as_object() {
                    // Convert object to HashMap
                    let mut map = std::collections::HashMap::new();
                    for (k, val) in obj {
                        if let Some(b) = val.as_bool() {
                            map.insert(k.clone(), b);
                        }
                    }
                    crate::battle_actions::IgnoreImmunity::Specific(map)
                } else {
                    crate::battle_actions::IgnoreImmunity::All
                }
            }),
            ignore_defensive: move_data.ignore_defensive,
            ignore_offensive: false,
            ignore_negative_offensive: false,
            ignore_positive_defensive: false,
            infiltrates: false,
            will_crit: move_data.will_crit,
            multi_accuracy: move_data.multiaccuracy,
            multi_hit: move_data.multihit.clone(),
            multi_hit_type: None,
            no_damage_variance: None,
            non_ghost_target: None,
            spread_modifier: None,
            sleep_usable: move_data.sleep_usable,
            smart_target: move_data.smart_target,
            tracks_target: move_data.tracks_target,
            calls_move: move_data.calls_move,
            has_crash_damage: move_data.has_crash_damage,
            is_confusion_self_hit: None,
            stalling_move: move_data.stalling_move,
            base_move: move_data.base_move.clone(),

            // From HitEffect
            boosts: move_data.boosts,
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
