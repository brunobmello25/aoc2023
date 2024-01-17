use std::{error::Error, fs, str::FromStr};

#[derive(Debug, PartialEq)]
struct MapConversion {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

#[derive(Debug, PartialEq)]
struct Almanac1 {
    maps: Vec<Map>,
    seeds: Vec<usize>,
}

impl Almanac1 {
    fn get_seeds_destinations(&self) -> usize {
        let mut lowest = usize::MAX;

        for seed in &self.seeds {
            let mut found = false;
            let mut origin = "seed".to_string();
            let mut seed_value = *seed;

            while !found {
                let conversion_map = self.maps.iter().find(|m| m.from == origin).unwrap();

                seed_value = conversion_map.get_destination(seed_value);
                origin = conversion_map.to.clone();

                if conversion_map.to == "location" {
                    found = true;
                    if lowest > seed_value {
                        lowest = seed_value;
                    }
                }
            }
        }

        return lowest;
    }
}

impl Almanac2 {
    fn get_seeds_destinations(&self) -> usize {
        let mut lowest: usize = usize::MAX;

        for seed_data in &self.seeds_data {
            println!(
                "calculating seed from {} to {}",
                seed_data.start,
                seed_data.start + seed_data.length
            );
            for seed in seed_data.start..(seed_data.start + seed_data.length) {
                let mut found = false;
                let mut origin = "seed".to_string();
                let mut seed_value = seed;

                while !found {
                    let conversion_map = self.maps.iter().find(|m| m.from == origin).unwrap();

                    seed_value = conversion_map.get_destination(seed_value);
                    origin = conversion_map.to.clone();

                    if conversion_map.to == "location" {
                        found = true;
                        if lowest > seed_value {
                            lowest = seed_value;
                        }
                    }
                }
            }
        }

        return lowest;
    }
}

impl FromStr for Almanac1 {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();

        let (seeds, maps) = match input.split_once("\n\n") {
            Some((seeds, maps)) => (seeds, maps),
            _ => unreachable!(),
        };

        let seeds: Result<Vec<_>, _> = seeds
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();
        let seeds = seeds?;

        let maps: Result<Vec<_>, _> = maps.split("\n\n").map(|s| s.parse::<Map>()).collect();
        let maps = maps?;

        Ok(Almanac1 { seeds, maps })
    }
}

#[derive(Debug, PartialEq)]
struct Almanac2 {
    maps: Vec<Map>,
    seeds_data: Vec<SeedData>,
}

#[derive(Debug, PartialEq)]
struct SeedData {
    start: usize,
    length: usize,
}

impl SeedData {
    fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
}

impl FromStr for Almanac2 {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();

        let (seeds, maps) = match input.split_once("\n\n") {
            Some((seeds, maps)) => (seeds, maps),
            _ => unreachable!(),
        };

        let seeds_data = seeds.replace("seeds: ", "");
        let seeds_data: Vec<usize> = seeds_data
            .split_whitespace()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let seeds_data: Vec<SeedData> = seeds_data
            .chunks(2)
            .filter_map(|chunk| match *chunk {
                [start, length] => Some(SeedData::new(start, length)),
                _ => panic!("Invalid input"),
            })
            .collect();

        let maps: Result<Vec<_>, _> = maps.split("\n\n").map(|s| s.parse::<Map>()).collect();
        let maps = maps?;

        Ok(Almanac2 { seeds_data, maps })
    }
}

impl FromStr for MapConversion {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        let [dest, source, length] = match parts[..] {
            [dest, source, length] => [dest, source, length],
            _ => return Err("Invalid input".into()),
        };

        let dest = dest.parse::<usize>()?;
        let source = source.parse::<usize>()?;
        let length = length.parse::<usize>()?;

        Ok(MapConversion {
            source_start: source,
            destination_start: dest,
            length,
        })
    }
}

impl MapConversion {
    fn get_destination(&self, source: usize) -> usize {
        if (self.source_start..(self.source_start + self.length)).contains(&source) {
            return self.destination_start + (source - self.source_start);
        }

        return source;
    }

