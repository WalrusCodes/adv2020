fn main() {
    let input = std::env::args().nth(1).expect("no arg");
    let contents = std::fs::read_to_string(input).expect("read failed");

    let items = contents
        .lines()
        .map(|s| s.parse::<i32>().expect("failed to parse"))
        .collect::<Vec<i32>>();
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if (items[i] + items[j]) == 2020 {
                println!(
                    "{} + {} == 2020 {} * {} = {}",
                    items[i],
                    items[j],
                    items[i],
                    items[j],
                    items[i] * items[j]
                );
            }
        }
    }
}
