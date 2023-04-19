//! Tests for rail-fence-cipher
//!
//! Generated by [script][script] using [canonical data][canonical-data]
//!
//! [script]: https://github.com/exercism/rust/blob/main/bin/init_exercise.py
//! [canonical-data]: https://raw.githubusercontent.com/exercism/problem-specifications/main/exercises/rail-fence-cipher/canonical_data.json
//!
//! The tests do not expect any normalization or cleaning.
//! That trade is tested in enough other exercises.

use rail_fence_cipher::*;

/// Process a single test case for the property `encode`
///
/// All cases for the `encode` property are implemented
/// in terms of this function.
fn process_encode_case(input: &str, rails: u32, expected: &str) {
    let rail_fence = RailFence::new(rails);
    assert_eq!(rail_fence.encode(input), expected);
}

/// Process a single test case for the property `decode`
///
/// All cases for the `decode` property are implemented
/// in terms of this function.
fn process_decode_case(input: &str, rails: u32, expected: &str) {
    let rail_fence = RailFence::new(rails);
    assert_eq!(rail_fence.decode(input), expected);
}

// encode

#[test]
/// encode with two rails
fn test_encode_with_two_rails() {
    process_encode_case("XOXOXOXOXOXOXOXOXO", 2, "XXXXXXXXXOOOOOOOOO");
}

#[test]
/// encode with three rails
fn test_encode_with_three_rails() {
    process_encode_case("WEAREDISCOVEREDFLEEATONCE", 3, "WECRLTEERDSOEEFEAOCAIVDEN");
}

#[test]
/// encode with ending in the middle
fn test_encode_with_ending_in_the_middle() {
    process_encode_case("EXERCISES", 4, "ESXIEECSR");
}

// decode

#[test]
/// decode with three rails
fn test_decode_with_three_rails() {
    process_decode_case("TEITELHDVLSNHDTISEIIEA", 3, "THEDEVILISINTHEDETAILS");
}

#[test]
/// decode with five rails
fn test_decode_with_five_rails() {
    println!("{:?}", RailFence::new(5).encode("EXERCISMISAWESOME"));
    process_decode_case("EIEXMSMESAORIWSCE", 5, "EXERCISMISAWESOME");
}

#[test]
/// decode with six rails
fn test_decode_with_six_rails() {
    process_decode_case(
        "133714114238148966225439541018335470986172518171757571896261",
        6,
        "112358132134558914423337761098715972584418167651094617711286",
    );
}

#[test]
/// encode wide characters
///
/// normally unicode is not part of exercism exercises, but in an exercise
/// specifically oriented around shuffling characters, it seems worth ensuring
/// that wide characters are handled properly
///
/// this text is possibly one of the most famous haiku of all time, by
/// Matsuo Bashō (松尾芭蕉)
fn test_encode_wide_characters() {
    process_encode_case("古池蛙飛び込む水の音", 3, "古びの池飛込水音蛙む");
}