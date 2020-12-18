fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("None");

    let result = match problem {
        "day1a" => "TBD",
        _ => "We haven't solved that problem yet."
    };

    println!("{}", result);
}