    fn has_conversion(&self, source: usize) -> bool {
        return (self.source_start..(self.source_start + self.length)).contains(&source);
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    conversions: Vec<MapConversion>,
    from: String,
    to: String,
}

impl Map {
    fn get_destination(&self, source: usize) -> usize {
        for conversion in &self.conversions {
            if conversion.has_conversion(source) {
                return conversion.get_destination(source);
            }
        }
        return source;
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (title, body) = match s.trim().split_once("\n") {
            Some((title, body)) => (title, body),
            _ => return Err("Invalid input".into()),
        };

        let title = title.replace(" map:", "");
        let (from, to) = match title.trim().split_once("-to-") {
            Some((from, to)) => (from, to),
            _ => return Err("Invalid input".into()),
        };

        let conversions: Vec<_> = body
            .lines()
            .map(|line| line.parse::<MapConversion>().unwrap())
            .collect();

        return Ok(Map {
            from: from.into(),
            to: to.into(),
            conversions,
        });
    }
}

#[allow(dead_code)]
pub fn run() {
    run_part_2();
}

#[allow(dead_code)]
fn run_part_2() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    println!("finished reading");
    let input = input.trim();
    println!("finished triming");

    let almanac = input.parse::<Almanac2>().unwrap();
    println!("finished parsing");

    println!("Result: {}", almanac.get_seeds_destinations());
}

#[allow(dead_code)]
fn run_part_1() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let input = input.trim();

    let almanac = input.parse::<Almanac1>().unwrap();

    println!("Result: {}", almanac.get_seeds_destinations());
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::day5::{Almanac2, SeedData};

    use super::{Almanac1, Map, MapConversion};

    #[test]
    fn test_seeds_part_2_parsing() {
        let input = indoc! {"
            seeds: 20 5 10 8

            a-to-b map:
            0 1 2
        "};

        let parsed = input.parse::<Almanac2>().unwrap();

        assert_eq!(
            parsed,
            Almanac2 {
                seeds_data: vec![SeedData::new(20, 5), SeedData::new(10, 8)],
                maps: vec![Map {
                    from: "a".to_string(),
                    to: "b".to_string(),
                    conversions: vec![MapConversion {
                        destination_start: 0,
                        source_start: 1,
                        length: 2
                    }],
                }]
            }
        )
    }

    #[test]
    fn test_get_seeds_destinations_part_2() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let almanac = input.parse::<Almanac2>().unwrap();
        let result = almanac.get_seeds_destinations();
        assert_eq!(result, 46);
    }

    #[test]
    fn test_get_seeds_destinations_part_1() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let almanac = input.parse::<Almanac1>().unwrap();
        assert_eq!(almanac.get_seeds_destinations(), 35);
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            "};

        let result = input.parse::<Almanac1>().unwrap();
        assert_eq!(
            result,
            Almanac1 {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    Map {
                        from: "seed".to_string(),
                        to: "soil".to_string(),
                        conversions: vec![
                            MapConversion {
                                destination_start: 50,
                                source_start: 98,
                                length: 2
                            },
                            MapConversion {
                                destination_start: 52,
                                source_start: 50,
                                length: 48
                            }
                        ],
                    },
                    Map {
                        from: "soil".to_string(),
                        to: "fertilizer".to_string(),
                        conversions: vec![
                            MapConversion {
                                destination_start: 0,
                                source_start: 15,
                                length: 37,
                            },
                            MapConversion {
                                destination_start: 37,
                                source_start: 52,
                                length: 2,
                            },
                            MapConversion {
                                destination_start: 39,
                                source_start: 0,
                                length: 15,
                            },
                        ],
                    }
                ],
            }
        )
    }

    #[test]
    fn test_convert_to_destination() {
        let tests = vec![
            (0, 0),
            (1, 1),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (96, 98),
            (97, 99),
            (98, 50),
            (99, 51),
        ];

        let map = Map {
            from: "".to_string(),
            to: "".to_string(),
            conversions: vec![
                MapConversion {
                    destination_start: 50,
                    source_start: 98,
                    length: 2,
                },
                MapConversion {
                    destination_start: 52,
                    source_start: 50,
                    length: 48,
                },
            ],
        };

        for test in tests {
            assert_eq!(map.get_destination(test.0), test.1);
        }
    }

    #[test]
    fn test_map_from_str() {
        let input = "\nseed-to-soil map:\n50 98 2\n52 50 48\n";

        assert_eq!(
            Map::from_str(input).unwrap(),
            Map {
                from: "seed".to_string(),
                to: "soil".to_string(),
                conversions: vec![
                    MapConversion {
                        destination_start: 50,
                        source_start: 98,
                        length: 2
                    },
                    MapConversion {
                        destination_start: 52,
                        source_start: 50,
                        length: 48
                    }
                ]
            }
        );
    }

    #[test]
    fn test_map_conversion_from_str() {
        assert_eq!(
            MapConversion::from_str("0 15 37").unwrap(),
            MapConversion {
                destination_start: 0,
                source_start: 15,
                length: 37
            }
        );

        assert_eq!(
            MapConversion::from_str("1 2 3").unwrap(),
            MapConversion {
                destination_start: 1,
                source_start: 2,
                length: 3
            }
        );
    }
}
