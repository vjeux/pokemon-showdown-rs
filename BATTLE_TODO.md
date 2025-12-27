# Battle Module TODO - Method Reference

This file contains all Rust methods from battle*.rs files.
Use this as a reference to track which methods need TypeScript source comments.

Generated: 2025-12-27

---

## battle.rs

**Total methods:** 186 (125 public, 61 private)

### Public Methods

- [ ] `new` (line 165)
  ```rust
  pub fn new(id: &str) -> Self
  ```

- [ ] `new` (line 388)
  ```rust
  pub fn new(options: BattleOptions) -> Self
  ```

- [ ] `pokemon_at` (line 493)
  ```rust
  pub fn pokemon_at(&self, side_idx: usize, poke_idx: usize) -> Option<&Pokemon>
  ```

- [ ] `pokemon_at_mut` (line 500)
  ```rust
  pub fn pokemon_at_mut(&mut self, side_idx: usize, poke_idx: usize) -> Option<&mut Pokemon>
  ```

- [ ] `set_player` (line 544)
  ```rust
  pub fn set_player(&mut self, side_id: SideID, options: PlayerOptions)
  ```

- [ ] `start` (line 711)
  ```rust
  pub fn start(&mut self)
  ```

- [ ] `start_battle` (line 840)
  ```rust
  pub fn start_battle(&mut self)
  ```

- [ ] `switch_in` (line 877)
  ```rust
  pub fn switch_in( &mut self, side_index: usize, pos: usize, pokemon_index: usize, source_effect: Option<&ID>, is_drag: bool, ) -> SwitchResult
  ```

- [ ] `add_log` (line 1050)
  ```rust
  pub fn add_log(&mut self, event_type: &str, args: &[&str])
  ```

- [ ] `random` (line 1065)
  ```rust
  pub fn random(&mut self, n: i32) -> i32
  ```

- [ ] `random_chance` (line 1076)
  ```rust
  pub fn random_chance(&mut self, numerator: i32, denominator: i32) -> bool
  ```

- [ ] `sample` (line 1089)
  ```rust
  pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T>
  ```

- [ ] `shuffle` (line 1094)
  ```rust
  pub fn shuffle<T>(&mut self, items: &mut [T])
  ```

- [ ] `get_side` (line 1104)
  ```rust
  pub fn get_side(&self, side_id: SideID) -> Option<&Side>
  ```

- [ ] `get_side_mut` (line 1109)
  ```rust
  pub fn get_side_mut(&mut self, side_id: SideID) -> Option<&mut Side>
  ```

- [ ] `p1` (line 1114)
  ```rust
  pub fn p1(&self) -> Option<&Side>
  ```

- [ ] `p2` (line 1119)
  ```rust
  pub fn p2(&self) -> Option<&Side>
  ```

- [ ] `get_all_active` (line 1139)
  ```rust
  pub fn get_all_active(&self, include_fainted: bool) -> Vec<&crate::pokemon::Pokemon>
  ```

- [ ] `check_win` (line 1172)
  ```rust
  pub fn check_win(&mut self, _faint_data: Option<FaintData>) -> bool
  ```

- [ ] `end` (line 1233)
  ```rust
  pub fn end(&mut self, winner: Option<&str>)
  ```

- [ ] `next_effect_order` (line 1246)
  ```rust
  pub fn next_effect_order(&mut self) -> i32
  ```

- [ ] `init_effect_state` (line 1265)
  ```rust
  pub fn init_effect_state(&mut self, id: ID) -> EffectState
  ```

- [ ] `choose` (line 1295)
  ```rust
  pub fn choose(&mut self, side_id: SideID, choice: &str) -> Result<(), String>
  ```

- [ ] `get_log` (line 1609)
  ```rust
  pub fn get_log(&self) -> String
  ```

- [ ] `make_choices` (line 1632)
  ```rust
  pub fn make_choices(&mut self, p1_choice: &str, p2_choice: &str)
  ```

- [ ] `drag_in` (line 2011)
  ```rust
  pub fn drag_in(&mut self, side_idx: usize, slot: usize) -> bool
  ```

- [ ] `run_switch` (line 2085)
  ```rust
  pub fn run_switch(&mut self, side_idx: usize, poke_idx: usize)
  ```

- [ ] `cure_status` (line 2436)
  ```rust
  pub fn cure_status(&mut self, target: (usize, usize)) -> bool
  ```

- [ ] `debug` (line 2862)
  ```rust
  pub fn debug(&mut self, activity: &str)
  ```

- [ ] `get_debug_log` (line 2875)
  ```rust
  pub fn get_debug_log(&self) -> String
  ```

- [ ] `add` (line 2907)
  ```rust
  pub fn add(&mut self, event_type: &str, args: &[Arg])
  ```

- [ ] `add_move` (line 2982)
  ```rust
  pub fn add_move(&mut self, parts: &[&str])
  ```

- [ ] `hint` (line 3004)
  ```rust
  pub fn hint(&mut self, hint_text: &str, once: bool, side_id: Option<SideID>)
  ```

- [ ] `trunc` (line 3031)
  ```rust
  pub fn trunc(&self, num: f64) -> i32
  ```

- [ ] `chain` (line 3055)
  ```rust
  pub fn chain(&self, previous_mod: (i32, i32), next_mod: (i32, i32)) -> f64
  ```

- [ ] `chain_f` (line 3067)
  ```rust
  pub fn chain_f(&self, previous_mod: f64, next_mod: f64) -> f64
  ```

- [ ] `modify` (line 3094)
  ```rust
  pub fn modify(&self, value: i32, numerator: i32, denominator: i32) -> i32
  ```

- [ ] `modify_tuple` (line 3104)
  ```rust
  pub fn modify_tuple(&self, value: i32, fraction: (i32, i32)) -> i32
  ```

- [ ] `modify_f` (line 3110)
  ```rust
  pub fn modify_f(&self, value: i32, multiplier: f64) -> i32
  ```

- [ ] `event_modifier` (line 3120)
  ```rust
  pub fn event_modifier(&self) -> i32
  ```

- [ ] `tie` (line 3131)
  ```rust
  pub fn tie(&mut self) -> bool
  ```

- [ ] `win` (line 3164)
  ```rust
  pub fn win(&mut self, side: Option<SideID>) -> bool
  ```

- [ ] `lose` (line 3246)
  ```rust
  pub fn lose(&mut self, side_id: SideID)
  ```

- [ ] `force_win` (line 3313)
  ```rust
  pub fn force_win(&mut self, side: Option<SideID>) -> bool
  ```

- [ ] `set_active_move` (line 3331)
  ```rust
  pub fn set_active_move(&mut self, move_id: Option<ID>, pokemon: Option<(usize, usize)>, target: Option<(usize, usize)>)
  ```

- [ ] `clear_active_move` (line 3351)
  ```rust
  pub fn clear_active_move(&mut self, failed: bool)
  ```

- [ ] `suppressing_ability` (line 3371)
  ```rust
  pub fn suppressing_ability(&self, target: Option<(usize, usize)>) -> bool
  ```

