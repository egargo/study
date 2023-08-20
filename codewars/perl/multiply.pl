# https://www.codewars.com/kata/50654ddff44f800200000004/train/perl

use strict;
use warnings;

sub multiply {
    my $a = shift;
    my $b = shift;
    return $a * $b;
}

my $a = <>;
my $b = <>;

print(multiply($a, $b))
