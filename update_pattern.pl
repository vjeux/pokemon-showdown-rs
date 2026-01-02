#!/usr/bin/env perl

use strict;
use warnings;

my $file = $ARGV[0] or die "Usage: $0 <file>\n";
my $content = do { local $/; open my $fh, '<', $file or die $!; <$fh> };
my $original = $content;

# Pattern 1: let pokemon = battle.pokemon_at_mut(pokemon_pos...); pokemon.add_volatile(...)
$content =~ s/let pokemon = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\s+Some\(p\) => p,\s+None => return EventResult::Continue,\s+\};\s+pokemon\.add_volatile\(([^)]+)\);/Pokemon::add_volatile(battle, pokemon_pos, $1, None);/gs;

# Pattern 2: let pokemon_mut = battle.pokemon_at_mut(pokemon_pos...); pokemon_mut.add_volatile(...)
$content =~ s/let pokemon_mut = match battle\.pokemon_at_mut\(pokemon_pos\.0, pokemon_pos\.1\) \{\s+Some\(p\) => p,\s+None => return EventResult::Continue,\s+\};\s+pokemon_mut\.add_volatile\(([^)]+)\);/Pokemon::add_volatile(battle, pokemon_pos, $1, None);/gs;

# Pattern 3: Similar but with source_pos
$content =~ s/let pokemon = match battle\.pokemon_at_mut\(source_pos\.0, source_pos\.1\) \{\s+Some\(p\) => p,\s+None => return EventResult::Continue,\s+\};\s+pokemon\.add_volatile\(([^)]+)\);/Pokemon::add_volatile(battle, source_pos, $1, None);/gs;

$content =~ s/let source_pokemon = match battle\.pokemon_at_mut\(source_pos\.0, source_pos\.1\) \{\s+Some\(p\) => p,\s+None => return EventResult::Continue,\s+\};\s+source_pokemon\.add_volatile\(([^)]+)\);/Pokemon::add_volatile(battle, source_pos, $1, None);/gs;

# Pattern 4: pokemon variable from source
$content =~ s/let pokemon = match battle\.pokemon_at_mut\(source\.0, source\.1\) \{\s+Some\(p\) => p,\s+None => return EventResult::Continue,\s+\};\s+pokemon\.add_volatile\(([^)]+)\);/Pokemon::add_volatile(battle, source, $1, None);/gs;

if ($content ne $original) {
    open my $fh, '>', $file or die $!;
    print $fh $content;
    close $fh;
    print "Updated: $file\n";
}