- [ ] `get_all_pokemon` (line 3425)
  ```rust
  pub fn get_all_pokemon(&self) -> Vec<&crate::pokemon::Pokemon>
  ```

- [ ] `get_random_target` (line 3473)
  ```rust
  pub fn get_random_target(&mut self, user_side: usize, user_idx: usize, move_target: &str) -> Option<(usize, usize)>
  ```

- [ ] `update_speed` (line 3571)
  ```rust
  pub fn update_speed(&mut self)
  ```

- [ ] `damage` (line 3607)
  ```rust
  pub fn damage( &mut self, damage: i32, target: Option<(usize, usize)>, source: Option<(usize, usize)>, effect: Option<&ID>, instafaint: bool, ) -> Option<i32>
  ```

- [ ] `direct_damage` (line 3702)
  ```rust
  pub fn direct_damage( &mut self, mut damage: i32, target: Option<(usize, usize)>, source: Option<(usize, usize)>, effect: Option<&ID>, ) -> i32
  ```

- [ ] `heal` (line 3926)
  ```rust
  pub fn heal( &mut self, mut damage: i32, target: Option<(usize, usize)>, source: Option<(usize, usize)>, effect: Option<&ID>, ) -> Option<i32>
  ```

- [ ] `boost` (line 4158)
  ```rust
  pub fn boost(&mut self, boosts: &[(&str, i8)], target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>) -> bool
  ```

- [ ] `faint` (line 4301)
  ```rust
  pub fn faint(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>)
  ```

- [ ] `check_fainted` (line 4340)
  ```rust
  pub fn check_fainted(&mut self)
  ```

- [ ] `clamp_int_range` (line 4353)
  ```rust
  pub fn clamp_int_range(&self, num: i32, min: i32, max: i32) -> i32
  ```

- [ ] `compare_priority` (line 4385)
  ```rust
  pub fn compare_priority(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering
  ```

- [ ] `compare_redirect_order` (line 4424)
  ```rust
  pub fn compare_redirect_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering
  ```

- [ ] `compare_left_to_right_order` (line 4450)
  ```rust
  pub fn compare_left_to_right_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering
  ```

- [ ] `speed_sort` (line 4504)
  ```rust
  pub fn speed_sort<T, F>(&mut self, list: &mut [T], mut get_priority: F) where F: FnMut(&T) -> PriorityItem,
  ```

- [ ] `get_pokemon` (line 4569)
  ```rust
  pub fn get_pokemon(&self, fullname: &str) -> Option<(usize, usize, &crate::pokemon::Pokemon)>
  ```

- [ ] `get_pokemon_mut` (line 4582)
  ```rust
  pub fn get_pokemon_mut(&mut self, fullname: &str) -> Option<(usize, usize)>
  ```

- [ ] `can_switch` (line 4601)
  ```rust
  pub fn can_switch(&self, side_idx: usize) -> usize
  ```

- [ ] `get_random_switchable` (line 4613)
  ```rust
  pub fn get_random_switchable(&mut self, side_idx: usize) -> Option<usize>
  ```

- [ ] `get_loc_of` (line 4667)
  ```rust
  pub fn get_loc_of(&self, source: (usize, usize), target: (usize, usize)) -> i32
  ```

- [ ] `valid_target_loc` (line 4728)
  ```rust
  pub fn valid_target_loc(&self, target_loc: i32, source: (usize, usize), target_type: &str) -> bool
  ```

- [ ] `valid_target` (line 4810)
  ```rust
  pub fn valid_target(&self, target: (usize, usize), source: (usize, usize), target_type: &str) -> bool
  ```

- [ ] `get_at_slot` (line 4822)
  ```rust
  pub fn get_at_slot(&self, slot: Option<&str>) -> Option<&Pokemon>
  ```

- [ ] `end_turn` (line 5036)
  ```rust
  pub fn end_turn(&mut self)
  ```

- [ ] `turn_loop` (line 5153)
  ```rust
  pub fn turn_loop(&mut self)
  ```

- [ ] `run_action` (line 5489)
  ```rust
  pub fn run_action(&mut self, action: &crate::battle_queue::Action)
  ```

- [ ] `all_choices_done` (line 5605)
  ```rust
  pub fn all_choices_done(&mut self) -> bool
  ```

- [ ] `check_move_makes_contact` (line 5645)
  ```rust
  pub fn check_move_makes_contact(&self, move_id: &ID, attacker: (usize, usize)) -> bool
  ```

- [ ] `get_action_speed` (line 5711)
  ```rust
  pub fn get_action_speed(&mut self, action: &mut crate::battle_queue::Action)
  ```

- [ ] `swap_position` (line 5816)
  ```rust
  pub fn swap_position(&mut self, pokemon: (usize, usize), new_position: usize, attributes: Option<&str>) -> bool
  ```

- [ ] `get_category` (line 5913)
  ```rust
  pub fn get_category(&self, move_id: &ID) -> String
  ```

- [ ] `clear_request` (line 5931)
  ```rust
  pub fn clear_request(&mut self)
  ```

- [ ] `make_request` (line 5979)
  ```rust
  pub fn make_request(&mut self, request_type: Option<BattleRequestState>)
  ```

- [ ] `maybe_trigger_endless_battle_clause` (line 6128)
  ```rust
  pub fn maybe_trigger_endless_battle_clause(&mut self) -> bool
  ```

- [ ] `restart` (line 6177)
  ```rust
  pub fn restart(&mut self)
  ```

- [ ] `reset_rng` (line 6197)
  ```rust
  pub fn reset_rng(&mut self, seed: Option<crate::prng::PRNGSeed>)
  ```

- [ ] `join` (line 6218)
  ```rust
  pub fn join( &mut self, slot: SideID, name: &str, avatar: &str, team: Option<Vec<crate::pokemon::Pokemon>>, ) -> Option<usize>
  ```

- [ ] `destroy` (line 6283)
  ```rust
  pub fn destroy(&mut self)
  ```

- [ ] `single_event` (line 6387)
  ```rust
  pub fn single_event( &mut self, event_id: &str, effect_id: &ID, target: Option<(usize, usize)>, source: Option<(usize, usize)>, _source_effect: Option<&ID>, ) -> crate::event::EventResult
  ```

- [ ] `run_event` (line 7329)
  ```rust
  pub fn run_event( &mut self, event_id: &str, target: Option<(usize, usize)>, source: Option<(usize, usize)>, source_effect: Option<&ID>, relay_var: Option<i32>, ) -> Option<i32>
  ```

- [ ] `run_event_bool` (line 7404)
  ```rust
  pub fn run_event_bool( &mut self, event_id: &str, target: Option<(usize, usize)>, source: Option<(usize, usize)>, source_effect: Option<&ID>, ) -> bool
  ```

- [ ] `priority_event` (line 7604)
  ```rust
  pub fn priority_event( &mut self, event_id: &str, target: Option<(usize, usize)>, source: Option<(usize, usize)>, effect: Option<&ID>, ) -> Option<i32>
  ```

