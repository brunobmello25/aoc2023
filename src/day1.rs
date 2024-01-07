pub fn run() {
    run_part_2();
}

const NUMERIC_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[allow(dead_code)]
fn run_part_1() {
    let contents = std::fs::read_to_string("input/day1.txt").unwrap();

    let sum = get_sum(contents);

    println!("sum is {}", sum);
}

fn run_part_2() {
    let contents = std::fs::read_to_string("input/day1.txt").unwrap();
    let mut cleaned_lines = vec![];

    for line in contents.lines() {
        let cleaned_line = get_cleaned_line(line.to_string());
        cleaned_lines.push(cleaned_line);
    }

    let sum = get_sum(cleaned_lines.join("\n"));
    println!("sum is {}", sum);
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

fn replace_numeric_word(line: String) -> String {
    for word in NUMERIC_WORDS {
        if line.contains(word) {
            let digit = get_numeric_word_digit(word);
            return line.replace(word, &digit.to_string());
        }
    }

    return line;
}

fn get_cleaned_line(line: String) -> String {
    let mut consuming = "".to_string();

    for ch in line.chars() {
        consuming.push(ch);
        if has_numeric_word(&consuming) {
            consuming = replace_numeric_word(consuming);
        }
    }

    return consuming;
}

fn get_numeric_word_digit(word: &str) -> usize {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

fn has_numeric_word(line: &String) -> bool {
    for word in NUMERIC_WORDS {
        if line.contains(word) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::day1::get_sum;

    use super::get_cleaned_line;

    #[test]
    fn test_get_cleaned_line() {
        assert_eq!(
            get_cleaned_line("4bvnccbdh4onefztdrpq62vvbnvpxxvgrngnfjgfk".to_string()),
            "4bvnccbdh41fztdrpq62vvbnvpxxvgrngnfjgfk"
        );
        assert_eq!(
            get_cleaned_line("2ninebvgdcfxtktqjxjqvxfgjdqfhv5threegqtsfhtfxg".to_string()),
            "29bvgdcfxtktqjxjqvxfgjdqfhv53gqtsfhtfxg"
        );
        assert_eq!(
            get_cleaned_line("xtwo7threemxbtpsvjkgrfivethree2".to_string()),
            "x273mxbtpsvjkgr532"
        );
        assert_eq!(
            get_cleaned_line("cqoneighteightjnrfkplvninefivemck18mnhszhkv4".to_string()),
            "cq1ight8jnrfkplv95mck18mnhszhkv4"
        );
        assert_eq!(
            get_cleaned_line("tbvdcsjsvmxtshv3fourseven4kmxvvfour9".to_string()),
            "tbvdcsjsvmxtshv3474kmxvv49"
        );
        assert_eq!(
            get_cleaned_line("bxcsix19six8dnqsbx".to_string()),
            "bxc61968dnqsbx"
        );
    }

    #[test]
    fn test_get_sum() {
        let tests = vec![
            ("n7", 77),
            ("1three2", 12),
            ("cqoneighteightjnrfkplvninefivemck18mnhszhkv4", 14),
            ("tbvdcsjsvmxtshv3fourseven4kmxvvfour9", 39),
            ("bxcsix19six8dnqsbx", 68),
            ("cqoneighteightjnrfkplvninefivemck18mnhszhkv4\ntbvdcsjsvmxtshv3fourseven4kmxvvfour9\nbxcsix19six8dnqsbx", 14 + 39 + 68),
            ("2ninebvgdcfxtktqjxjqvxfgjdqfhv5threegqtsfhtfxg", 23),
            ("6threev", 63),
            ("69sixnine", 69),
            ("1btphrrvxdeightonekdhv8", 18),
            ("ninerlsbznvfn9", 99),
            ("sixthree6lxcrsevenseven69twonegs", 62),
            ("xrbtzbklqsl11", 11),
            ("5p", 55),
            ("66bnfj", 66),
            ("6bnfj", 66),
            ("bnfj6", 66),
            ("bnfjsix", 66),
            ("sixbnfjix", 66),
            ("6bnfj", 66),
            ("bn655556fj", 66),
            ("bn65fivefj", 65),
            ("5lclone", 51),
            ("six89bdlssd", 69),
            ("tbvdcsjsvmxtshv3fourseven4kmxvvfour9", 39),
            ("f1lhrbsix", 16),
            ("mbbkv7ffpk", 77),
            ("9eightseventhree", 93),
            ("9jpvccsvhqpnhsl8", 98),
            ("nvcnninefour9", 99),
            ("v237ppqbhb", 27),
            ("78six", 76),
            ("9eightseventhree", 93),
            ("9jpvccsvhqpnhsl8", 98),
            ("nvcnninefour9", 99),
            ("v237ppqbhb", 27),
            ("78six", 76),
            ("four165oneightxcm",41),
            ("9pjcsfbrghnineqzth4smx", 94),
            ("46fpfptrq1mbqmbnktqeight", 48),
            ("stbxvlcqz5krd1threethreeonefour", 54),
            ("48six5seven", 47),
            ("6bmltlrvrgpcfhjhmfiveqzfxptjtwo4zvsqqxgbrdlzsfmtzdd", 64),
            (
            "9eightseventhree\n9jpvccsvhqpnhsl8\nnvcnninefour9\nv237ppqbhb\n78six\nfour165oneightxcm\n9pjcsfbrghnineqzth4smx\n46fpfptrq1mbqmbnktqeight\nstbxvlcqz5krd1threethreeonefour\n48six5seven\n6bmltlrvrgpcfhjhmfiveqzfxptjtwo4zvsqqxgbrdlzsfmtzdd",
            93 + 98 + 99 + 27 + 76 + 41 + 94 + 48 + 54 + 47 + 64
            )

        ];
        for test in tests {
            assert_eq!(get_sum(get_cleaned_line(test.0.to_string())), test.1);
        }
    }
}
