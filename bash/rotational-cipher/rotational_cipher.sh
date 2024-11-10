#!/usr/bin/env bash

input="$1"
shift="$2"

alphabet="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
encrypted="${alphabet:$shift:$((26 - shift))}${alphabet::$shift}"
encrypted+="${alphabet:$((26 + shift))}${alphabet:26:$shift}"

declare -A letters
output=""

for ((i = 0; i < ${#alphabet}; i++)); do
    letters[${alphabet:$i:1}]="${encrypted:$i:1}"
done

for ((i = 0; i < ${#input}; i++)); do
    char="${input:$i:1}"
    if [[ "${letters[$char]}" ]]; then
        output+="${letters[$char]}"
    else
        output+="$char"
    fi
done

echo "$output"