- [ ] `get_event_modifier` (line 7616)
  ```rust
  pub fn get_event_modifier(&self) -> i32
  ```

- [ ] `set_event_modifier` (line 7621)
  ```rust
  pub fn set_event_modifier(&mut self, modifier: i32)
  ```

- [ ] `randomizer` (line 7643)
  ```rust
  pub fn randomizer(&mut self, base_damage: i32) -> i32
  ```

- [ ] `each_event` (line 7671)
  ```rust
  pub fn each_event(&mut self, event_id: &str, effect: Option<&ID>)
  ```

- [ ] `get_target` (line 7757)
  ```rust
  pub fn get_target( &mut self, user: (usize, usize), move_id: &ID, target_loc: i8, original_target: Option<(usize, usize)>, ) -> Option<(usize, usize)>
  ```

- [ ] `undo_choice` (line 7904)
  ```rust
  pub fn undo_choice(&mut self, side_id: SideID) -> bool
  ```

- [ ] `spread_damage` (line 8041)
  ```rust
  pub fn spread_damage( &mut self, damages: &[Option<i32>],  // Can be true (max damage), false, number, or undefined targets: &[Option<(usize, usize)>], source: Option<(usize, usize)>, effect: Option<&ID>, instafaint: bool, ) -> Vec<Option<i32>>
  ```

- [ ] `final_modify` (line 8302)
  ```rust
  pub fn final_modify(&mut self, value: i32) -> i32
  ```

- [ ] `add_split` (line 8340)
  ```rust
  pub fn add_split(&mut self, side_id: &str, secret: &[&str], shared: Option<&[&str]>)
  ```

- [ ] `attr_last_move` (line 8382)
  ```rust
  pub fn attr_last_move(&mut self, args: &[&str])
  ```

- [ ] `chain_modify` (line 8432)
  ```rust
  pub fn chain_modify(&mut self, numerator: i32, denominator: i32)
  ```

- [ ] `check_ev_balance` (line 8468)
  ```rust
  pub fn check_ev_balance(&mut self)
  ```

- [ ] `clear_effect_state` (line 8514)
  ```rust
  pub fn clear_effect_state(&mut self, target: (usize, usize), effect_id: &ID)
  ```

- [ ] `debug_error` (line 8530)
  ```rust
  pub fn debug_error(&mut self, activity: &str)
  ```

- [ ] `field_event` (line 8629)
  ```rust
  pub fn field_event(&mut self, event_id: &str)
  ```

- [ ] `get_callback` (line 8669)
  ```rust
  pub fn get_callback(&self, _effect_id: &ID, _event_id: &str) -> Option<String>
  ```

- [ ] `get_overflowed_turn_count` (line 8688)
  ```rust
  pub fn get_overflowed_turn_count(&self) -> i32
  ```

- [ ] `get_requests` (line 8749)
  ```rust
  pub fn get_requests(&self) -> Vec<serde_json::Value>
  ```

- [ ] `get_team` (line 8788)
  ```rust
  pub fn get_team(&self, side_idx: usize) -> Option<&[crate::pokemon::Pokemon]>
  ```

- [ ] `resolve_priority` (line 8868)
  ```rust
  pub fn resolve_priority(&mut self, handler: &mut EventListener, callback_name: &str)
  ```

- [ ] `retarget_last_move` (line 8961)
  ```rust
  pub fn retarget_last_move(&mut self, new_target: (usize, usize))
  ```

- [ ] `run_pick_team` (line 9027)
  ```rust
  pub fn run_pick_team(&mut self)
  ```

- [ ] `send_updates` (line 9092)
  ```rust
  pub fn send_updates(&mut self)
  ```

- [ ] `show_open_team_sheets` (line 9143)
  ```rust
  pub fn show_open_team_sheets(&mut self, _side_idx: Option<usize>)
  ```

- [ ] `spread_modify` (line 9161)
  ```rust
  pub fn spread_modify(&self, base_stats: &StatsTable, set: &PokemonSet) -> StatsTable
  ```

- [ ] `stat_modify` (line 9196)
  ```rust
  pub fn stat_modify(&self, base_stats: &StatsTable, set: &PokemonSet, stat_name: &str) -> i32
  ```

- [ ] `tiebreak` (line 9302)
  ```rust
  pub fn tiebreak(&mut self) -> Option<usize>
  ```

- [ ] `to_json` (line 9441)
  ```rust
  pub fn to_json(&self) -> serde_json::Value
  ```

- [ ] `to_string` (line 9453)
  ```rust
  pub fn to_string(&self) -> String
  ```

- [ ] `find_battle_event_handlers` (line 9488)
  ```rust
  pub fn find_battle_event_handlers(&self, event_id: &str) -> Vec<ID>
  ```

- [ ] `find_field_event_handlers` (line 9532)
  ```rust
  pub fn find_field_event_handlers(&self, event_id: &str) -> Vec<(ID, Option<(usize, usize)>)>
  ```

- [ ] `find_side_event_handlers` (line 9574)
  ```rust
  pub fn find_side_event_handlers(&self, _event_id: &str, side_idx: usize) -> Vec<(ID, Option<(usize, usize)>)>
  ```

- [ ] `find_pokemon_event_handlers` (line 9652)
  ```rust
  pub fn find_pokemon_event_handlers(&self, _event_id: &str, target: (usize, usize)) -> Vec<(ID, Option<(usize, usize)>)>
  ```

- [ ] `get_damage` (line 9714)
  ```rust
  pub fn get_damage( &mut self, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &ID, ) -> Option<i32>
  ```

- [ ] `try_spread_move_hit` (line 9897)
  ```rust
  pub fn try_spread_move_hit( &mut self, targets: &[(usize, usize)], pokemon_pos: (usize, usize), move_id: &ID, ) -> bool
  ```

- [ ] `on_event` (line 10230)
  ```rust
  pub fn on_event<F>(&mut self, event_id: &str, callback: F) where F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
  ```

- [ ] `on_event_priority` (line 10244)
  ```rust
  pub fn on_event_priority<F>(&mut self, event_id: &str, priority: i32, callback: F) where F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
  ```

### Private Methods

- [ ] `fmt` (line 99)
  ```rust
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  ```

- [ ] `from` (line 110)
  ```rust
  fn from(p: &'a Pokemon) -> Self
  ```

- [ ] `from` (line 116)
  ```rust
  fn from(s: &'a str) -> Self
  ```

- [ ] `from` (line 122)
  ```rust
  fn from(s: String) -> Self
  ```

- [ ] `fmt` (line 128)
  ```rust
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  ```

- [ ] `default` (line 177)
  ```rust
  fn default() -> Self
  ```

- [ ] `from_event_info` (line 211)
  ```rust
  fn from_event_info(event_id: &str, event_info: &EventInfo, relay_var: Option<i32>) -> Self
  ```

- [ ] `minimal` (line 223)
  ```rust
  fn minimal(event_id: &str) -> Self
  ```

