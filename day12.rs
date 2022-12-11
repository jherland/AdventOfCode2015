use std::io;

fn traverse_numbers(parsed: &json::JsonValue, sink: &mut Vec<i64>, ignore_red: bool) {
    use json::JsonValue::*;
    match parsed {
        Object(obj) => {
            if !ignore_red || !obj.iter().any(|(_, obj)| obj.is_string() && obj == "red") {
                obj.iter()
                    .for_each(|(_, obj)| traverse_numbers(obj, sink, ignore_red));
            }
        }
        Array(objs) => {
            objs.iter()
                .for_each(|obj| traverse_numbers(obj, sink, ignore_red));
        }
        Number(n) => {
            sink.push(n.as_fixed_point_i64(0).unwrap());
        }
        _ => (),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line from stdin!");
    let parsed = json::parse(&input).unwrap();

    let mut numbers = Vec::new();
    traverse_numbers(&parsed, &mut numbers, false);
    println!("Part 1: {}", numbers.iter().sum::<i64>());

    let mut numbers = Vec::new();
    traverse_numbers(&parsed, &mut numbers, true);
    println!("Part 2: {}", numbers.iter().sum::<i64>());
}
