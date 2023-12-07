use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day5.txt").expect("Failed to open input");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read input");

    let mut first_ans = u64::MAX;
    let mut second_ans = u64::MAX;

    let mut sections = s.split("\n\n").into_iter();
    let mut values = read_seeds(sections.next().unwrap());

    for section in sections {
        let map = Map::parse(section);
        for i in 0..values.len() {
            map.convert(&mut values[i]);
        }
    }

    for value in values {
        first_ans = first_ans.min(value[MapType::Location.to_idx() + 1]);
    }

    let mut sections = s.split("\n\n").into_iter();
    let mut values = read_seeds_ranges(sections.next().unwrap());

    for section in sections {
        let map = Map::parse(section);
        values = map.convert_values_ranges(values);
        if map.m_type == MapType::Location {
            break;
        }
    }

    for value in values {
        second_ans = second_ans.min(value.min());
    }

    println!("{}", first_ans);
    println!("{}", second_ans);
}

#[derive(Debug, PartialEq)]
struct Map {
    m_type: MapType,
    ranges: Vec<MapRange>,
}

impl Map {
    fn parse(section: &str) -> Self {
        let lines = section.lines().collect::<Vec<_>>();
        let first_line = *lines.first().unwrap();

        let m_type = MapType::new(first_line.split_whitespace().nth(0).unwrap());

        let ranges = lines
            .iter()
            .skip(1)
            .map(|line| MapRange::new(line))
            .collect();

        Self { m_type, ranges }
    }

    fn convert(&self, values: &mut Vec<u64>) {
        let old_value = values[self.m_type.to_idx()];

        for range in &self.ranges {
            if let Ok(new_value) = range.try_convert(old_value) {
                values.push(new_value);
                return;
            }
        }
        values.push(old_value);
    }

    fn convert_values_ranges(&self, values: Vec<ValueRange>) -> Vec<ValueRange> {
        let mut new_values = Vec::new();

        for range in values {
            let mut cur_left = vec![range];

            for map_range in &self.ranges {
                let mut temp_left = Vec::new();
                for r in cur_left {
                    if let Ok((new_range, leftover)) = map_range.try_convert_value_range(r) {
                        new_values.push(new_range);
                        if leftover.len() > 0 {
                            temp_left.extend(leftover);
                        }
                    } else {
                        temp_left.push(r);
                    }
                }
                cur_left = temp_left;
            }

            new_values.extend(cur_left);
        }

        new_values
    }
}

#[derive(Debug, PartialEq)]
enum MapType {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl MapType {
    fn new(s: &str) -> Self {
        match s.split("-").last() {
            Some("soil") => MapType::Soil,
            Some("fertilizer") => MapType::Fertilizer,
            Some("water") => MapType::Water,
            Some("light") => MapType::Light,
            Some("temperature") => MapType::Temperature,
            Some("humidity") => MapType::Humidity,
            Some("location") => MapType::Location,
            _ => panic!("Invalid map type"),
        }
    }

