use std::{env, slice};

struct ParseDigits<I>(I);

impl<I> Iterator for ParseDigits<I>
    where I: Iterator<Item=u8>
{
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.0.next().and_then(|c| {
            match c {
                b'0' => Some(0),
                b'1' => Some(1),
                b'2' => Some(2),
                b'3' => Some(3),
                b'4' => Some(4),
                b'5' => Some(5),
                b'6' => Some(6),
                b'7' => Some(7),
                b'8' => Some(8),
                b'9' => Some(9),
                _ => None,
            }
        })
    }
}

fn sum_matching<A, B>(a: A, b: B) -> u32
    where A: Iterator<Item=u8>,
          B: Iterator<Item=u8>,
{
    ParseDigits(a).zip(ParseDigits(b)).filter_map(|(l, r)| {
        if l == r {
            Some(l)
        } else {
            None
        }
    }).sum()
}

struct CircularChars<'a>(slice::Iter<'a, u8>, slice::Iter<'a, u8>);

impl<'a> Iterator for CircularChars<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.0.next().or_else(|| self.1.next()).cloned()
    }
}

fn sum_next(input: &str) -> u32 {
    let (l, r) = input.as_bytes().split_at(1);
    let cc = CircularChars(r.iter(), l.iter());
    sum_matching(input.bytes(), cc)
}

fn sum_halfway(input: &str) -> u32 {
    let (l, r) = input.as_bytes().split_at(input.as_bytes().len() / 2);
    let cc = CircularChars(r.iter(), l.iter());
    sum_matching(input.bytes(), cc)
}

#[test]
fn test_examples() {
    let examples = [
        ("1122", 3),
        ("1111", 4),
        ("1234", 0),
        ("91212129", 9),
    ];
    for &(input, expected) in &examples {
        assert_eq!(sum_next(input), expected);
    }

    let examples = [
        ("1212", 6),
        ("1221", 0),
        ("123425", 4),
        ("123123", 12),
        ("12131415", 4),
    ];
    for &(input, expected) in &examples {
        assert_eq!(sum_halfway(input), expected);
    }
}

fn main() {
    let arg = env::args().nth(1).unwrap();
    println!("{}", sum_next(&arg));
    println!("{}", sum_halfway(&arg));
}
