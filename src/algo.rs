use std::cmp::max;

pub struct MatchResult {
    start: usize,
    end: usize,
    score: i32,
}

pub fn fuzzy_match(text: &str, pattern: &[char]) -> MatchResult {
    if pattern.is_empty() {
        return MatchResult {
            start: 0,
            end: 0,
            score: 0,
        };
    }

    // to reimplemnt when understood

    // let idx = ascii_fuzzy_index(text, pattern);

    // if idx < 0 {
    //     return MatchResult {
    //         start: -1,
    //         end: -1,
    //         score: 0,
    //     };
    // }

    let forward_match_indexes = forward_pass(text, pattern);

    match forward_match_indexes {
        None => MatchResult {
            start: 0,
            end: 0,
            score: 0,
        },
        Some((start_index, end_index)) => {
            let optimal_start_index = backward_pass(text, pattern, start_index, end_index);
            let score = calculate_score(text, pattern, optimal_start_index, end_index);
            MatchResult {
                start: start_index,
                end: end_index,
                score,
            }
        }
    }
}

fn forward_pass(text: &str, pattern: &[char]) -> Option<(usize, usize)> {
    let mut pattern_index: usize = 0;
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;

    for (char_index, char) in text.chars().enumerate() {
        let pattern_char = pattern[pattern_index];
        if char == pattern_char {
            if start_index.is_none() {
                start_index = Some(char_index);
            }

            pattern_index += 1;
            if pattern_index == pattern.len() {
                end_index = Some(char_index + 1);
                break;
            }
        }
    }

    match (start_index, end_index) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    }
}

fn backward_pass(text: &str, pattern: &[char], start_index: usize, end_index: usize) -> usize {
    let mut pattern_index = pattern.len() - 1;

    let section = &text[start_index..end_index];

    for (forward_index, char) in section.chars().rev().enumerate() {
        if char == pattern[pattern_index] {
            if pattern_index == 0 {
                return end_index - forward_index - 1;
            }
            pattern_index -= 1;
        }
    }

    start_index
}

fn ascii_fuzzy_index(text: &str, pattern: &[char]) -> i32 {
    return 0;
}

const DELIMITER_CHARS: &str = "'/',:;|";
const WHITE_CHARS: &str = " \t\n\x0B\x0Cx\r\u{85}\u{A0}";

#[derive(Debug)]
enum CharType {
    White = 0,
    NonWord,
    Delimiter,
    Lower,
    Upper,
    Letter,
    Number,
}

const SCORE_MATCH: i32 = 16;
const SCORE_GAP_EXTENSION: i32 = -1;
const SCORE_GAP_START: i32 = -3;

const BONUS_FIRST_CHAR_MULTIPLIER: i32 = 2;
const BONUS_BOUNDARY: i32 = SCORE_MATCH / 2;
const BONUS_BOUNDARY_WHITE: i32 = BONUS_BOUNDARY + 2;
const BONUS_BOUNDARY_DELIMITER: i32 = BONUS_BOUNDARY + 1;
const BONUS_CAMEL_123: i32 = BONUS_BOUNDARY + SCORE_GAP_EXTENSION;
const BONUS_NON_WORD: i32 = SCORE_MATCH / 2;
const BONUS_CONSECUTIVE: i32 = -(SCORE_GAP_EXTENSION + SCORE_GAP_START);

