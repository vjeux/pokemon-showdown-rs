#!/usr/bin/env perl
use strict;
use warnings;

# Script to add `use crate::Pokemon;` import to files that call Pokemon::remove_volatile

while (my $file = shift @ARGV) {
    open my $fh, '<', $file or die "Cannot open $file: $!";
    my $content = do { local $/; <$fh> };
    close $fh;

    my $original = $content;

    # Only add if file contains Pokemon::remove_volatile and doesn't already have the import
    if ($content =~ /Pokemon::remove_volatile/ && $content !~ /use crate::Pokemon;/) {
        # Find the last use statement and add after it
        $content =~ s/(use crate::[^;]+;)(\s*\n)(?!use crate::)/$1\nuse crate::Pokemon;$2/;

        if ($content ne $original) {
            open my $out, '>', $file or die "Cannot write $file: $!";
            print $out $content;
            close $out;
            print "Added import to: $file\n";
        }
    }
}
