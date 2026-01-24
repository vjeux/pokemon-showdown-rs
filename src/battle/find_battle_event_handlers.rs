// 1:1 port of findBattleEventHandlers from battle.ts

use crate::*;
use crate::battle::{Effect, EffectHolder, EventListener, EffectType};

impl Battle {
    /// Find battle-level event handlers
    /// Equivalent to battle.ts findBattleEventHandlers()
    ///
    // JS Source:
    // 	findBattleEventHandlers(callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    //
    // 		let callback;
    // 		const format = this.format;
    // 		callback = this.getCallback(this, format, callbackName);
    // 		if (callback !== undefined || (getKey && this.formatData[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: format, callback, state: this.formatData, end: null, effectHolder: customHolder || this,
    // 			}, callbackName));
    // 		}
    // 		if (this.events && (callback = this.events[callbackName]) !== undefined) {
    // 			for (const handler of callback) {
    // 				const state = (handler.target.effectType === 'Format') ? this.formatData : null;
    // 				handlers.push({
    // 					effect: handler.target, callback: handler.callback, state, end: null, effectHolder: customHolder || this,
    // 					priority: handler.priority, order: handler.order, subOrder: handler.subOrder,
    // 				});
    // 			}
    // 		}
    // 		return handlers;
    // 	}
    pub fn find_battle_event_handlers(
        &self,
        callback_name: &str,
        get_key: Option<&str>,
        custom_holder: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        debug_elog!("[FIND_BATTLE_EVENT_HANDLERS] callback_name='{}', get_key={:?}, custom_holder={:?}",
            callback_name, get_key, custom_holder);

        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();

        // JS: let callback;
        // JS: const format = this.format;
        // JS: callback = this.getCallback(this, format, callbackName);
        // TODO: getCallback is architectural difference - returns None in Rust
        // Format callbacks are handled through static dispatch (handle_format_event, etc.)

        // JS: if (callback !== undefined || (getKey && this.formatData[getKey])) {
        // Check if formatData has the specified key
        let has_get_key = get_key.is_some_and(|key| {
            match key {
                "duration" => self.format_data.borrow().duration.is_some(),
                _ => false,
            }
        });

        // Since getCallback returns None, we can't create the format handler yet
        // but we check get_key for correctness
        // TODO: Implement format event handlers when static dispatch system is ready
        if has_get_key {
            // Would create format handler here if getCallback was implemented
        }

        // Static dispatch for format/rule callbacks
        // Check if the format includes rules with callbacks and add handlers for them
        if callback_name == "onSetStatus" {
            // Sleep Clause Mod: Prevents putting multiple Pokemon to sleep
            // JavaScript: rulesets.ts lines 1392-1410
            let _has_sleep_clause = self.rule_table.as_ref().map(|rt| rt.has("sleepclausemod")).unwrap_or(false);

            // Debug: print all rules in rule_table
            if let Some(ref rule_table) = self.rule_table {
                let _all_rules: Vec<_> = rule_table.keys().collect();
                debug_elog!("[FIND_BATTLE_EVENT_HANDLERS] All rules in rule_table: {:?}", _all_rules);
            }

            debug_elog!("[FIND_BATTLE_EVENT_HANDLERS] Checking for sleepclausemod, rule_table exists={}, has_sleep_clause={}",
                self.rule_table.is_some(), _has_sleep_clause);

            if let Some(ref rule_table) = self.rule_table {
                if rule_table.has("sleepclausemod") {
                    handlers.push(EventListener {
                        callback_name: "SetStatus".to_string(),
                        effect: Effect {
                            id: ID::new("sleepclausemod"),
                            name: "Sleep Clause Mod".into(),
                            effect_type: EffectType::Format,
                            effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Battle)),
                            side_index: None,
                            prankster_boosted: false,
                        },
                        target: None,
                        index: None,
                        state: None,
                        effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Battle)),
                        order: None,
                        priority: 0,
                        sub_order: 0,
                        effect_order: None,
                        speed: None,
                    });
                }
            }
        }

        // JS: if (this.events && (callback = this.events[callbackName]) !== undefined) {
        if let Some(custom_handlers) = self.events.get(callback_name) {
            // JS: for (const handler of callback) {
            for handler in custom_handlers {
                // JS: const state = (handler.target.effectType === 'Format') ? this.formatData : null;
                let state = if handler.target_type == EffectType::Format {
                    Some(self.format_data.clone())
                } else {
                    None
                };

                // JS: handlers.push({
                // JS:     effect: handler.target,
                // JS:     callback: handler.callback,
                // JS:     state,
                // JS:     end: null,
                // JS:     effectHolder: customHolder || this,
                // JS:     priority: handler.priority,
                // JS:     order: handler.order,
                // JS:     subOrder: handler.subOrder,
                // JS: });
                handlers.push(EventListener {
                    callback_name: String::new(),
                    effect: Effect {
                        id: handler.target_id.clone(),
                        name: handler.target_id.as_str().into(),
                        effect_type: handler.target_type,
                        effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Battle)),
                        side_index: None,
                        prankster_boosted: false,
                    },
                    target: None,
                    index: None,
                    state,
                    effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Battle)),
                    order: if handler.order { Some(0) } else { None },
                    priority: handler.priority,
                    sub_order: handler.sub_order,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: return handlers;
        handlers
    }
}
