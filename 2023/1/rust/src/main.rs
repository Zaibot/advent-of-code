fn main() {
    println!("Hello, world!");
}

#[test]
fn example_input() {
    let expected = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];
}

#[test]
fn eval_input() {
    let lines = std::fs::read_to_string("../input.txt").expect("Unable to read file");
    let lines = lines.lines();
}
