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
