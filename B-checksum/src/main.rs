use std::cmp;
use std::io::{self, Read};

fn checksum(input: &str) -> u32 {
    input.split('\n').filter_map(|line| {
        let mut values = line.split_whitespace().filter_map(|v| v.parse::<u32>().ok());
        values.next().map(|initial| {
            let (min, max) = values.fold((initial, initial),
                                         |(min, max), v| (cmp::min(min, v), cmp::max(max, v)));
            max - min
        })
    }).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result = checksum(&input);
    println!("Checksum: {}", result);
}

#[test]
fn example() {
    let example = "5 1 9 5\n7 5 3\n2 4 6 8";
    assert_eq!(checksum(example), 18);
}