- [ ] `each_event_internal` (line 1038)
  ```rust
  fn each_event_internal(&mut self, event_name: &str)
  ```

- [ ] `get_foe_pokemon_left` (line 1199)
  ```rust
  fn get_foe_pokemon_left(&self, side_idx: usize) -> usize
  ```

- [ ] `validate_single_choice` (line 1322)
  ```rust
  fn validate_single_choice(&mut self, side_id: SideID, choice: &str, pokemon_index: Option<usize>) -> Result<(), String>
  ```

- [ ] `parse_choice` (line 1650)
  ```rust
  fn parse_choice(&mut self, side_id: SideID, choice: &str)
  ```

- [ ] `commit_choices` (line 1815)
  ```rust
  fn commit_choices(&mut self)
  ```

- [ ] `find_valid_switch_target` (line 1946)
  ```rust
  fn find_valid_switch_target(&self, side_idx: usize, current_poke_idx: usize) -> Option<usize>
  ```

- [ ] `do_switch` (line 1957)
  ```rust
  fn do_switch(&mut self, side_idx: usize, slot: usize, switch_to: usize)
  ```

- [ ] `do_switch_with_drag` (line 2043)
  ```rust
  fn do_switch_with_drag(&mut self, side_idx: usize, slot: usize, switch_to: usize, is_drag: bool)
  ```

- [ ] `field_event_switch_in` (line 2118)
  ```rust
  fn field_event_switch_in(&mut self, switchers: &[(usize, usize)])
  ```

- [ ] `trigger_switch_in_abilities` (line 2127)
  ```rust
  fn trigger_switch_in_abilities(&mut self, side_idx: usize, poke_idx: usize)
  ```

- [ ] `apply_hazards` (line 2137)
  ```rust
  fn apply_hazards(&mut self, side_idx: usize, _slot: usize, poke_idx: usize)
  ```

- [ ] `run_move` (line 2206)
  ```rust
  fn run_move(&mut self, side_idx: usize, poke_idx: usize, move_id: &ID, target_loc: i8)
  ```

- [ ] `get_move_target` (line 2249)
  ```rust
  fn get_move_target(&self, side_idx: usize, target_loc: i8) -> (usize, usize)
  ```

- [ ] `get_move_priority` (line 2273)
  ```rust
  fn get_move_priority(&self, move_id: &ID) -> i8
  ```

- [ ] `get_multi_hit_count` (line 2283)
  ```rust
  fn get_multi_hit_count(&mut self, move_id: &ID) -> i32
  ```

- [ ] `get_move_accuracy` (line 2315)
  ```rust
  fn get_move_accuracy(&self, move_id: &ID) -> i32
  ```

- [ ] `apply_confusion` (line 2328)
  ```rust
  fn apply_confusion(&mut self, side_idx: usize, poke_idx: usize)
  ```

- [ ] `remove_all_hazards` (line 2354)
  ```rust
  fn remove_all_hazards(&mut self, side_idx: usize)
  ```

- [ ] `apply_status` (line 2378)
  ```rust
  fn apply_status(&mut self, side_idx: usize, poke_idx: usize, status: &str)
  ```

- [ ] `apply_boost` (line 2489)
  ```rust
  fn apply_boost(&mut self, side_idx: usize, poke_idx: usize, stat: &str, amount: i8)
  ```

- [ ] `run_residual` (line 2531)
  ```rust
  fn run_residual(&mut self)
  ```

- [ ] `faint_messages` (line 2637)
  ```rust
  fn faint_messages(&mut self, last_first: bool, force_check: bool, mut check_win: bool) -> bool
  ```

- [ ] `next_turn` (line 2798)
  ```rust
  fn next_turn(&mut self)
  ```

- [ ] `add_direct_damage_log` (line 3845)
  ```rust
  fn add_direct_damage_log(&mut self, target: (usize, usize), effect: Option<&ID>)
  ```

- [ ] `add_heal_log` (line 4024)
  ```rust
  fn add_heal_log(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&ID>)
  ```

- [ ] `boost_stats` (line 4284)
  ```rust
  fn boost_stats(&mut self, target_side: usize, target_idx: usize, boosts_map: &HashMap<String, i32>)
  ```

- [ ] `shuffle_range` (line 4545)
  ```rust
  fn shuffle_range<T>(&mut self, list: &mut [T], start: usize, end: usize)
  ```

- [ ] `possible_switches` (line 4637)
  ```rust
  fn possible_switches(&self, side_idx: usize) -> Vec<usize>
  ```

- [ ] `get_pokemon_action_speed` (line 5775)
  ```rust
  fn get_pokemon_action_speed(&self, side_idx: usize, poke_idx: usize) -> i32
  ```

- [ ] `get_effect_type` (line 6510)
  ```rust
  fn get_effect_type(&self, effect_id: &ID) -> &str
  ```

- [ ] `dispatch_single_event` (line 6545)
  ```rust
  fn dispatch_single_event( &mut self, event_id: &str, effect_id: &ID, target: Option<(usize, usize)>, _source: Option<(usize, usize)>, ) -> crate::event::EventResult
  ```

- [ ] `handle_ability_event` (line 6580)
  ```rust
  fn handle_ability_event( &mut self, event_id: &str, ability_id: &ID, target: Option<(usize, usize)>, ) -> crate::event::EventResult
  ```

- [ ] `handle_item_event` (line 6732)
  ```rust
  fn handle_item_event( &mut self, event_id: &str, item_id: &ID, target: Option<(usize, usize)>, ) -> crate::event::EventResult
  ```

- [ ] `handle_move_event` (line 6835)
  ```rust
  fn handle_move_event( &mut self, event_id: &str, move_id: &str, target: Option<(usize, usize)>, ) -> crate::event::EventResult
  ```

- [ ] `check_volatile_try_hit` (line 6914)
  ```rust
  fn check_volatile_try_hit( &mut self, target: (usize, usize), source: (usize, usize), move_id: &ID, ) -> bool
  ```

- [ ] `handle_condition_event` (line 6951)
  ```rust
  fn handle_condition_event( &mut self, event_id: &str, condition_id: &str, target: Option<(usize, usize)>, ) -> crate::event::EventResult
  ```

- [ ] `handle_side_condition_event` (line 7017)
  ```rust
  fn handle_side_condition_event( &mut self, event_id: &str, side_idx: usize, condition_id: &ID, )
  ```

- [ ] `find_event_handlers` (line 7491)
  ```rust
  fn find_event_handlers( &self, event_id: &str, target: Option<(usize, usize)>, source: Option<(usize, usize)>, ) -> Vec<(ID, Option<(usize, usize)>)>
  ```

- [ ] `get_at_loc` (line 7846)
  ```rust
  fn get_at_loc(&self, source: (usize, usize), target_loc: i8) -> Option<(usize, usize)>
  ```

- [ ] `add_damage_log` (line 8245)
  ```rust
  fn add_damage_log(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&ID>)
  ```

