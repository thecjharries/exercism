#!/usr/bin/env bats
# generated on 2023-08-17T10:24:18Z
load bats-extra

@test 'empty' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": []
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'no nesting' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    0,
    1,
    2
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  0,
  1,
  2
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'flattens a nested array' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    [
      []
    ]
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'flattens array with just integers present' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    1,
    [
      2,
      3,
      4,
      5,
      6,
      7
    ],
    8
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test '5 level nesting' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    0,
    2,
    [
      [
        2,
        3
      ],
      8,
      100,
      4,
      [
        [
          [
            50
          ]
        ]
      ]
    ],
    -2
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  0,
  2,
  2,
  3,
  8,
  100,
  4,
  50,
  -2
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test '6 level nesting' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    1,
    [
      2,
      [
        [
          3
        ]
      ],
      [
        4,
        [
          [
            5
          ]
        ]
      ],
      6,
      7
    ],
    8
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'null values are omitted from the final result' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    1,
    2,
    null
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  1,
  2
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'consecutive null values at the front of the list are omitted from the final result' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    null,
    null,
    3
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  3
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'consecutive null values in the middle of the list are omitted from the final result' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    1,
    null,
    null,
    4
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  1,
  4
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test '6 level nest list with null values' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    0,
    2,
    [
      [
        2,
        3
      ],
      8,
      [
        [
          100
        ]
      ],
      null,
      [
        [
          null
        ]
      ]
    ],
    -2
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[
  0,
  2,
  2,
  3,
  8,
  100,
  -2
]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}

@test 'all values in nested list are null' {

    run jq -r -f flatten-array.jq << 'END_INPUT'
{
  "array": [
    null,
    [
      [
        [
          null
        ]
      ]
    ],
    null,
    null,
    [
      [
        null,
        null
      ],
      null
    ],
    null
  ]
}
END_INPUT

    assert_success
    expected=$(cat << 'END_EXPECTED'
[]
END_EXPECTED
)
    assert_equal "$output" "$expected"
}
