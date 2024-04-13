package Luhn;

use v5.38;

use Exporter qw<import>;
our @EXPORT_OK = qw<is_luhn_valid>;

sub is_luhn_valid ($number) {
    $number =~ s/\s//g;
    return 0 if $number !~ /^\d{2,}$/;
    my $sum = 0;
    my $index = 1;
    for my $digit (reverse split //, $number) {
        $sum += $index % 2 ? $digit : $digit * 2 > 9 ? $digit * 2 - 9 : $digit * 2;
        $index++;
    }
    return $sum % 10 == 0;
}
