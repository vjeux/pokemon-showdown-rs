//! BattleActions struct

use crate::dex::Dex;

/// Battle Actions struct - 1:1 port of BattleActions class
/// Note: In Rust, this struct needs a reference to battle state.
/// The actual methods that need battle access are implemented on Battle directly.
pub struct BattleActions<'a> {
    /// Dex reference
    pub dex: &'a Dex,
    pub gen: u8,
}

/// Max move names by type
pub mod max_moves {
    pub const FLYING: &str = "Max Airstream";
    pub const DARK: &str = "Max Darkness";
    pub const FIRE: &str = "Max Flare";
    pub const BUG: &str = "Max Flutterby";
    pub const WATER: &str = "Max Geyser";
    pub const STATUS: &str = "Max Guard";
    pub const ICE: &str = "Max Hailstorm";
    pub const FIGHTING: &str = "Max Knuckle";
    pub const ELECTRIC: &str = "Max Lightning";
    pub const PSYCHIC: &str = "Max Mindstorm";
    pub const POISON: &str = "Max Ooze";
    pub const GRASS: &str = "Max Overgrowth";
    pub const GHOST: &str = "Max Phantasm";
    pub const GROUND: &str = "Max Quake";
    pub const ROCK: &str = "Max Rockfall";
    pub const FAIRY: &str = "Max Starfall";
    pub const STEEL: &str = "Max Steelspike";
    pub const NORMAL: &str = "Max Strike";
    pub const DRAGON: &str = "Max Wyrmwind";
}

/// Z-move names by type
pub mod z_moves {
    pub const POISON: &str = "Acid Downpour";
    pub const FIGHTING: &str = "All-Out Pummeling";
    pub const DARK: &str = "Black Hole Eclipse";
    pub const GRASS: &str = "Bloom Doom";
    pub const NORMAL: &str = "Breakneck Blitz";
    pub const ROCK: &str = "Continental Crush";
    pub const STEEL: &str = "Corkscrew Crash";
    pub const DRAGON: &str = "Devastating Drake";
    pub const ELECTRIC: &str = "Gigavolt Havoc";
    pub const WATER: &str = "Hydro Vortex";
    pub const FIRE: &str = "Inferno Overdrive";
    pub const GHOST: &str = "Never-Ending Nightmare";
    pub const BUG: &str = "Savage Spin-Out";
    pub const PSYCHIC: &str = "Shattered Psyche";
    pub const ICE: &str = "Subzero Slammer";
    pub const FLYING: &str = "Supersonic Skystrike";
    pub const GROUND: &str = "Tectonic Rage";
    pub const FAIRY: &str = "Twinkle Tackle";
}

impl<'a> BattleActions<'a> {
}