- [ ] `modify_internal` (line 8315)
  ```rust
  fn modify_internal(&self, value: i32, modifier: i32) -> i32
  ```

- [ ] `modify_damage` (line 9841)
  ```rust
  fn modify_damage( &mut self, mut base_damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_data: &crate::dex::MoveData, ) -> i32
  ```

- [ ] `spread_move_hit` (line 9930)
  ```rust
  fn spread_move_hit( &mut self, targets: &[Option<(usize, usize)>], source_pos: (usize, usize), move_id: &ID, is_secondary: bool, is_self: bool, ) -> (Vec<Option<i32>>, Vec<Option<(usize, usize)>>)
  ```

- [ ] `get_spread_damage` (line 9995)
  ```rust
  fn get_spread_damage( &mut self, damages: &[Option<i32>], targets: &[Option<(usize, usize)>], source_pos: (usize, usize), move_id: &ID, _is_secondary: bool, _is_self: bool, ) -> Vec<Option<i32>>
  ```

- [ ] `run_move_effects` (line 10021)
  ```rust
  fn run_move_effects( &mut self, damages: &[Option<i32>], targets: &[Option<(usize, usize)>], _source_pos: (usize, usize), move_data: &crate::dex::MoveData, _is_secondary: bool, _is_self: bool, ) -> Vec<Option<i32>>
  ```

- [ ] `game_type_to_string` (line 10147)
  ```rust
  fn game_type_to_string(game_type: &GameType) -> String
  ```

- [ ] `run_custom_event_handlers` (line 10271)
  ```rust
  fn run_custom_event_handlers(&mut self, event_name: &str) -> Option<i32>
  ```

- [ ] `create_test_team` (line 10326)
  ```rust
  fn create_test_team() -> Vec<PokemonSet>
  ```

- [ ] `test_battle_creation` (line 10348)
  ```rust
  fn test_battle_creation()
  ```

- [ ] `test_battle_with_players` (line 10360)
  ```rust
  fn test_battle_with_players()
  ```

- [ ] `test_battle_start` (line 10385)
  ```rust
  fn test_battle_start()
  ```

- [ ] `test_battle_prng_deterministic` (line 10412)
  ```rust
  fn test_battle_prng_deterministic()
  ```

- [ ] `fmt` (line 10439)
  ```rust
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  ```

---

## battle_actions.rs

**Total methods:** 76 (63 public, 13 private)

### Public Methods

- [ ] `get_max_move_name` (line 29)
  ```rust
  pub fn get_max_move_name(move_type: &str) -> &'static str
  ```

- [ ] `get_z_move_name` (line 55)
  ```rust
  pub fn get_z_move_name(move_type: &str) -> &'static str
  ```

- [ ] `new` (line 268)
  ```rust
  pub fn new(dex: &'a Dex, gen: u8) -> Self
  ```

- [ ] `calculate_damage` (line 275)
  ```rust
  pub fn calculate_damage( &self, attacker: &Pokemon, defender: &Pokemon, move_data: &MoveData, is_crit: bool, random_factor: i32, ) -> DamageResult
  ```

- [ ] `get_boost_modifier` (line 363)
  ```rust
  pub fn get_boost_modifier(boost: i8) -> (i32, i32)
  ```

- [ ] `calculate_stat_with_boost` (line 384)
  ```rust
  pub fn calculate_stat_with_boost(base_stat: i32, boost: i8) -> i32
  ```

- [ ] `calc_recoil_damage` (line 399)
  ```rust
  pub fn calc_recoil_damage(damage_dealt: i32, move_id: &str, recoil: Option<(i32, i32)>, pokemon_max_hp: i32) -> i32
  ```

- [ ] `get_confusion_damage` (line 433)
  ```rust
  pub fn get_confusion_damage(level: i32, attack: i32, defense: i32, base_power: i32, random_factor: i32) -> i32
  ```

- [ ] `target_type_choices` (line 450)
  ```rust
  pub fn target_type_choices(target_type: &str) -> bool
  ```

- [ ] `combine_results` (line 475)
  ```rust
  pub fn combine_results(left: Option<DamageValue>, right: Option<DamageValue>) -> Option<DamageValue>
  ```

- [ ] `get_z_move` (line 518)
  ```rust
  pub fn get_z_move( move_name: &str, move_type: &str, move_category: &str, z_move_base_power: Option<i32>, item_z_move: Option<&str>, item_z_move_from: Option<&str>, item_z_move_type: Option<&str>, z_move_used: bool, ) -> Option<String>
  ```

- [ ] `can_z_move` (line 591)
  ```rust
  pub fn can_z_move( z_move_used: bool, is_transformed: bool, species_is_mega: bool, species_is_primal: bool, species_forme: &str, item_z_move: bool, item_user: Option<&[String]>, species_name: &str, ) -> bool
  ```

- [ ] `get_max_move` (line 636)
  ```rust
  pub fn get_max_move(move_name: &str, move_type: &str, move_category: &str) -> Option<String>
  ```

- [ ] `can_mega_evo` (line 690)
  ```rust
  pub fn can_mega_evo( species_name: &str, species_other_formes: Option<&[String]>, item_mega_evolves: Option<&str>, item_mega_stone: Option<&str>, base_moves: &[ID], item_is_z_move: bool, gen: u8, ) -> Option<String>
  ```

- [ ] `can_ultra_burst` (line 734)
  ```rust
  pub fn can_ultra_burst(species_name: &str, item_id: &str) -> Option<String>
  ```

- [ ] `can_terastallize` (line 753)
  ```rust
  pub fn can_terastallize( item_is_z_move: bool, can_mega_evo: bool, gen: u8, tera_type: Option<&str>, ) -> Option<String>
  ```

- [ ] `hit_step_invulnerability_event` (line 795)
  ```rust
  pub fn hit_step_invulnerability_event( targets: &[(usize, usize)], attacker_flying: bool, move_flags_contact: bool, move_target: &str, ) -> Vec<(usize, usize, bool)>
  ```

- [ ] `hit_step_type_immunity` (line 824)
  ```rust
  pub fn hit_step_type_immunity( defender_types: &[String], move_type: &str, ignore_immunity: bool, ) -> bool
  ```

- [ ] `hit_step_accuracy` (line 906)
  ```rust
  pub fn hit_step_accuracy( move_accuracy: i32, move_always_hit: bool, move_ohko: bool, attacker_accuracy_boost: i8, defender_evasion_boost: i8, ignore_accuracy: bool, ignore_evasion: bool, random_value: i32, // 0-99 ) -> bool
  ```

- [ ] `hit_step_break_protect` (line 988)
  ```rust
  pub fn hit_step_break_protect( move_breaks_protect: bool, move_is_z: bool, target_protected: bool, ) -> bool
  ```

- [ ] `hit_step_steal_boosts` (line 1038)
  ```rust
  pub fn hit_step_steal_boosts( move_steals_boosts: bool, target_boosts: &BoostsTable, ) -> Option<BoostsTable>
  ```

