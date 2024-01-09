#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

#[derive(Debug, PartialEq, Default)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Default)]
struct GameAmounts {
    red: usize,
    green: usize,
    blue: usize,
}

pub fn run() {
    run_part_1();
}

#[allow(dead_code)]
fn run_part_1() {
    let contents = std::fs::read_to_string("input/day2.txt").unwrap();

    let games = parse_all_lines(contents);
    let mut possibles = vec![];
    for game in games {
        let game_id = game.id;
        let game_amounts = get_game_amount(game);
        if game_amounts.red <= 12 && game_amounts.green <= 13 && game_amounts.blue <= 14 {
            possibles.push(game_id);
        }
    }
    println!("sum: {}", possibles.iter().sum::<usize>());
}

fn get_game_amount(game: Game) -> GameAmounts {
    let mut game_amounts = GameAmounts::default();

    for reveal in game.reveals {
        if game_amounts.red < reveal.red {
            game_amounts.red = reveal.red;
        }
        if game_amounts.green < reveal.green {
            game_amounts.green = reveal.green;
        }
        if game_amounts.blue < reveal.blue {
            game_amounts.blue = reveal.blue;
        }
    }

    return game_amounts;
}

fn parse_all_lines(contents: impl Into<String>) -> Vec<Game> {
    let contents: String = contents.into();

    return contents.lines().map(parse_line).collect();
}

fn parse_line(line: impl Into<String>) -> Game {
    let line = line.into();

    let parts: Vec<_> = line.split(": ").collect();
    let (game_info, game_data) = match parts[..] {
        [game_info, game_data] => (game_info, game_data),
        _ => unreachable!(),
    };

    let game_id: usize = game_info
        .replace("Game ", "")
        .parse()
        .expect(format!("failed to parse game id. Was \"{}\"", game_info).as_str());

    let mut reveals = vec![];

    for reveal_set in game_data.split("; ") {
        let reveal = parse_reveal_set(reveal_set);
        reveals.push(reveal);
    }

    let game = Game {
        id: game_id,
        reveals,
    };

    return game;
}

fn parse_reveal_set(reveal_set: impl Into<String>) -> Reveal {
    let reveal_set = reveal_set.into();

    let mut reveal = Reveal::default();

    for reveal_info in reveal_set.split(", ") {
        match reveal_info.split(" ").collect::<Vec<_>>()[..] {
            [amount, color] => match color {
                "red" => reveal.red = amount.parse().unwrap(),
                "green" => reveal.green = amount.parse().unwrap(),
                "blue" => reveal.blue = amount.parse().unwrap(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    return reveal;
}

#[cfg(test)]
mod tests {
    use crate::day2::{get_game_amount, parse_line, parse_reveal_set, Game, GameAmounts};

    use super::Reveal;

    #[test]
    fn test_get_game_amount() {
        let game = Game {
            id: 20,
            reveals: vec![
                Reveal {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Reveal {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
            ],
        };

        assert_eq!(
            get_game_amount(game),
            GameAmounts {
                red: 4,
                green: 2,
                blue: 6
            }
        )
    }

    #[test]
    fn test_parse_revel_set() {
        let tests = vec![
            (
                "3 blue, 4 red",
                Reveal {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
            ),
            (
                "1 red, 2 green, 6 blue",
                Reveal {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
            ),
            (
                "2 green",
                Reveal {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ),
        ];

        for (input, expected) in tests {
            let actual = parse_reveal_set(input.to_string());
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_line("Game 20: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 20,
                reveals: vec![
                    Reveal {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Reveal {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Reveal {
                        red: 0,
                        green: 2,
                        blue: 0,
                    }
                ],
            }
        )
    }
}
