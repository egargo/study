# https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/perl

package Solution;

use 5.30.0;
use strict;
use warnings;
use Exporter qw(import);
our @EXPORT_OK = qw(comp);

sub no_space {
    my $string = $_[0];

    $string =~ s/\s//g;

    return $string;
}

my $string = <>;

print(no_space($string))
