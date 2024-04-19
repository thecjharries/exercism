const std = @import("std");

pub fn primes(buffer: []u32, comptime limit: u32) []u32 {
    var sieve = std.StaticBitSet(limit + 1).initEmpty();
    sieve.set(0);
    sieve.set(1);
    var prime_index: u32 = 0;
    var possible_prime: u32 = 2;
    while (possible_prime <= limit) : (possible_prime += 1) {
        if (sieve.isSet(possible_prime)) continue;
        buffer[prime_index] = possible_prime;
        prime_index += 1;
        var multiple: u32 = possible_prime * possible_prime;
        while (multiple <= limit) : (multiple += possible_prime) {
            sieve.set(multiple);
        }
    }
    return buffer[0..prime_index];
}
