# https://www.codewars.com/kata/57eae65a4321032ce000002d/train/perl

package Solution;

use 5.30.0;
use strict;
use warnings;
use Exporter qw(import);
our @EXPORT_OK = qw(comp);

sub fake_bin {
    my $input = $_[0];
    my $bin = "";

    foreach my $char (split('', $input)) {
        if($char lt 5) {
            $bin = $bin."0";
        } else {
            $bin = $bin."1";
        }
    }

    return $bin
}

my $input = <>;

print(fake_bin($input))
