/// https://adventofcode.com/2020/day/13

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (usize, Vec<(usize, usize)>) {
    let parts: Vec<_> = input.split('\n').collect();
    if parts.len() < 2 {
        unreachable!()
    }
    let buses: Vec<(usize, usize)> = parts[1]
        .split(',')
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, bus)| (i, bus.parse().unwrap()))
        .collect();

    (parts[0].parse().unwrap(), buses)
}

#[aoc(day13, part1)]
fn part1(notes: &(usize, Vec<(usize, usize)>)) -> usize {
    let (earliest, buses) = notes;
    let (bus, early) = buses
        .iter()
        .map(|(_, bus)| (*bus, *bus - *earliest % *bus))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    bus * early
}

#[aoc(day13, part2)]
fn part2(notes: &(usize, Vec<(usize, usize)>)) -> usize {
    chinese_remainder(
        notes
            .1
            .iter()
            .map(|(off, bus)| (simplify_mod(*bus as isize - *off as isize, *bus), *bus))
            .collect(),
    )
}

// Chinese remainder theorem
// https://www.youtube.com/watch?v=zIFehsBHB8o
fn chinese_remainder(mods: Vec<(usize, usize)>) -> usize {
    let prod: usize = mods.iter().map(|m| m.1).product();
    mods.iter()
        .map(|m| {
            let p = prod / m.1;
            m.0 * p * mod_inv(p, m.1).unwrap()
        })
        .sum::<usize>()
        % prod
}

// https://brilliant.org/wiki/extended-euclidean-algorithm/
// returns gcd, and x, y where xa + yb = gcd(a, b)
fn egcd(a: usize, b: usize) -> (usize, isize, isize) {
    let (mut r0, mut r1) = (a, b); // don't want input to be captured

    // Initilize
    let (mut s0, mut t0, mut s1, mut t1) = (1, 0, 0, 1);
    while r1 != 0 {
        let (q2, r2) = (r0 / r1, r0 % r1);
        let (s2, t2) = (s0 - s1 * (q2 as isize), t0 - t1 * (q2 as isize));
        // Shift
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }
    (r0, s0, t0)
}

fn mod_inv(res: usize, m: usize) -> Option<usize> {
    let (g, x, _) = egcd(res, m);
    if g == 1 {
        Some(simplify_mod(x, m))
    } else {
        None
    }
}

fn simplify_mod(res: isize, m: usize) -> usize {
    ((res % m as isize + m as isize) % m as isize) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    static NOTES: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(NOTES),
            (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)])
        );
    }

    #[test]
    fn running_part1() {
        let notes = parse_input(NOTES);
        assert_eq!(part1(&notes), 295);
    }

    static NOTES2: &str = "939
17,x,13,19";
    static NOTES3: &str = "939
67,7,59,61";
    static NOTES4: &str = "939
67,x,7,59,61";
    static NOTES5: &str = "939
67,7,x,59,61";
    static NOTES6: &str = "939
1789,37,47,1889";

    #[test]
    fn running_part2() {
        let notes = parse_input(NOTES);
        assert_eq!(part2(&notes), 1068781);
        let notes = parse_input(NOTES2);
        assert_eq!(part2(&notes), 3417);
        let notes = parse_input(NOTES3);
        assert_eq!(part2(&notes), 754018);
        let notes = parse_input(NOTES4);
        assert_eq!(part2(&notes), 779210);
        let notes = parse_input(NOTES5);
        assert_eq!(part2(&notes), 1261476);
        let notes = parse_input(NOTES6);
        assert_eq!(part2(&notes), 1202161486);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(egcd(1914, 899), (29, 8, -17));
        assert_eq!(egcd(899, 1914), (29, -17, 8));
        assert_eq!(egcd(102, 38), (2, 3, -8));
        assert_eq!(egcd(352, 168), (8, -10, 21));
        assert_eq!(egcd(3458, 4864), (38, -45, 32));
        assert_eq!(egcd(1432, 123211), (1, -22973, 267));
    }

    #[test]
    fn test_chinese() {
        assert_eq!(chinese_remainder(vec![(3, 5), (1, 7), (6, 8)]), 78);
        assert_eq!(chinese_remainder(vec![(2, 3), (2, 4), (1, 5)]), 26);
    }
}
