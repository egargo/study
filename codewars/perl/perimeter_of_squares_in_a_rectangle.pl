#!/usr/bin/perl -W

# https://www.codewars.com/kata/559a28007caad2ac4e000083/train/perl

package Solution;

use 5.30.0;
use strict;
use warnings;
use Exporter qw(import);
our @EXPORT_OK = qw(perimeter);

sub perimeter {
    my $n = $_[0];

    if ($n == 0) {
        return 4;
    }

    my $ft = 1;
    my $st = 1;
    my $nt = 0;
    my $s = $ft + $st;

    for my $i (2..$n) {
        $nt = $ft + $st;
        $st = $ft;
        $ft = $nt;
        $s = $s + $nt;
    }

   return $s * 4;
}

my $n = <>;

print(perimeter($n));