- [ ] `self_drops` (line 1081)
  ```rust
  pub fn self_drops( move_self_boost: Option<&BoostsTable>, already_dropped: bool, ) -> Option<BoostsTable>
  ```

- [ ] `get_secondaries` (line 1093)
  ```rust
  pub fn get_secondaries( secondaries: &[SecondaryEffect], has_sheer_force: bool, ) -> Vec<SecondaryEffect>
  ```

- [ ] `get_active_z_move` (line 1132)
  ```rust
  pub fn get_active_z_move( base_move_id: &str, base_move_type: &str, base_move_category: &str, base_move_base_power: i32, z_crystal_base_power: Option<i32>, ) -> ActiveMove
  ```

- [ ] `get_active_max_move` (line 1204)
  ```rust
  pub fn get_active_max_move( base_move_id: &str, base_move_type: &str, base_move_category: &str, base_move_base_power: i32, gmax_move: Option<&str>, ) -> ActiveMove
  ```

- [ ] `get_z_power_effect` (line 1258)
  ```rust
  pub fn get_z_power_effect(z_move_effect: Option<&str>) -> Option<ZPowerEffect>
  ```

- [ ] `get_spread_damage_modifier` (line 1272)
  ```rust
  pub fn get_spread_damage_modifier(target_count: usize) -> f64
  ```

- [ ] `modify_damage` (line 1399)
  ```rust
  pub fn modify_damage( base_damage: i32, modifiers: &[f64], ) -> i32
  ```

- [ ] `should_force_switch` (line 1412)
  ```rust
  pub fn should_force_switch( move_force_switch: bool, target_hp: i32, target_is_dynamaxed: bool, ) -> bool
  ```

- [ ] `get_multi_hit_count` (line 1431)
  ```rust
  pub fn get_multi_hit_count(multi_hit: Option<i32>, random_value: i32) -> i32
  ```

- [ ] `is_critical_hit` (line 1452)
  ```rust
  pub fn is_critical_hit( crit_ratio: i32, attacker_focus_energy: bool, move_will_crit: Option<bool>, random_value: i32, ) -> bool
  ```

- [ ] `run_mega_evo_check` (line 1486)
  ```rust
  pub fn run_mega_evo_check( species_name: &str, mega_forme: Option<&str>, already_mega: bool, ) -> Option<String>
  ```

- [ ] `terastallize_check` (line 1500)
  ```rust
  pub fn terastallize_check( tera_type: Option<&str>, already_terastallized: bool, side_terastallize_used: bool, ) -> Option<String>
  ```

- [ ] `get_damage` (line 1663)
  ```rust
  pub fn get_damage( attacker_level: i32, attacker_attack: i32, defender_defense: i32, base_power: i32, stab_modifier: f64, type_effectiveness: f64, is_crit: bool, random_factor: i32, // 85-100 other_modifiers: &[f64], ) -> i32
  ```

- [ ] `try_spread_move_hit_check` (line 1708)
  ```rust
  pub fn try_spread_move_hit_check( target_count: usize, move_target_type: &str, ) -> bool
  ```

- [ ] `move_hit_result` (line 1723)
  ```rust
  pub fn move_hit_result( damage: i32, type_effectiveness: f64, is_crit: bool, ) -> MoveHitData
  ```

- [ ] `hit_step_try_immunity` (line 1763)
  ```rust
  pub fn hit_step_try_immunity( move_type: &str, defender_types: &[String], defender_ability: &str, ignore_immunity: bool, ) -> bool
  ```

- [ ] `hit_step_try_hit_event` (line 1804)
  ```rust
  pub fn hit_step_try_hit_event( target_has_substitute: bool, move_ignores_substitute: bool, move_is_sound: bool, ) -> bool
  ```

- [ ] `after_move_secondary_event` (line 1827)
  ```rust
  pub fn after_move_secondary_event( move_self_switch: Option<&str>, move_force_switch: bool, ) -> AfterMoveResult
  ```

- [ ] `try_move_hit_check` (line 1839)
  ```rust
  pub fn try_move_hit_check( accuracy_passed: bool, type_immunity_passed: bool, invulnerability_passed: bool, ) -> bool
  ```

- [ ] `hit_step_move_hit_loop_count` (line 1849)
  ```rust
  pub fn hit_step_move_hit_loop_count( multi_hit: Option<i32>, move_hit_type: Option<&str>, random_value: i32, ) -> i32
  ```

- [ ] `spread_move_hit_modifier` (line 1867)
  ```rust
  pub fn spread_move_hit_modifier(target_count: usize) -> f64
  ```

- [ ] `try_primary_hit_event` (line 1884)
  ```rust
  pub fn try_primary_hit_event( move_has_primary_effect: bool, move_primary_chance: Option<i32>, random_value: i32, ) -> bool
  ```

- [ ] `run_move_effects_list` (line 1902)
  ```rust
  pub fn run_move_effects_list( move_boosts: Option<&BoostsTable>, move_heal: Option<(i32, i32)>, move_status: Option<&str>, move_volatile: Option<&str>, ) -> MoveEffects
  ```

- [ ] `run_move_stub` (line 2011)
  ```rust
  pub fn run_move_stub( move_id: &ID, pokemon_index: usize, target_loc: i32, options: RunMoveOptions, ) -> RunMoveResult
  ```

- [ ] `use_move_stub` (line 2034)
  ```rust
  pub fn use_move_stub( move_id: &ID, pokemon_index: usize, options: UseMoveOptions, ) -> bool
  ```

- [ ] `use_move_inner_stub` (line 2049)
  ```rust
  pub fn use_move_inner_stub( move_id: &ID, pokemon_index: usize, options: UseMoveOptions, ) -> bool
  ```

- [ ] `try_spread_move_hit_stub` (line 2063)
  ```rust
  pub fn try_spread_move_hit_stub( target_indices: &[usize], pokemon_index: usize, move_id: &ID, not_active: bool, ) -> bool
  ```

- [ ] `hit_step_move_hit_loop_stub` (line 2077)
  ```rust
  pub fn hit_step_move_hit_loop_stub( target_indices: &[usize], pokemon_index: usize, move_id: &ID, multi_hit: Option<i32>, gen: u8, ) -> SpreadMoveDamage
  ```

- [ ] `spread_move_hit_stub` (line 2103)
  ```rust
  pub fn spread_move_hit_stub( target_indices: &[usize], pokemon_index: usize, move_id: &ID, is_secondary: bool, is_self: bool, ) -> (SpreadMoveDamage, SpreadMoveTargets)
  ```

- [ ] `move_hit_stub` (line 2128)
  ```rust
  pub fn move_hit_stub( target_index: Option<usize>, pokemon_index: usize, move_id: &ID, is_secondary: bool, is_self: bool, ) -> Option<i32>
  ```

- [ ] `get_spread_damage_stub` (line 2161)
  ```rust
  pub fn get_spread_damage_stub( damage: &mut SpreadMoveDamage, target_indices: &[usize], source_index: usize, move_id: &ID, is_secondary: bool, is_self: bool, )
  ```

