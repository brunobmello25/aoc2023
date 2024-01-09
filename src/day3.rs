use std::fs;

#[derive(Debug, PartialEq)]
struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(input: Vec<Vec<T>>) -> Self {
        Matrix { data: input }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        let row = self.data.get(x as usize)?;
        let col = row.get(y as usize)?;
        return Some(col);
    }
}

pub fn run() {
    run_part_1();
}

#[allow(dead_code)]
fn run_part_1() {
    let contents = fs::read_to_string("input/day3.txt").unwrap();
    let matrix = parse_input(contents);

    let mut numbers: Vec<usize> = vec![];
    let mut consuming = "".to_string();
    let mut has_around = false;
    for (x, _) in matrix.data.iter().enumerate() {
        if let Some(row) = matrix.data.get(x) {
            for (y, _) in row.iter().enumerate() {
                if let Some(_) = row.get(y) {
                    if let Some(current_digit) = matrix.get(x as isize, y as isize) {
                        if current_digit.is_ascii_digit() {
                            consuming.push(*current_digit);
                            has_around =
                                has_around || has_symbol_around(&matrix, x as isize, y as isize);
                        } else {
                            if consuming.len() > 0 && has_around {
                                println!("{} has around", consuming);
                                numbers.push(consuming.parse::<usize>().unwrap());
                            }
                            consuming = "".to_string();
                            has_around = false;
                        }
                    }
                }
            }
            if consuming.len() > 0 && has_around {
                println!("{} has around", consuming);
                numbers.push(consuming.parse::<usize>().unwrap());
            }
            consuming = "".to_string();
            has_around = false;
        }
    }

    println!("sum is {}", numbers.iter().sum::<usize>());
}

fn is_symbol(ch: char) -> bool {
    return !ch.is_ascii_digit() && ch != '.';
}

fn has_gear_around(matrix: &Matrix<char>, x: isize, y: isize) -> bool {
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut result = false;
    for (dx, dy) in directions {
        if let Some(ch) = matrix.get(x + dx, y + dy) {
            if *ch == '*' {
                result = true;
            }
        }
    }
    return result;
}

fn has_symbol_around(matrix: &Matrix<char>, x: isize, y: isize) -> bool {
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut result = false;
    for (dx, dy) in directions {
        if let Some(ch) = matrix.get(x + dx, y + dy) {
            if is_symbol(*ch) {
                result = true;
            }
        }
    }
    return result;
}

fn parse_input(contents: String) -> Matrix<char> {
    return Matrix::new(
        contents
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    );
}

#[cfg(test)]
mod tests {
    use crate::day3::{has_symbol_around, Matrix};

    use super::parse_input;

    #[test]
    fn test_matrix_get() {
        let input = Matrix::new(vec![
            vec!['1', '2', '3'],
            vec!['4', '5'],
            vec!['7', '8', '9'],
            vec!['a', 'b', 'c'],
        ]);

        assert_eq!(input.get(0, 0), Some(&'1'));
        assert_eq!(input.get(0, 1), Some(&'2'));
        assert_eq!(input.get(0, 2), Some(&'3'));
        assert_eq!(input.get(0, 3), None);
        assert_eq!(input.get(1, 0), Some(&'4'));
        assert_eq!(input.get(1, 1), Some(&'5'));
        assert_eq!(input.get(1, 2), None);
        assert_eq!(input.get(2, 0), Some(&'7'));
        assert_eq!(input.get(2, 1), Some(&'8'));
        assert_eq!(input.get(2, 2), Some(&'9'));
        assert_eq!(input.get(3, 0), Some(&'a'));
        assert_eq!(input.get(3, 1), Some(&'b'));
        assert_eq!(input.get(3, 2), Some(&'c'));
    }

    #[test]
    fn test_has_symbol_around() {
        let input = parse_input("..#\n...\n...".to_string());
        assert!(!has_symbol_around(&input, 0, 0));
        assert!(has_symbol_around(&input, 0, 1));
        assert!(!has_symbol_around(&input, 0, 2));

        assert!(!has_symbol_around(&input, 1, 0));
        assert!(has_symbol_around(&input, 1, 1));
        assert!(has_symbol_around(&input, 1, 2));

        assert!(!has_symbol_around(&input, 2, 0));
        assert!(!has_symbol_around(&input, 2, 1));
        assert!(!has_symbol_around(&input, 2, 2));
    }
}
