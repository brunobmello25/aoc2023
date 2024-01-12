use std::{error::Error, fs, str::FromStr};

#[derive(Debug, PartialEq)]
struct MapConversion {
    source_start: usize,
    destination_start: usize,
    length: usize,
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
    fn new(conversion_strings: &[&str]) -> Result<Self, String> {
        let mut conversions = Vec::new();

        for &conversion_str in conversion_strings {
            let conversion_map = MapConversion::from_str(conversion_str).unwrap();
            conversions.push(conversion_map);
        }

        Ok(Map { conversions })
    }

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

        let title = title.replace(" map", "");
        let (from, to) = match s.trim().split_once("-to-") {
            Some((from, to)) => (from, to),
            _ => return Err("Invalid input".into()),
        };

        return Ok(Map {
            from: from.into(),
            to: to.into(),
            conversions: Vec::new(),
        });
    }
}

#[allow(dead_code)]
pub fn run() {
    run_part_1();
}

#[allow(dead_code)]
fn run_part_1() {
    let contents = fs::read_to_string("input/day5.txt").unwrap();
    let contents = contents.trim();

    println!("{:?}", contents);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day5::MapConversion;

    use super::Map;

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

        let conversions = Map::new(&["50 98 2", "52 50 48"]).unwrap();

        for test in tests {
            assert_eq!(conversions.get_destination(test.0), test.1);
        }
    }

    #[test]
    fn test_map_from_str() {
        let input = "\nseed-to-soil map:\n50 98 2\n52 50 48\n";

        assert_eq!(
            Map::from_str(input).unwrap(),
            Map {
                conversions: vec![
                    MapConversion {
                        destination_start: 98,
                        source_start: 50,
                        length: 2
                    },
                    MapConversion {
                        destination_start: 50,
                        source_start: 52,
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