- [ ] `force_switch_stub` (line 2178)
  ```rust
  pub fn force_switch_stub( damage: &mut SpreadMoveDamage, target_indices: &[usize], source_index: usize, move_id: &ID, )
  ```

- [ ] `run_z_power` (line 2235)
  ```rust
  pub fn run_z_power( move_category: &str, z_move_boost: Option<&BoostsTable>, z_move_effect: Option<&str>, pokemon_has_ghost_type: bool, ) -> ZPowerResult
  ```

- [ ] `run_mega_evo_stub` (line 2277)
  ```rust
  pub fn run_mega_evo_stub( pokemon_index: usize, can_mega_evo: Option<&str>, can_ultra_burst: Option<&str>, ) -> Option<String>
  ```

- [ ] `terastallize_stub` (line 2298)
  ```rust
  pub fn terastallize_stub( pokemon_index: usize, tera_type: &str, species_base_species: &str, ) -> TerastallizeResult
  ```

- [ ] `try_move_hit_stub` (line 2326)
  ```rust
  pub fn try_move_hit_stub( target_or_targets: &[usize], pokemon_index: usize, move_id: &ID, ) -> Option<i32>
  ```

- [ ] `secondaries_stub` (line 2357)
  ```rust
  pub fn secondaries_stub( target_indices: &[usize], source_index: usize, move_id: &ID, is_self: bool, )
  ```

- [ ] `switch_in_stub` (line 2386)
  ```rust
  pub fn switch_in_stub( pokemon_index: usize, side_index: usize, pos: usize, source_effect: Option<&ID>, is_drag: bool, ) -> SwitchInResult
  ```

- [ ] `drag_in_stub` (line 2406)
  ```rust
  pub fn drag_in_stub( side_index: usize, pos: usize, ) -> bool
  ```

- [ ] `run_switch_stub` (line 2421)
  ```rust
  pub fn run_switch_stub( pokemon_index: usize, ) -> bool
  ```

- [ ] `use_move` (line 2509)
  ```rust
  pub fn use_move( battle: &mut crate::battle::Battle, move_id: &ID, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, source_effect: Option<&ID>, z_move: Option<&str>, max_move: Option<&str>, ) -> bool
  ```

- [ ] `use_move_inner` (line 2707)
  ```rust
  pub fn use_move_inner( battle: &mut crate::battle::Battle, move_or_move_name: &ID, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, source_effect_param: Option<&ID>, z_move_param: Option<&str>, max_move_param: Option<&str>, ) -> bool
  ```

### Private Methods

- [ ] `get_accuracy_modifier` (line 939)
  ```rust
  fn get_accuracy_modifier(stages: i8) -> (i32, i32)
  ```

- [ ] `z_move_power_table` (line 1164)
  ```rust
  fn z_move_power_table(base_power: i32) -> i32
  ```

- [ ] `max_move_power_table` (line 1241)
  ```rust
  fn max_move_power_table(base_power: i32, move_type: &str) -> i32
  ```

- [ ] `default` (line 1985)
  ```rust
  fn default() -> Self
  ```

- [ ] `test_boost_modifier` (line 3028)
  ```rust
  fn test_boost_modifier()
  ```

- [ ] `test_stat_with_boost` (line 3038)
  ```rust
  fn test_stat_with_boost()
  ```

- [ ] `test_recoil_damage` (line 3048)
  ```rust
  fn test_recoil_damage()
  ```

- [ ] `test_confusion_damage` (line 3060)
  ```rust
  fn test_confusion_damage()
  ```

- [ ] `test_target_type_choices` (line 3067)
  ```rust
  fn test_target_type_choices()
  ```

- [ ] `test_max_move_name` (line 3076)
  ```rust
  fn test_max_move_name()
  ```

- [ ] `test_z_move_name` (line 3084)
  ```rust
  fn test_z_move_name()
  ```

- [ ] `test_can_ultra_burst` (line 3091)
  ```rust
  fn test_can_ultra_burst()
  ```

- [ ] `test_can_terastallize` (line 3107)
  ```rust
  fn test_can_terastallize()
  ```

---

## battle_queue.rs

**Total methods:** 45 (43 public, 2 private)

### Public Methods

- [ ] `order` (line 158)
  ```rust
  pub fn order(&self) -> i32
  ```

- [ ] `priority` (line 188)
  ```rust
  pub fn priority(&self) -> i8
  ```

- [ ] `speed` (line 199)
  ```rust
  pub fn speed(&self) -> i32
  ```

- [ ] `fractional_priority` (line 210)
  ```rust
  pub fn fractional_priority(&self) -> f64
  ```

- [ ] `pokemon_index` (line 218)
  ```rust
  pub fn pokemon_index(&self) -> Option<usize>
  ```

- [ ] `side_index` (line 229)
  ```rust
  pub fn side_index(&self) -> Option<usize>
  ```

- [ ] `is_move` (line 240)
  ```rust
  pub fn is_move(&self) -> bool
  ```

- [ ] `is_switch` (line 245)
  ```rust
  pub fn is_switch(&self) -> bool
  ```

- [ ] `is_run_switch` (line 250)
  ```rust
  pub fn is_run_switch(&self) -> bool
  ```

- [ ] `get_switch_target` (line 255)
  ```rust
  pub fn get_switch_target(&self) -> Option<(usize, usize)>
  ```

- [ ] `new` (line 272)
  ```rust
  pub fn new() -> Self
  ```

- [ ] `shift` (line 284)
  ```rust
  pub fn shift(&mut self) -> Option<Action>
  ```

- [ ] `peek` (line 297)
  ```rust
  pub fn peek(&self) -> Option<&Action>
  ```

- [ ] `peek_end` (line 302)
  ```rust
  pub fn peek_end(&self) -> Option<&Action>
  ```

- [ ] `push` (line 311)
  ```rust
  pub fn push(&mut self, action: Action)
  ```

- [ ] `unshift` (line 320)
  ```rust
  pub fn unshift(&mut self, action: Action)
  ```

- [ ] `len` (line 325)
  ```rust
  pub fn len(&self) -> usize
  ```

- [ ] `is_empty` (line 330)
  ```rust
  pub fn is_empty(&self) -> bool
  ```

- [ ] `clear` (line 340)
  ```rust
  pub fn clear(&mut self)
  ```

- [ ] `cancel_action` (line 357)
  ```rust
  pub fn cancel_action(&mut self, side_index: usize, pokemon_index: usize) -> bool
  ```

- [ ] `cancel_move` (line 377)
  ```rust
  pub fn cancel_move(&mut self, side_index: usize, pokemon_index: usize) -> bool
  ```

- [ ] `will_move` (line 403)
  ```rust
  pub fn will_move(&self, side_index: usize, pokemon_index: usize) -> Option<&MoveAction>
  ```

- [ ] `will_move_pokemon` (line 416)
  ```rust
  pub fn will_move_pokemon(&self, pokemon: &crate::pokemon::Pokemon) -> Option<&MoveAction>
  ```

