fn main() {
    println!("Hello, world!");
}

#[test]
fn test_example_input() {
    #[rustfmt::skip]
    let expected = [
        (true,  "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        (true,  "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        (false, "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
        (false, "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
        (true,  "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
    ];

    let bag_red = 12;
    let bag_green = 13;
    let bag_blue = 14;
}
