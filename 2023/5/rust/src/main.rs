use std::cell::{Cell, UnsafeCell};

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Failed to read ../input.txt");

    let tables = parse_input(&input);
    let seed_to_location = SeedToLocation::from_tables(&tables);

    {
        let seeds = seed_to_location.seeds();
        let seed_locations = seeds
            .iter()
            .map(|&seed| seed_to_location.seed_to_location(seed))
            .collect::<Vec<_>>();

        println!("Lowest location: {}", seed_locations.iter().min().unwrap());
    }

    {
        let start = std::time::Instant::now();
        let seed_ranges = seed_to_location.seed_ranges();
        let seed_locations = seed_to_location.seed_ranges_to_locations(&seed_ranges);
        println!("Elapsed: {:?}", start.elapsed());

        println!(
            "Lowest location by seed ranges: {}",
            seed_locations.iter().min().unwrap()
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Row {
    cells: Vec<i64>,
}

impl Row {
    fn new(cells: &[i64]) -> Row {
        Row {
            cells: cells.to_vec(),
        }
    }

    fn cells(&self) -> impl Iterator<Item = &i64> {
        self.cells.iter()
    }

    fn cell(&self, index: usize) -> i64 {
        self.cells[index]
    }
}

impl AsRef<Vec<i64>> for Row {
    fn as_ref(&self) -> &Vec<i64> {
        &self.cells
    }
}

#[derive(Debug, Clone)]
struct Table {
    name: String,
    rows: Vec<Row>,
}

impl Table {
    fn new(name: impl AsRef<str>) -> Table {
        Table {
            name: name.as_ref().to_string(),
            rows: Vec::new(),
        }
    }

    fn add(&mut self, row: Row) {
        assert_eq!(
            row.as_ref().len(),
            self.rows.first().unwrap_or(&row).as_ref().len(),
            "Row length mismatch"
        );

        self.rows.push(row);
    }

    fn row(&self, index: usize) -> &Row {
        &self.rows[index]
    }
}

struct MapTable {
    last: Cell<(i64, i64, i64)>,
    data: Box<[i64]>,
}

impl From<Table> for MapTable {
    fn from(table: Table) -> MapTable {
        assert_eq!(table.row(0).cells().count(), 3);

        let mut data = Vec::new();

        for row in table.rows {
            data.push(row.source_range().start);
            data.push(row.source_range().end);
            data.push(row.destination_range().start);
        }

        MapTable {
            last: Cell::new((0, 0, 0)),
            data: data.into_boxed_slice(),
        }
    }
}

impl MapSourceDestination for MapTable {
    fn map_source_destination(&self, source: i64) -> i64 {
        {
            let (src_start, src_end, dst_start) = self.last.get();
            if src_start <= source && source < src_end {
                return dst_start + source - src_start;
            }
        }

        for i in (0..self.data.len()).step_by(3) {
            let src_start = self.data[i];
            let src_end = self.data[i + 1];
            let dst_start = self.data[i + 2];
            if src_start <= source && source < src_end {
                self.last.replace((src_start, src_end, dst_start));
                return dst_start + source - src_start;
            }
        }
        source
    }
}

trait TableLookup {
    fn by_name(&self, name: impl AsRef<str>) -> Option<&Table>;
}

impl<T> TableLookup for T
where
    T: AsRef<[Table]>,
{
    fn by_name(&self, name: impl AsRef<str>) -> Option<&Table> {
        self.as_ref()
            .iter()
            .find(|table| table.name == name.as_ref())
    }
}

fn parse_table(reader: &mut impl Iterator<Item = String>) -> Table {
    let line = reader.next().unwrap();

    let Some((name, first_row)) = line.split_once(':') else {
        panic!("Expected table name but got: {:#?}", line);
    };

    let mut table = Table::new(name);
    let line = first_row.to_string();

    if !line.is_empty() {
        table.add(parse_row(&line));
    }

    for line in reader {
        match line.as_str() {
            "" => {
                break;
            }
            line => {
                table.add(parse_row(line));
            }
        }
    }

    table
}

fn parse_row(text: &str) -> Row {
    Row {
        cells: text
            .split_ascii_whitespace()
            .map(|s| {
                s.trim()
                    .parse()
                    .expect(format!("Expected integer but got: {:#?}", s).as_str())
            })
            .collect(),
    }
}

fn parse_input(text: &str) -> Vec<Table> {
    let mut tables = Vec::new();

    let mut lines = text.lines().map(|s| s.to_string()).peekable();
    while let Some(line) = lines.peek() {
        if line.is_empty() {
            lines.next();
        } else {
            let table = parse_table(&mut lines);
            tables.push(table);
        }
    }

    tables
}
trait TableMap {
    fn by_source(&self, source: i64) -> Option<&Row>;
    fn by_destination(&self, destination: i64) -> Option<&Row>;
}

impl TableMap for Table {
    fn by_source(&self, source: i64) -> Option<&Row> {
        self.rows
            .iter()
            .find(|&row| row.source_range().contains(&source))
    }

    fn by_destination(&self, destination: i64) -> Option<&Row> {
        self.rows
            .iter()
            .find(|&row| row.destination_range().contains(&destination))
    }
}

trait SomeMap {
    fn range(&self) -> i64;
    fn source_start(&self) -> i64;
    fn destination_start(&self) -> i64;

    fn source_range(&self) -> std::ops::Range<i64> {
        self.source_start()..self.source_start() + self.range()
    }

    fn destination_range(&self) -> std::ops::Range<i64> {
        self.destination_start()..self.destination_start() + self.range()
    }
}

impl SomeMap for Row {
    fn range(&self) -> i64 {
        self.cells[2]
    }

    fn source_start(&self) -> i64 {
        self.cells[1]
    }

    fn destination_start(&self) -> i64 {
        self.cells[0]
    }
}

trait MapSourceDestination {
    fn map_source_destination(&self, source: i64) -> i64;
}

impl MapSourceDestination for Table {
    fn map_source_destination(&self, source: i64) -> i64 {
        match self.by_source(source) {
            Some(src) => src.destination_start() + source - src.source_start(),
            None => source,
        }
    }
}

struct SeedToLocation {
    seeds: Table,
    seed_to_soil: MapTable,
    soil_to_fertilizer: MapTable,
    fertilizer_to_water: MapTable,
    water_to_light: MapTable,
    light_to_temperature: MapTable,
    temperature_to_humidity: MapTable,
    humidity_to_location: MapTable,
}

impl SeedToLocation {
    fn from_tables(tables: &[Table]) -> SeedToLocation {
        SeedToLocation {
            seeds: tables
                .by_name("seeds")
                .expect("seeds table not found")
                .clone()
                .into(),
            seed_to_soil: tables
                .by_name("seed-to-soil map")
                .expect("seed-to-soil map not found")
                .clone()
                .into(),
            soil_to_fertilizer: tables
                .by_name("soil-to-fertilizer map")
                .expect("soil-to-fertilizer map not found")
                .clone()
                .into(),
            fertilizer_to_water: tables
                .by_name("fertilizer-to-water map")
                .expect("fertilizer-to-water map not found")
                .clone()
                .into(),
            water_to_light: tables
                .by_name("water-to-light map")
                .expect("water-to-light map not found")
                .clone()
                .into(),
            light_to_temperature: tables
                .by_name("light-to-temperature map")
                .expect("light-to-temperature map not found")
                .clone()
                .into(),
            temperature_to_humidity: tables
                .by_name("temperature-to-humidity map")
                .expect("temperature-to-humidity map not found")
                .clone()
                .into(),
            humidity_to_location: tables
                .by_name("humidity-to-location map")
                .expect("humidity-to-location map not found")
                .clone()
                .into(),
        }
    }

    fn seeds(&self) -> Vec<i64> {
        self.seeds.row(0).cells().copied().collect()
    }

    fn seed_ranges(&self) -> Vec<std::ops::Range<i64>> {
        self.seeds()
            .chunks_exact(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect()
    }

    fn seed_to_soil(&self, seed: i64) -> i64 {
        self.seed_to_soil.map_source_destination(seed)
    }

    fn soil_to_fertilizer(&self, soil: i64) -> i64 {
        self.soil_to_fertilizer.map_source_destination(soil)
    }

    fn fertilizer_to_water(&self, fertilizer: i64) -> i64 {
        self.fertilizer_to_water.map_source_destination(fertilizer)
    }

    fn water_to_light(&self, water: i64) -> i64 {
        self.water_to_light.map_source_destination(water)
    }

    fn light_to_temperature(&self, light: i64) -> i64 {
        self.light_to_temperature.map_source_destination(light)
    }

    fn temperature_to_humidity(&self, temperature: i64) -> i64 {
        self.temperature_to_humidity
            .map_source_destination(temperature)
    }

    fn humidity_to_location(&self, humidity: i64) -> i64 {
        self.humidity_to_location.map_source_destination(humidity)
    }

    fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil(seed);
        let fertilizer = self.soil_to_fertilizer(soil);
        let water = self.fertilizer_to_water(fertilizer);
        let light = self.water_to_light(water);
        let temperature = self.light_to_temperature(light);
        let humidity = self.temperature_to_humidity(temperature);

        self.humidity_to_location(humidity)
    }

    fn seed_ranges_to_locations(&self, seed_ranges: &[std::ops::Range<i64>]) -> Vec<i64> {
        seed_ranges
            .iter()
            .flat_map(|seed_range| {
                println!("seed_range: {:#?}", seed_range);
                seed_range.clone()
            })
            .map(|seed| self.seed_to_location(seed))
            .collect()
    }
}

#[test]
fn test_input_1() {
    let tables = parse_input(INPUT_1);

    assert_eq!(tables.len(), 8);
    assert_eq!(tables[0].name, "seeds");
    assert_eq!(tables[0].row(0).cell(0), 79);
    assert_eq!(tables[7].name, "humidity-to-location map");
    assert_eq!(tables[7].row(0).cell(1), 56);
}

#[test]
fn test_stage_1_seed_to_soil() {
    let tables = parse_input(INPUT_1);
    let seed_to_location = SeedToLocation::from_tables(&tables);

    let seed_soil = seed_to_location
        .seeds()
        .iter()
        .map(|&seed| (seed, seed_to_location.seed_to_soil(seed)))
        .collect::<Vec<_>>();

    assert_eq!(seed_soil[0].0, 79, "{:#?}", seed_soil[0]);
    assert_eq!(seed_soil[0].1, 81, "{:#?}", seed_soil[0]);

    assert_eq!(seed_soil[1].0, 14, "{:#?}", seed_soil[1]);
    assert_eq!(seed_soil[1].1, 14, "{:#?}", seed_soil[1]);

    assert_eq!(seed_soil[2].0, 55, "{:#?}", seed_soil[2]);
    assert_eq!(seed_soil[2].1, 57, "{:#?}", seed_soil[2]);

    assert_eq!(seed_soil[3].0, 13, "{:#?}", seed_soil[3]);
    assert_eq!(seed_soil[3].1, 13, "{:#?}", seed_soil[3]);
}

#[test]
fn test_stage_2_seed_ranges() {
    let tables = parse_input(INPUT_1);
    let seed_to_location = SeedToLocation::from_tables(&tables);
    let seed_range = seed_to_location.seed_ranges();

    assert_eq!(seed_range[0], 79..93);
    assert_eq!(seed_range[1], 55..68);
}

#[test]
fn test_stage_2_seed_ranges_to_locations() {
    let tables = parse_input(INPUT_1);
    let seed_to_location = SeedToLocation::from_tables(&tables);

    let seed_ranges = seed_to_location.seed_ranges();
    let seed_locations = seed_to_location.seed_ranges_to_locations(&seed_ranges);

    let lowest_location = seed_locations.into_iter().min().unwrap();

    assert_eq!(lowest_location, 46);
}

#[cfg(test)]
const INPUT_1: &str = r#"
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
"#;