- [ ] `will_switch` (line 431)
  ```rust
  pub fn will_switch(&self, side_index: usize, pokemon_index: usize) -> Option<&SwitchAction>
  ```

- [ ] `will_act` (line 453)
  ```rust
  pub fn will_act(&self) -> bool
  ```

- [ ] `insert_run_switch` (line 461)
  ```rust
  pub fn insert_run_switch(&mut self, side_index: usize, pokemon_index: usize)
  ```

- [ ] `insert_choice` (line 517)
  ```rust
  pub fn insert_choice(&mut self, action: Action)
  ```

- [ ] `sort` (line 530)
  ```rust
  pub fn sort(&mut self)
  ```

- [ ] `prioritize_action` (line 573)
  ```rust
  pub fn prioritize_action(&mut self, side_index: usize, pokemon_index: usize) -> bool
  ```

- [ ] `iter` (line 588)
  ```rust
  pub fn iter(&self) -> impl Iterator<Item = &Action>
  ```

- [ ] `iter_mut` (line 593)
  ```rust
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Action>
  ```

- [ ] `will_act_full` (line 602)
  ```rust
  pub fn will_act_full(&self) -> Option<&Action>
  ```

- [ ] `change_action` (line 628)
  ```rust
  pub fn change_action(&mut self, side_index: usize, pokemon_index: usize, new_action: Action)
  ```

- [ ] `insert_in_order` (line 635)
  ```rust
  pub fn insert_in_order(&mut self, action: Action)
  ```

- [ ] `add_choice` (line 690)
  ```rust
  pub fn add_choice(&mut self, action: Action)
  ```

- [ ] `debug` (line 705)
  ```rust
  pub fn debug(&self) -> String
  ```

- [ ] `entries` (line 737)
  ```rust
  pub fn entries(&self) -> impl Iterator<Item = (usize, &Action)>
  ```

- [ ] `entries_mut` (line 742)
  ```rust
  pub fn entries_mut(&mut self) -> impl Iterator<Item = (usize, &mut Action)>
  ```

- [ ] `find` (line 747)
  ```rust
  pub fn find<F>(&self, predicate: F) -> Option<&Action> where F: Fn(&Action) -> bool,
  ```

- [ ] `remove_where` (line 755)
  ```rust
  pub fn remove_where<F>(&mut self, predicate: F) -> Vec<Action> where F: Fn(&Action) -> bool,
  ```

- [ ] `prioritize_action_ref` (line 772)
  ```rust
  pub fn prioritize_action_ref(&mut self, action: &Action) -> bool
  ```

- [ ] `resolve_action` (line 912)
  ```rust
  pub fn resolve_action(&self, action: &mut Action, mid_turn: bool) -> Vec<Action>
  ```

- [ ] `get_order_for_choice` (line 1008)
  ```rust
  pub fn get_order_for_choice(choice: &str) -> i32
  ```

### Private Methods

- [ ] `test_queue_basic_operations` (line 1034)
  ```rust
  fn test_queue_basic_operations()
  ```

- [ ] `test_queue_sorting` (line 1051)
  ```rust
  fn test_queue_sorting()
  ```

---

## battle_stream.rs

**Total methods:** 30 (17 public, 13 private)

### Public Methods

- [ ] `parse` (line 125)
  ```rust
  pub fn parse(line: &str) -> Option<Self>
  ```

- [ ] `to_protocol` (line 386)
  ```rust
  pub fn to_protocol(&self) -> String
  ```

- [ ] `new` (line 513)
  ```rust
  pub fn new() -> Self
  ```

- [ ] `with_options` (line 527)
  ```rust
  pub fn with_options(options: BattleStreamOptions) -> Self
  ```

- [ ] `with_battle` (line 540)
  ```rust
  pub fn with_battle(battle: Battle) -> Self
  ```

- [ ] `start` (line 562)
  ```rust
  pub fn start(&mut self, options: BattleOptions)
  ```

- [ ] `write` (line 572)
  ```rust
  pub fn write(&mut self, input: &str)
  ```

- [ ] `push_message` (line 728)
  ```rust
  pub fn push_message(&mut self, msg_type: &str, data: &str)
  ```

- [ ] `read` (line 752)
  ```rust
  pub fn read(&mut self) -> Option<String>
  ```

- [ ] `battle` (line 770)
  ```rust
  pub fn battle(&self) -> Option<&Battle>
  ```

- [ ] `battle_mut` (line 775)
  ```rust
  pub fn battle_mut(&mut self) -> Option<&mut Battle>
  ```

- [ ] `ended` (line 780)
  ```rust
  pub fn ended(&self) -> bool
  ```

- [ ] `winner` (line 785)
  ```rust
  pub fn winner(&self) -> Option<String>
  ```

- [ ] `destroy` (line 791)
  ```rust
  pub fn destroy(&mut self)
  ```

- [ ] `new` (line 821)
  ```rust
  pub fn new() -> Self
  ```

- [ ] `push_update` (line 834)
  ```rust
  pub fn push_update(&mut self, data: &str, channels: &[i8])
  ```

- [ ] `split_first` (line 884)
  ```rust
  pub fn split_first(s: &str, delimiter: &str, limit: usize) -> Vec<String>
  ```

### Private Methods

- [ ] `write_line` (line 586)
  ```rust
  fn write_line(&mut self, cmd: &str, args: &str)
  ```

- [ ] `default` (line 850)
  ```rust
  fn default() -> Self
  ```

- [ ] `default` (line 903)
  ```rust
  fn default() -> Self
  ```

- [ ] `test_parse_turn` (line 913)
  ```rust
  fn test_parse_turn()
  ```

- [ ] `test_parse_switch` (line 919)
  ```rust
  fn test_parse_switch()
  ```

- [ ] `test_parse_move` (line 930)
  ```rust
  fn test_parse_move()
  ```

- [ ] `test_parse_damage` (line 941)
  ```rust
  fn test_parse_damage()
  ```

- [ ] `test_parse_boost` (line 947)
  ```rust
  fn test_parse_boost()
  ```

- [ ] `test_parse_weather` (line 957)
  ```rust
  fn test_parse_weather()
  ```

- [ ] `test_parse_status` (line 963)
  ```rust
  fn test_parse_status()
  ```

- [ ] `test_to_protocol` (line 972)
  ```rust
  fn test_to_protocol()
  ```

- [ ] `test_battle_stream_creation` (line 985)
  ```rust
  fn test_battle_stream_creation()
  ```

- [ ] `test_parse_terastallize` (line 992)
  ```rust
  fn test_parse_terastallize()
  ```

---

## Summary Statistics

- **battle.rs**: 186 methods (125 public, 61 private)
- **battle_actions.rs**: 76 methods (63 public, 13 private)
- **battle_queue.rs**: 45 methods (43 public, 2 private)
- **battle_stream.rs**: 30 methods (17 public, 13 private)

**Grand Total:** 337 methods (248 public, 89 private)
