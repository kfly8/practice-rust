use strict;
use warnings;
use POSIX qw(floor);

sub bitonic_sort {
    my ($x, $up) = @_;

    if (@$x == 1) {
        return $x;
    }
    else {
        my $mid_point = floor(@$x / 2);
        my $first = bitonic_sort( [ @$x[0 .. $mid_point - 1] ], !!1);
        my $second = bitonic_sort( [ @$x[$mid_point .. @$x - 1] ], !!0);

        my $x1 = [ @$first, @$second ];

        return _sub_sort($x1, $up);
    }
} 

sub _sub_sort {
    my ($x, $up) = @_;

    if (@$x == 1) {
        return $x
    }
    else {
        _compare_and_swap($x, $up);

        my $mid_point = floor(@$x / 2);
        my $first = _sub_sort( [ @$x[0 .. $mid_point - 1] ], $up);
        my $second = _sub_sort( [ @$x[$mid_point .. @$x - 1] ], $up);

        return [ @$first, @$second ];
    }
}

sub _compare_and_swap {
    my ($x, $up) = @_;

    my $mid_point = floor(@$x / 2);
    for my $i ( 0 .. $mid_point - 1 ) {
        if ( ($x->[$i] > $x->[$mid_point + $i]) == $up ) {
            ($x->[$i], $x->[$mid_point + $i]) = ($x->[$mid_point + $i], $x->[$i])
        }
    }
}

use Test2::V0;

is(bitonic_sort( [10, 30, 11, 20, 4, 330, 21, 110], !!1 ), [4, 10, 11, 20, 21, 30, 110, 330]);
is(bitonic_sort( [10, 30, 11, 20, 4, 330, 21, 110], !!0 ), [330, 110, 30, 21, 20, 11, 10, 4]);

done_testing;
