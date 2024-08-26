unit module SecretHandshake;

sub handshake ( $number ) is export {
    my @actions = ['wink', 'double blink', 'close your eyes', 'jump'];
    my @result = ();
    for @actions.kv -> $i, $action {
        @result.push: $action if $number +& (1 +< $i);
    }
    @result = @result.reverse if $number +& 0b10000;
    @result;
}
