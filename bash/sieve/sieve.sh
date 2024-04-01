#!/usr/bin/env bash

(($# == 1)) || exit 1

declare -i n=$1
declare -a primes out

for ((i = 2; i <= n; i++)); do
    primes[i]=1
done

for ((i = 2; i <= n; i++)); do
    ((primes[i])) || continue
    out+=("$i")
    for ((j = 2 * i; j <= n; j += i)); do
        primes[j]=0
    done
done

echo "${out[@]}"
