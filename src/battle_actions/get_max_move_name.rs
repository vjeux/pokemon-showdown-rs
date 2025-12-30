
/// Max Move names by type
pub fn get_max_move_name(move_type: &str) -> &'static str {
    match move_type {
        "Flying" => "Max Airstream",
        "Dark" => "Max Darkness",
        "Fire" => "Max Flare",
        "Bug" => "Max Flutterby",
        "Water" => "Max Geyser",
        "Status" => "Max Guard",
        "Ice" => "Max Hailstorm",
        "Fighting" => "Max Knuckle",
        "Electric" => "Max Lightning",
        "Psychic" => "Max Mindstorm",
        "Poison" => "Max Ooze",
        "Grass" => "Max Overgrowth",
        "Ghost" => "Max Phantasm",
        "Ground" => "Max Quake",
        "Rock" => "Max Rockfall",
        "Fairy" => "Max Starfall",
        "Steel" => "Max Steelspike",
        "Normal" => "Max Strike",
        "Dragon" => "Max Wyrmwind",
        _ => "Max Strike",
    }
}
