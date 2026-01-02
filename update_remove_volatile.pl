#!/usr/bin/env perl
use strict;
use warnings;

# Script to update remove_volatile callsites in data/ files
# Pattern: pokemon_var.remove_volatile(&id) -> Pokemon::remove_volatile(battle, (pokemon_var.side_index, pokemon_var.position), &id)

while (my $file = shift @ARGV) {
    open my $fh, '<', $file or die "Cannot open $file: $!";
    my $content = do { local $/; <$fh> };
    close $fh;

    my $original = $content;

    # Pattern 1: Simple case - pokemon.remove_volatile(&ID::from("..."))
    # Extract pokemon variable name and convert
    $content =~ s/(\w+)\.remove_volatile\((&ID::from\([^)]+\))\)/Pokemon::remove_volatile(battle, ($1.side_index, $1.position), $2)/g;

    # Pattern 2: Variable ID - pokemon.remove_volatile(&some_id)
    $content =~ s/(\w+)\.remove_volatile\((&\w+)\)/Pokemon::remove_volatile(battle, ($1.side_index, $1.position), $2)/g;

    if ($content ne $original) {
        open my $out, '>', $file or die "Cannot write $file: $!";
        print $out $content;
        close $out;
        print "Updated: $file\n";
    }
}
