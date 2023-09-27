#!/usr/bin/env bats
# generated on 2022-11-02T20:59:00Z
load bats-extra

@test 'stating something' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Tom-ay-to, tom-aaaah-to."
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'shouting' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "WATCH OUT!"
        }
END_INPUT

    assert_success
    expected='Whoa, chill out!'
    assert_equal "$output" "$expected"
}

@test 'shouting gibberish' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "FCECDFCAAB"
        }
END_INPUT

    assert_success
    expected='Whoa, chill out!'
    assert_equal "$output" "$expected"
}

@test 'asking a question' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Does this cryogenic chamber make me look fat?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'asking a numeric question' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "You are, what, like 15?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'asking gibberish' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "fffbbcbeab?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'talking forcefully' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Hi there!"
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'using acronyms in regular speech' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "It's OK if you don't want to go work for NASA."
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'forceful question' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "WHAT'S GOING ON?"
        }
END_INPUT

    assert_success
    expected='Calm down, I know what I'\''m doing!'
    assert_equal "$output" "$expected"
}

@test 'shouting numbers' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "1, 2, 3 GO!"
        }
END_INPUT

    assert_success
    expected='Whoa, chill out!'
    assert_equal "$output" "$expected"
}

@test 'no letters' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "1, 2, 3"
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'question with no letters' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "4?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'shouting with special characters' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"
        }
END_INPUT

    assert_success
    expected='Whoa, chill out!'
    assert_equal "$output" "$expected"
}

@test 'shouting with no exclamation mark' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "I HATE THE DENTIST"
        }
END_INPUT

    assert_success
    expected='Whoa, chill out!'
    assert_equal "$output" "$expected"
}

@test 'statement containing question mark' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Ending with ? means a question."
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'non-letters with question' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": ":) ?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'prattling on' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Wait! Hang on. Are you going to be OK?"
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'silence' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": ""
        }
END_INPUT

    assert_success
    expected='Fine. Be that way!'
    assert_equal "$output" "$expected"
}

@test 'prolonged silence' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "          "
        }
END_INPUT

    assert_success
    expected='Fine. Be that way!'
    assert_equal "$output" "$expected"
}

@test 'alternate silence' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "\t\t\t\t\t\t\t\t\t\t"
        }
END_INPUT

    assert_success
    expected='Fine. Be that way!'
    assert_equal "$output" "$expected"
}

@test 'multiple line question' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "\nDoes this cryogenic chamber make me look fat?\nNo."
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'starting with whitespace' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "         hmmmmmmm..."
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

@test 'ending with whitespace' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "Okay if like my  spacebar  quite a bit?   "
        }
END_INPUT

    assert_success
    expected='Sure.'
    assert_equal "$output" "$expected"
}

@test 'other whitespace' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "\n\r \t"
        }
END_INPUT

    assert_success
    expected='Fine. Be that way!'
    assert_equal "$output" "$expected"
}

@test 'non-question ending with whitespace' {

    run jq -r -f bob.jq << 'END_INPUT'
        {
          "heyBob": "This is a statement ending with whitespace      "
        }
END_INPUT

    assert_success
    expected='Whatever.'
    assert_equal "$output" "$expected"
}

