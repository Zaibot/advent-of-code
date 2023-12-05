#![allow(dead_code)]

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Failed to read ../input.txt");

    let tables = parse_input(&input);

    let seed_location = seeds(&tables)
        .iter()
        .copied()
        .map(|seed| seed_to_location(&tables, seed))
        .collect::<Vec<_>>();

    println!("Lowest location: {}", seed_location.iter().min().unwrap());
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

    fn get(&self, cell: Coord) -> i64 {
        self.rows[cell.1].as_ref()[cell.0]
    }

    fn row(&self, index: usize) -> &Row {
        &self.rows[index]
    }

    fn rows(&self) -> &Vec<Row> {
        &self.rows
    }
}

impl AsRef<Vec<Row>> for Table {
    fn as_ref(&self) -> &Vec<Row> {
        &self.rows
    }
}

struct Coord(usize, usize);

trait TableLookup {
    fn by_name(&self, name: impl AsRef<str>) -> Option<&Table>;

    fn cell_by_name(&mut self, table_name: impl AsRef<str>, cell: Coord) -> i64 {
        self.by_name(table_name.as_ref())
            .expect(format!("Table not found: {:#?}", table_name.as_ref()).as_str())
            .get(cell)
    }
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

#[test]
fn test_input_1() {
    let tables = parse_input(INPUT_1);

    assert_eq!(tables.len(), 8);
    assert_eq!(tables[0].name, "seeds");
    assert_eq!(tables[0].get(Coord(0, 0)), 79);
    assert_eq!(tables[7].name, "humidity-to-location map");
    assert_eq!(tables[7].get(Coord(0, 1)), 56);
}

trait TableMap {
    fn by_source(&self, source: i64) -> Option<&Row>;
    fn by_destination(&self, destination: i64) -> Option<&Row>;
}

impl TableMap for Table {
    fn by_source(&self, source: i64) -> Option<&Row> {
        self.rows
            .iter()
            .find(|row| row.source_range().contains(&source))
    }

    fn by_destination(&self, destination: i64) -> Option<&Row> {
        self.rows
            .iter()
            .find(|row| row.destination_range().contains(&destination))
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

fn seeds(tables: &[Table]) -> Vec<i64> {
    tables
        .by_name("seeds")
        .expect("seeds table not found")
        .row(0)
        .cells()
        .copied()
        .collect()
}

fn seed_to_soil(tables: &[Table], seed: i64) -> i64 {
    let soil = tables
        .by_name("seed-to-soil map")
        .expect("seed-to-soil map not found");

    match soil.by_source(seed) {
        Some(src) => src.destination_start() + seed - src.source_start(),
        None => seed,
    }
}

fn soil_to_fertilizer(tables: &[Table], soil: i64) -> i64 {
    let fertilizer = tables
        .by_name("soil-to-fertilizer map")
        .expect("soil-to-fertilizer map not found");

    match fertilizer.by_source(soil) {
        Some(src) => src.destination_start() + soil - src.source_start(),
        None => soil,
    }
}

fn fertilizer_to_water(tables: &[Table], fertilizer: i64) -> i64 {
    let water = tables
        .by_name("fertilizer-to-water map")
        .expect("fertilizer-to-water map not found");

    match water.by_source(fertilizer) {
        Some(src) => src.destination_start() + fertilizer - src.source_start(),
        None => fertilizer,
    }
}

fn water_to_light(tables: &[Table], water: i64) -> i64 {
    let light = tables
        .by_name("water-to-light map")
        .expect("water-to-light map not found");

    match light.by_source(water) {
        Some(src) => src.destination_start() + water - src.source_start(),
        None => water,
    }
}

fn light_to_temperature(tables: &[Table], light: i64) -> i64 {
    let temperature = tables
        .by_name("light-to-temperature map")
        .expect("light-to-temperature map not found");

    match temperature.by_source(light) {
        Some(src) => src.destination_start() + light - src.source_start(),
        None => light,
    }
}

fn temperature_to_humidity(tables: &[Table], temperature: i64) -> i64 {
    let humidity = tables
        .by_name("temperature-to-humidity map")
        .expect("temperature-to-humidity map not found");

    match humidity.by_source(temperature) {
        Some(src) => src.destination_start() + temperature - src.source_start(),
        None => temperature,
    }
}

fn humidity_to_location(tables: &[Table], humidity: i64) -> i64 {
    let location = tables
        .by_name("humidity-to-location map")
        .expect("humidity-to-location map not found");

    match location.by_source(humidity) {
        Some(src) => src.destination_start() + humidity - src.source_start(),
        None => humidity,
    }
}

fn seed_to_location(tables: &[Table], seed: i64) -> i64 {
    let soil = seed_to_soil(tables, seed);
    let fertilizer = soil_to_fertilizer(tables, soil);
    let water = fertilizer_to_water(tables, fertilizer);
    let light = water_to_light(tables, water);
    let temperature = light_to_temperature(tables, light);
    let humidity = temperature_to_humidity(tables, temperature);
    let location = humidity_to_location(tables, humidity);

    location
}

#[test]
fn test_stage_1_seed_to_soil() {
    let tables = parse_input(INPUT_1);

    let seed_soil = seeds(&tables)
        .iter()
        .copied()
        .map(|seed| (seed, seed_to_soil(&tables, seed)))
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
fn test_stage_1_seed_to_location() {
    let tables = parse_input(INPUT_1);

    let seed_location = seeds(&tables)
        .iter()
        .copied()
        .map(|seed| seed_to_location(&tables, seed))
        .collect::<Vec<_>>();

    assert_eq!(seed_location, vec![82, 43, 86, 35]);
}

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
