type WordData = (String, isize);
type DigitData = (char, isize);

pub fn run() {
    run_part_2();
}

#[allow(dead_code)]
fn run_part_1() {
    let contents = std::fs::read_to_string("input/day1.txt").unwrap();

    let mut sum = 0;

    for line in contents.lines() {
        let combined = get_combined_of_line_with_only_digits(line.to_string());
        let combined = combined.parse::<i32>().unwrap();
        sum += combined;
    }

    println!("sum is {}", sum);
}

#[allow(dead_code)]
fn run_part_2() {
    let contents = std::fs::read_to_string("input/day1.txt").unwrap();

    let mut sum = 0;

    for line in contents.lines() {
        let combined = get_combined_of_line_considering_words(line.to_string());
        let combined = combined.parse::<i32>().unwrap();
        sum += combined;
    }

    println!("sum is {}", sum);
}

fn get_combined_of_line_considering_words(line: String) -> String {
    let first_word = get_first_word_with_index(&line);
    let last_word = get_last_word_with_index(&line);
    let first_digit = get_first_digit_with_index(&line);
    let last_digit = get_last_digit_with_index(&line);

    let first_digit_to_append = if first_word.1 < first_digit.1 {
        word_to_digit(first_word.0)
    } else {
        first_digit.0
    };

    let last_digit_to_append = if last_word.1 > last_digit.1 {
        word_to_digit(last_word.0)
    } else {
        last_digit.0
    };

    let combined = format!("{}{}", first_digit_to_append, last_digit_to_append);

    return combined;
}

fn get_combined_of_line_with_only_digits(line: String) -> String {
    let first_digit = get_first_digit_with_index(&line);
    let last_digit = get_last_digit_with_index(&line);

    let combined = format!("{}{}", first_digit.0, last_digit.0);

    return combined;
}

fn word_to_digit(word: String) -> char {
    match word.as_str() {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => unreachable!("should never have gotten here when converting word to digit"),
    }
}

fn get_first_word_with_index(line: &String) -> WordData {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first_word = String::new();
    let mut first_index = isize::MAX;

    for word in words.iter() {
        if let Some(index) = line.find(*word) {
            let index = index as isize;
            if index <= first_index {
                first_word = word.to_string();
                first_index = index;
            }
        }
    }

    return (first_word, first_index);
}

fn get_last_word_with_index(line: &String) -> WordData {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut last_word = String::new();
    let mut last_index = isize::MIN;

    for word in words.iter() {
        if let Some(index) = line.rfind(*word) {
            let index = index as isize;
            if index >= last_index {
                last_word = word.to_string();
                last_index = index;
            }
        }
    }

    return (last_word, last_index);
}

fn get_first_digit_with_index(line: &String) -> DigitData {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut first_digit = '0';
    let mut first_index = isize::MAX;

    for digit in digits {
        if let Some(index) = line.find(std::char::from_digit(digit as u32, 10).unwrap()) {
            let index = index as isize;
            if index <= first_index {
                first_digit = std::char::from_digit(digit as u32, 10).unwrap();
                first_index = index;
            }
        }
    }

    return (first_digit, first_index);
}

fn get_last_digit_with_index(line: &String) -> DigitData {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut last_digit = '0';
    let mut last_index = isize::MIN;

    for digit in digits {
        if let Some(index) = line.rfind(std::char::from_digit(digit as u32, 10).unwrap()) {
            let index = index as isize;
            if index >= last_index {
                last_digit = std::char::from_digit(digit as u32, 10).unwrap();
                last_index = index;
            }
        }
    }

    return (last_digit, last_index);
}

#[cfg(test)]
mod tests {
    use crate::day1::{
        get_combined_of_line_considering_words, get_combined_of_line_with_only_digits,
        get_first_digit_with_index, get_first_word_with_index, get_last_digit_with_index,
        get_last_word_with_index,
    };

    #[test]
    fn test_get_first_word_with_index() {
        let tests = vec![
            ("12onetwothreefour", ("one", 2)),
            ("aaaaatwoone", ("two", 5)),
            ("one", ("one", 0)),
        ];

        for test in tests {
            let input = test.0.to_string();
            let expected_result = (test.1 .0.to_string(), test.1 .1);
            assert_eq!(get_first_word_with_index(&input), expected_result);
        }
    }

    #[test]
    fn test_get_last_word_with_index() {
        let tests = vec![
            ("12onetwothreefour", ("four", 13)),
            ("aaaaatwoone", ("one", 8)),
            ("aaaaatwone", ("one", 7)),
            ("one", ("one", 0)),
        ];

        for test in tests {
            let input = test.0.to_string();
            let expected_result = (test.1 .0.to_string(), test.1 .1);
            assert_eq!(get_last_word_with_index(&input), expected_result);
        }
    }

    #[test]
    fn test_get_first_digit_with_index() {
        let tests = vec![
            ("012onetwo34", ('1', 1)),
            ("12onetwo34", ('1', 0)),
            ("aaaaaa21onetwo34", ('2', 6)),
            ("22221onetwo34", ('2', 0)),
            ("2", ('2', 0)),
        ];

        for test in tests {
            let input = test.0.to_string();
            let expected_result = (test.1 .0, test.1 .1);
            assert_eq!(get_first_digit_with_index(&input), expected_result);
        }
    }

    #[test]
    fn test_get_last_digit_with_index() {
        let tests = vec![
            ("12onetwo34", ('4', 9)),
            ("12onetwo340", ('4', 9)),
            ("aaaaaa21onetwo34", ('4', 15)),
            ("22221onetwo34", ('4', 12)),
            ("1", ('1', 0)),
        ];

        for test in tests {
            let input = test.0.to_string();
            let expected_result = (test.1 .0, test.1 .1);
            assert_eq!(get_last_digit_with_index(&input), expected_result);
        }
    }

    #[test]
    fn test_get_combined_considering_only_digits() {
        let tests = vec![("1", "11")];

        for test in tests {
            assert_eq!(
                get_combined_of_line_with_only_digits(test.0.to_string()),
                test.1.to_string()
            );
        }
    }

    #[test]
    fn test_get_combined_considering_words() {
        let tests = vec![
            ("sixthree6lxcrsevenseven69twonegs", "61"),
            ("1", "11"),
            ("one", "11"),
        ];

        for test in tests {
            assert_eq!(
                get_combined_of_line_considering_words(test.0.to_string()),
                test.1.to_string()
            );
        }
    }
}
