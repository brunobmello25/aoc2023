pub fn run() {
    run_part_2();
}

fn run_part_1() {
    let contents = std::fs::read_to_string("input/day1.txt").unwrap();

    let sum = get_sum(contents);

    println!("sum is {}", sum);
}

fn run_part_2() {
    let mut contents = std::fs::read_to_string("input/day1.txt").unwrap();

    let mut resulting_lines = vec![];

    for line in contents.lines() {
        let mut new_line = "".to_string();

        for ch in line.chars() {
            new_line.push(ch);
            new_line = replace_numeric_word(new_line);
        }

        resulting_lines.push(new_line);
    }

    let sum = get_sum(resulting_lines.join("\n"));

    println!("sum is {}", sum);
}

fn replace_numeric_word(line: String) -> String {
    let mut result = line.clone();
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for word in words {
        if result.contains(word) {
            let numeric_equivalent = get_numeric_equivalent(word);
            println!("replacing {} with {} in {}", word, numeric_equivalent, line);
            result = result.replace(word, numeric_equivalent.to_string().as_str());
        }
    }

    return result;
}

fn get_numeric_equivalent(word: &str) -> char {
    match word {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("unknown word {}", word),
    }
}

fn get_sum(contents: String) -> usize {
    let mut sum = 0;

    for line in contents.lines() {
        let mut first_digit = "".to_string();
        let mut last_digit = "".to_string();
        for ch in line.chars() {
            if is_numeric(ch) {
                if first_digit == "" {
                    first_digit = ch.to_string();
                }
                last_digit = ch.to_string();
            }
        }
        let combined = format!("{}{}", first_digit, last_digit);
        let combined = combined.parse::<i32>().unwrap();
        sum += combined;
    }

    return sum as usize;
}

fn is_numeric(ch: char) -> bool {
    return ch >= '0' && ch <= '9';
}
