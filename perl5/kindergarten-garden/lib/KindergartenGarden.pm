package KindergartenGarden;

# use v5.40;
use v5.38;

use Exporter qw<import>;
our @EXPORT_OK = qw<plants>;

sub plants ( $diagram, $student ) {
    my %plants = (
        C => 'clover',
        G => 'grass',
        R => 'radishes',
        V => 'violets',
    );
    my $length = length($diagram) / 2;
    my $index = ord($student) - ord('A');

    return [
        $plants{ substr($diagram, $index * 2, 1) },
        $plants{ substr($diagram, $index * 2 + 1, 1) },
        $plants{ substr($diagram, $length + $index * 2 + 1, 1) },
        $plants{ substr($diagram, $length + $index * 2 + 2, 1) },
    ];
}

1;