    fn to_idx(&self) -> usize {
        match self {
            MapType::Soil => 0,
            MapType::Fertilizer => 1,
            MapType::Water => 2,
            MapType::Light => 3,
            MapType::Temperature => 4,
            MapType::Humidity => 5,
            MapType::Location => 6,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRange {
    fn new(line: &str) -> Self {
        let line = line.split_whitespace().collect::<Vec<_>>();

        let destination_start = line[0].parse::<u64>().unwrap();
        let source_start = line[1].parse::<u64>().unwrap();
        let length = line[2].parse::<u64>().unwrap();

        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn try_convert(&self, value: u64) -> Result<u64, ()> {
        if value < self.source_start || value >= self.source_start + self.length {
            return Err(());
        }
        Ok(value - self.source_start + self.destination_start)
    }

    fn try_convert_value_range(
        &self,
        mut value: ValueRange,
    ) -> Result<(ValueRange, Vec<ValueRange>), ()> {
        if value.end <= self.source_start || value.start >= self.source_start + self.length {
            return Err(());
        }

        let mut new_values = Vec::new();

        if value.start < self.source_start {
            let (new_range, leftover) = value.split(self.source_start);
            new_values.push(new_range);
            value = leftover;
        }

        if value.end > self.source_start + self.length {
            let (leftover, new_range) = value.split(self.source_start + self.length);
            new_values.push(new_range);
            value = leftover;
        }

        let new_start_val = self.try_convert(value.start).unwrap();

        let new_range = ValueRange::new(new_start_val, new_start_val + (value.end - value.start));

        Ok((new_range, new_values))
    }
}

// start inclusive, end non-inclusive
#[derive(Debug, PartialEq, Copy, Clone)]
struct ValueRange {
    start: u64,
    end: u64,
}

impl ValueRange {
    fn new(start: u64, end: u64) -> Self {
        if start >= end {
            panic!("Invalid value range");
        }
        Self { start, end }
    }

    fn min(&self) -> u64 {
        self.start
    }

    fn split(&self, point: u64) -> (ValueRange, ValueRange) {
        let new_range = ValueRange::new(self.start, point);
        let leftover = ValueRange::new(point, self.end);
        (new_range, leftover)
    }
}

fn read_seeds(s: &str) -> Vec<Vec<u64>> {
    s.split_whitespace()
        .skip(1)
        .map(|s| vec![s.parse::<u64>().unwrap()])
        .collect()
}

fn read_seeds_ranges(s: &str) -> Vec<ValueRange> {
    let seeds = s.split_whitespace().skip(1).collect::<Vec<_>>();

    let mut i = 0;

    let mut values = Vec::new();

    while i < seeds.len() {
        let start = seeds[i].parse::<u64>().unwrap();
        let length = seeds[i + 1].parse::<u64>().unwrap();

        values.push(ValueRange::new(start, start + length));

        i += 2;
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let section = "seed-to-soil map:\n50 98 2\n52 50 48";

        let map = Map::parse(section);

        assert_eq!(
            map,
            Map {
                m_type: MapType::Soil,
                ranges: vec![
                    MapRange {
                        source_start: 98,
                        destination_start: 50,
                        length: 2
                    },
                    MapRange {
                        source_start: 50,
                        destination_start: 52,
                        length: 48
                    },
                ]
            }
        );
    }

    #[test]
    fn test_convert_map() {
        let map = Map {
            m_type: MapType::Soil,
            ranges: vec![
                MapRange {
                    source_start: 98,
                    destination_start: 50,
                    length: 2,
                },
                MapRange {
                    source_start: 50,
                    destination_start: 52,
                    length: 48,
                },
            ],
        };

        let mut values = vec![98];

        map.convert(&mut values);

        assert_eq!(values, vec![98, 50]);
    }

    #[test]
    fn test_read_seeds_ranges() {
        let s = "seeds: 79 14 55 13";

        let values = read_seeds_ranges(s);

        assert_eq!(
            values,
            vec![ValueRange::new(79, 93), ValueRange::new(55, 68),]
        );
    }

    #[test]
    fn test_convert_values_ranges() {
        let map = Map {
            m_type: MapType::Soil,
            ranges: vec![
                MapRange {
                    source_start: 98,
                    destination_start: 50,
                    length: 2,
                },
                MapRange {
                    source_start: 50,
                    destination_start: 52,
                    length: 48,
                },
            ],
        };

        let values = vec![ValueRange::new(79, 79 + 14), ValueRange::new(55, 55 + 13)];

        let new_values = map.convert_values_ranges(values);

        assert_eq!(
            new_values,
            vec![ValueRange::new(81, 81 + 14), ValueRange::new(57, 57 + 13),]
        );
    }

    #[test]
    fn test_try_convert_value_ranges() {
        let map_range = MapRange {
            source_start: 90,
            destination_start: 50,
            length: 2,
        };

        let value_range = ValueRange::new(79, 79 + 14);

        let (new_range, remaining_values) = map_range.try_convert_value_range(value_range).unwrap();

        assert_eq!(new_range, ValueRange::new(50, 52));
        assert_eq!(
            remaining_values,
            vec![ValueRange::new(79, 90), ValueRange::new(92, 93)]
        );

        let map_range = MapRange {
            source_start: 50,
            destination_start: 52,
            length: 48,
        };

        let value_range = ValueRange::new(79, 79 + 14);

        let (new_range, remaining_values) = map_range.try_convert_value_range(value_range).unwrap();

        assert_eq!(new_range, ValueRange::new(81, 95));
        assert_eq!(remaining_values, vec![]);
    }
}
