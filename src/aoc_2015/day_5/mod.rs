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
