use std::{error, fs::read_to_string};

fn check_vowels_requirement(word: &String) -> bool {
    let mut amount_of_vowels = 0;

    for letter in word.chars() {
        match letter {
            'a' => amount_of_vowels += 1,
            'e' => amount_of_vowels += 1,
            'i' => amount_of_vowels += 1,
            'o' => amount_of_vowels += 1,
            'u' => amount_of_vowels += 1,
            _ => {}
        }
    }

    amount_of_vowels >= 3
}

fn check_doule_letter_requirement(word: &String) -> bool {
    word.chars()
        .fold(('.', false), |acc, next_letter| {
            if acc.1 {
                return acc;
            }

            if next_letter == acc.0 {
                return (acc.0, true);
            }

            (next_letter, acc.1)
        })
        .1
}

fn check_prevented_substrings_requirement(word: &String) -> bool {
    let mut prevented_substring_occurs = true;

    for letter_index in 0..word.len() - 1 {
        if prevented_substring_occurs == false {
            break;
        }

        let checking_substring = word.get(letter_index..letter_index + 2);

        if let Some(substring) = checking_substring {
            match substring {
                "ab" => prevented_substring_occurs = false,
                "cd" => prevented_substring_occurs = false,
                "pq" => prevented_substring_occurs = false,
                "xy" => prevented_substring_occurs = false,
                _ => {}
            }
        }
    }

    prevented_substring_occurs
}

pub fn solution_first_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data")?.trim().to_string();

    let words: Vec<String> = file_string
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let amount_of_nice_words = words.iter().fold(0, |acc, next_word| {
        if check_vowels_requirement(next_word)
            && check_doule_letter_requirement(next_word)
            && check_prevented_substrings_requirement(next_word)
        {
            return acc + 1;
        }
        acc
    });

    Ok(amount_of_nice_words)
}

fn check_pairs_requirement(word: &String) -> bool {
    let mut has_twice_pair = false;

    for index in 0..word.len() - 1 {
        if has_twice_pair {
            break;
        }

        let pair = word.get(index..index + 2).unwrap();
        let rest_first_part = word.get(0..index).unwrap().to_string();
        let rest_second_pard = word.get(index + 2..word.len()).unwrap().to_string();

        if rest_first_part.contains(pair) || rest_second_pard.contains(pair) {
            has_twice_pair = true;
        };
    }

    has_twice_pair
}

fn check_pairs_with_middle_char(word: &String) -> bool {
    let mut has_same_letters_with_another_between = false;

    for index in 0..word.len() - 1 {
        if has_same_letters_with_another_between {
            break;
        }

        let first = word.chars().nth(index);
        let third = word.chars().nth(index + 2);

        if first == third {
            has_same_letters_with_another_between = true;
        };
    }

    has_same_letters_with_another_between
}

pub fn solution_second_part() -> Result<usize, Box<dyn error::Error>> {
    let file_string = read_to_string("data")?.trim().to_string();

    let words: Vec<String> = file_string
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let amount_of_nice_words = words.iter().fold(0, |acc, next_word| {
        if check_pairs_requirement(next_word) && check_pairs_with_middle_char(next_word) {
            return acc + 1;
        }
        acc
    });

    Ok(amount_of_nice_words)
}