fn calculate_score(text: &str, pattern: &[char], start_index: usize, end_index: usize) -> i32 {
    let mut pattern_index = 0;
    let mut score = 0;
    let mut in_gap = false;
    let mut consecutive = 0;
    let mut first_bonus = 0;
    let mut prev_class = CharType::White;

    if start_index > 0 {
        prev_class = get_char_type(text.chars().nth(start_index - 1).expect("SHOULD BE FINE"))
    }

    for index in start_index..end_index {
        let char = text.chars().nth(index as usize).expect("SHOULD BE FINE");
        let class = get_char_type(char);

        if char == pattern[pattern_index] {
            score += SCORE_MATCH;
            let mut bonus = bonus_for(&prev_class, &class);

            if consecutive == 0 {
                first_bonus = bonus;
            } else {
                if bonus >= BONUS_BOUNDARY && bonus > first_bonus {
                    first_bonus = bonus
                }
                bonus = max(max(bonus, first_bonus), BONUS_CONSECUTIVE)
            }

            if pattern_index == 0 {
                score += bonus * BONUS_FIRST_CHAR_MULTIPLIER
            } else {
                score += bonus
            }

            in_gap = false;
            consecutive += 1;
            pattern_index += 1;
        } else {
            if in_gap {
                score += SCORE_GAP_EXTENSION
            } else {
                score += SCORE_GAP_START
            }
            in_gap = true;
            consecutive = 0;
            first_bonus = 0;
        }
        prev_class = class
    }

    score
}

// implemented via lookup table on the true fzf
// charClassOf
fn get_char_type(char: char) -> CharType {
    if char.is_lowercase() {
        CharType::Lower
    } else if char.is_uppercase() {
        CharType::Upper
    } else if char.is_ascii_digit() {
        CharType::Number
    } else if DELIMITER_CHARS.contains(char) {
        CharType::Delimiter
    } else if WHITE_CHARS.contains(char) {
        CharType::White
    } else {
        get_char_type_non_ascii(char)
    }
}

fn get_char_type_non_ascii(char: char) -> CharType {
    if char.is_lowercase() {
        CharType::Lower
    } else if char.is_uppercase() {
        CharType::Upper
    } else if char.is_numeric() {
        CharType::Number
    } else if char.is_whitespace() {
        CharType::White
    } else if DELIMITER_CHARS.contains(char) {
        CharType::Delimiter
    } else {
        CharType::NonWord
    }
}

// implemented via lookup table on the real fzf
fn bonus_for(previous: &CharType, current: &CharType) -> i32 {
    let current_space_or_nonword = matches!(current, CharType::White | CharType::NonWord);

    if !current_space_or_nonword {
        match previous {
            CharType::White => return BONUS_BOUNDARY_WHITE,
            CharType::Delimiter => return BONUS_BOUNDARY_DELIMITER,
            CharType::NonWord => return BONUS_BOUNDARY,
            _ => {}
        }
    }

    match (previous, current) {
        (CharType::Lower, CharType::Upper) => return BONUS_CAMEL_123,
        (CharType::Number, CharType::Number) => {}
        (_, CharType::Number) => return BONUS_CAMEL_123,
        (_, _) => {}
    }

    match current {
        CharType::NonWord | CharType::Delimiter => BONUS_NON_WORD,
        CharType::White => BONUS_BOUNDARY_WHITE,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_results(s1: &str, s2: &str, pattern_str: &str) -> (i32, i32) {
        let pattern: Vec<char> = pattern_str.chars().collect();

        let res1 = fuzzy_match(s1, &pattern).score;
        let res2 = fuzzy_match(s2, &pattern).score;

        (res1, res2)
    }

    // a bunch of tests given by the real fzf
    #[test]
    fn matching_at_special_positions() {
        let (without_positions, with_positions) =
            compare_results("fuzzyfinder", "fuzzy-finder", "ff");

        assert!(with_positions > without_positions);
    }

    #[test]
    fn first_character_specialty() {
        let (first_char, second_char) = compare_results("fo-bar", "foob-r", "br");

        assert!(first_char > second_char);
    }

    #[test]
    fn gap_penalty() {
        let (normal, position_with_gap) =
            compare_results("fuzzyfinder", "fuzzy-blurry-finder", "ff");

        assert!(normal == position_with_gap);
    }

    #[test]
    fn consecutive_matching_chunk() {
        let (consecutive, broken) = compare_results("foobar", "foo-bar", "foob");

        assert!(consecutive > broken)
    }

    #[test]
    fn consecutive_bonus_depending_on_first() {
        let (consecutive, firsts) = compare_results("foobar", "out-of-bound", "oob");

        assert!(firsts > consecutive)
    }
}
