#!/usr/bin/env perl
use strict;
use warnings;

while (my $file = shift @ARGV) {
    open my $fh, '<', $file or die "Cannot open $file: $!";
    my $content = do { local $/; <$fh> };
    close $fh;

    my $original = $content;

    # Pattern 1: if let Some(pokemon) = battle.pokemon_at_mut(...) { Pokemon::remove_volatile(battle, (pokemon.side_index, pokemon.position), &id); }
    # Find the variable name used in the if let
    $content =~ s/if let Some\((\w+)\) = battle\.pokemon_at_mut\([^)]+\) \{\s*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);\s*\}/Pokemon::remove_volatile(battle, pokemon_pos, $2);/gs;

    # Pattern 2: let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) { ... };
    #            Pokemon::remove_volatile(battle, (pokemon_mut.side_index, pokemon_mut.position), &id);
    # This is trickier - need to find and remove the whole match block, then replace the call

    # Pattern 3: let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) { ... };
    #            Pokemon::remove_volatile(battle, (pokemon.side_index, pokemon.position), &id);
    # Replace with just the remove_volatile call

    # Actually, let's do a simpler approach - just fix the actual call sites
    # Change (pokemon_var.side_index, pokemon_var.position) to the original position variable

    # For files with pokemon_pos parameter:
    $content =~ s/\{[\s\n]*let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{[\s\n]*Some\(p\) => p,[\s\n]*None => return EventResult::Continue,[\s\n]*\};[\s\n]*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);[\s\n]*\}/Pokemon::remove_volatile(battle, $2, $3);/gs;

    # For single-line if let patterns
    $content =~ s/if let Some\((\w+)\) = battle\.pokemon_at_mut\(([^)]+)\) \{[\s\n]*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);[\s\n]*\}/Pokemon::remove_volatile(battle, pokemon_pos, $3);/gs;

    if ($content ne $original) {
        open my $out, '>', $file or die "Cannot write $file: $!";
        print $out $content;
        close $out;
        print "Fixed: $file\n";
    }
}
