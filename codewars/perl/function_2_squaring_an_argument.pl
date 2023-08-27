# https://www.codewars.com/kata/523b623152af8a30c6000027/train/perl

use strict;
package Solution;

sub square{
    return $_[0] * $_[0];
}

my $number = <>;

print(square($number))
