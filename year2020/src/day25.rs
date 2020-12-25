use std::unimplemented;

/// https://adventofcode.com/2020/day/25

#[aoc_generator(day25)]
fn parse_input(input: &str) -> (usize, usize) {
    let mut l = input.lines();
    let card = l.next().unwrap().parse::<usize>().unwrap();
    let door = l.next().unwrap().parse::<usize>().unwrap();
    (card, door)
}

fn loop_size(key: &usize, subject_number: usize) -> usize {
    let mut l: usize = 0;
    let mut value:usize = 1;
    loop {
        l += 1;
        value *= subject_number;
        value %= 20201227;
        if &value == key {
            break l;
        }
    }
}

fn encryption_key(key: &usize, loop_size: usize) -> usize {
    let mut value: usize = 1;
    for _ in 0..loop_size {
        value *= key;
        value %= 20201227;
    }
    value
}

#[aoc(day25, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (card, door) = input;
    let l = loop_size(card, 7);
    encryption_key(door, l)
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "5764801
17807724";

    #[test]
    fn parsing_input(){
        assert_eq!(parse_input(INPUT), (5764801, 17807724));
    }

    #[test]
    fn test_loop_size() {
        assert_eq!(loop_size(&5764801, 7), 8);
        assert_eq!(loop_size(&17807724, 7), 11);
    }

    #[test]
    fn test_encryption_key() {
        assert_eq!(encryption_key(&5764801, 11), 14897079);
        assert_eq!(encryption_key(&17807724, 8), 14897079);
    }

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 14897079)
    }
}