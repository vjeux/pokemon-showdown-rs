//! Parameter structs for battle action functions

/// Parameters for Z-move functions
pub struct ZMoveParams<'a> {
    pub move_name: &'a str,
    pub move_type: &'a str,
    pub move_category: &'a str,
    pub z_move_base_power: Option<i32>,
    pub item_z_move: Option<&'a str>,
    pub item_z_move_from: Option<&'a str>,
    pub item_z_move_type: Option<&'a str>,
    pub z_move_used: bool,
}

/// Parameters for can_z_move function
pub struct CanZMoveParams<'a> {
    pub z_move_used: bool,
    pub is_transformed: bool,
    pub species_is_mega: bool,
    pub species_is_primal: bool,
    pub species_forme: &'a str,
    pub item_z_move: bool,
    pub item_user: Option<&'a [String]>,
    pub species_name: &'a str,
}

/// Parameters for get_damage function
pub struct DamageCalcParams<'a> {
    pub attacker_level: i32,
    pub attacker_attack: i32,
    pub defender_defense: i32,
    pub base_power: i32,
    pub stab_modifier: f64,
    pub type_effectiveness: f64,
    pub is_crit: bool,
    pub random_factor: i32,
    pub other_modifiers: &'a [f64],
}
