fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Table {
    name: String,
    rows: Vec<Vec<i32>>,
}

impl Table {
    fn new(name: impl AsRef<str>) -> Table {
        Table {
            name: name.as_ref().to_string(),
            rows: Vec::new(),
        }
    }

    fn add(&mut self, row: Vec<i32>) {
        assert_eq!(
            row.len(),
            self.rows.first().unwrap_or(&row).len(),
            "Row length mismatch"
        );

        self.rows.push(row);
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        self.rows[row][col]
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

    while let Some(line) = reader.next() {
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

fn parse_row(text: &str) -> Vec<i32> {
    text.split_ascii_whitespace()
        .map(|s| {
            s.trim()
                .parse()
                .expect(format!("Expected integer but got: {:#?}", s).as_str())
        })
        .collect()
}

#[test]
fn test_stage_1() {
    let input = r#"
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

    let mut lines = input.lines().map(|s| s.to_string()).peekable();

    let mut tables = Vec::new();
    while let Some(line) = lines.peek() {
        if line.is_empty() {
            lines.next();
        } else {
            let table = parse_table(&mut lines);
            println!("{:?}", table);
            tables.push(table);
        }
    }

    assert_eq!(tables.len(), 8);
    assert_eq!(tables[0].name, "seeds");
    assert_eq!(tables[0].get(0, 0), 79);
    assert_eq!(tables[7].name, "humidity-to-location map");
    assert_eq!(tables[7].get(0, 1), 56);
}
