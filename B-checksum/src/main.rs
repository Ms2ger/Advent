use std::{cmp, iter, str};
use std::io::{self, Read};

fn parse_int(input: &str) -> Option<u32> {
    input.parse().ok()
}

type LineIter<'a> = iter::FilterMap<str::SplitWhitespace<'a>, fn(&str) -> Option<u32>>;

fn sum_lines<'a, F>(input: &'a str, mut line_handler: F) -> u32
    where F: FnMut(LineIter<'a>) -> Option<u32>
{
    input.split('\n').filter_map(|line| {
        line_handler(line.split_whitespace().filter_map(parse_int))
    }).sum()
}

fn checksum(input: &str) -> u32 {
    sum_lines(input, |mut values| {
        values.next().map(|initial| {
            let (min, max) = values.fold((initial, initial),
                                         |(min, max), v| (cmp::min(min, v), cmp::max(max, v)));
            max - min
        })
    })
}

fn divisible_values(input: &str) -> u32 {
    sum_lines(input, |values| {
        let values = values.collect::<Vec<_>>();
        values.iter().filter_map(|&v1| {
            values.iter().find(|&&v2| v1 != v2 && v2 % v1 == 0).map(|&v2| v2 / v1)
        }).next()
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result = checksum(&input);
    println!("Checksum: {}", result);
    let result = divisible_values(&input);
    println!("Divisible values: {}", result);
}

#[test]
fn example() {
    let example = "5 1 9 5\n7 5 3\n2 4 6 8";
    assert_eq!(checksum(example), 18);

    let example = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    assert_eq!(divisible_values(example), 9);
}
