#!/usr/bin/env perl
use strict;
use warnings;

# Fix E0499 errors by converting pokemon_at_mut + Pokemon::remove_volatile patterns

foreach my $file (@ARGV) {
    open my $fh, '<', $file or die "Cannot open $file: $!";
    my $content = do { local $/; <$fh> };
    close $fh;

    my $original = $content;

    # Pattern 1: Simple block with contains_key check
    # {
    #     let VAR = match battle.pokemon_at_mut(POS.0, POS.1) { ... };
    #     if VAR.volatiles.contains_key(&id) {
    #         Pokemon::remove_volatile(battle, (VAR.side_index, VAR.position), &id);
    #         return EventResult::Continue;
    #     }
    # }
    $content =~ s/\{\s*let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*Some\(p\) => p,\s*None => return EventResult::Continue,\s*\};\s*if \1\.volatiles\.contains_key\((&[^)]+)\) \{\s*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), \3\);\s*(?:\/\/ return;\s*)?return EventResult::Continue;\s*\}\s*\}/let has_volatile = {\n        let $1 = match battle.pokemon_at($2.0, $2.1) {\n            Some(p) => p,\n            None => return EventResult::Continue,\n        };\n        $1.volatiles.contains_key($3)\n    };\n\n    if has_volatile {\n        Pokemon::remove_volatile(battle, $2, $3);\n        return EventResult::Continue;\n    }/gs;

    # Pattern 2: Simple pokemon_at_mut followed by remove_volatile (no check)
    # let VAR = match battle.pokemon_at_mut(POS.0, POS.1) { ... };
    # Pokemon::remove_volatile(battle, (VAR.side_index, VAR.position), &id);
    $content =~ s/let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*Some\(p\) => p,\s*None => return EventResult::Continue,\s*\};\s*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);/Pokemon::remove_volatile(battle, $2, $3);/gs;

    # Pattern 3: Indented version (inside if/else blocks)
    $content =~ s/(\s+)let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \3\.1\) \{\s*Some\(p\) => p,\s*None => return EventResult::Continue,\s*\};\s*Pokemon::remove_volatile\(battle, \(\2\.side_index, \2\.position\), (&[^)]+)\);/${1}Pokemon::remove_volatile(battle, $3, $4);/gs;

    # Pattern 4: With intermediate checks
    $content =~ s/let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*Some\(p\) => p,\s*(?:\/\/ .*\n\s*)?None => return EventResult::Continue,\s*\};\s*Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);/Pokemon::remove_volatile(battle, $2, $3);/gs;

    if ($content ne $original) {
        open my $out, '>', $file or die "Cannot write $file: $!";
        print $out $content;
        close $out;
        print "Fixed: $file\n";
    }
}
