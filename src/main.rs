mod days;

fn main() {
    let name: String = std::env::args()
        .next()
        .expect("Script has no name?! Not sure what sorcery is going on here :3");

    let day: i64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: {name} <day> [--example]"));

    let example: bool = std::env::args()
        .nth(2)
        .filter(|s| s == "--example")
        .is_some();

    days::run_day(day, example);
}
