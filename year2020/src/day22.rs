/// https://adventofcode.com/2020/day/22
use pest::Parser;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Parser)]
#[grammar = "pest/day22.pest"]
struct InputParser;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut file = InputParser::parse(Rule::file, input).expect("could not parse the file");
    let player1 = file
        .next()
        .unwrap()
        .into_inner()
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let player2 = file
        .next()
        .unwrap()
        .into_inner()
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    (player1, player2)
}

fn play_compat((player_1, player_2): (&mut VecDeque<usize>, &mut VecDeque<usize>)) {
    loop {
        if player_1.is_empty() || player_2.is_empty() {
            break;
        }
        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();
        match p1.cmp(&p2) {
            std::cmp::Ordering::Less => {
                player_2.push_back(p2);
                player_2.push_back(p1);
            }
            std::cmp::Ordering::Greater => {
                player_1.push_back(p1);
                player_1.push_back(p2);
            }
            std::cmp::Ordering::Equal => unreachable!(),
        }
    }
}

enum Winner {
    Player1,
    Player2,
}

fn card_hasher(cards: &VecDeque<usize>) -> u64 {
    let mut s = DefaultHasher::new();
    cards.hash(&mut s);
    s.finish()
}

fn play_recurse((player_1, player_2): (&mut VecDeque<usize>, &mut VecDeque<usize>)) -> Winner {
    let (mut p1_hash, mut p2_hash) = (HashSet::new(), HashSet::new());
    loop {
        let (new_p1_hash, new_p2_hash) = (card_hasher(player_1), card_hasher(player_2));
        if p1_hash.contains(&new_p1_hash) || p2_hash.contains(&new_p2_hash) {
            break Winner::Player1;
        }
        p1_hash.insert(new_p1_hash);
        p2_hash.insert(new_p2_hash);

        if player_1.is_empty() || player_2.is_empty() {
            break if !player_1.is_empty() {
                Winner::Player1
            } else {
                Winner::Player2
            };
        }

        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        if p1 <= player_1.len() && p2 <= player_2.len() {
            let mut rec_player_1: VecDeque<_> = player_1.iter().take(p1).cloned().collect();
            let mut rec_player_2: VecDeque<_> = player_2.iter().take(p2).cloned().collect();
            match play_recurse((&mut rec_player_1, &mut rec_player_2)) {
                Winner::Player1 => {
                    player_1.push_back(p1);
                    player_1.push_back(p2);
                }
                Winner::Player2 => {
                    player_2.push_back(p2);
                    player_2.push_back(p1);
                }
            }
        } else {
            match p1.cmp(&p2) {
                std::cmp::Ordering::Less => {
                    player_2.push_back(p2);
                    player_2.push_back(p1);
                }
                std::cmp::Ordering::Greater => {
                    player_1.push_back(p1);
                    player_1.push_back(p2);
                }
                std::cmp::Ordering::Equal => unreachable!(),
            }
        }
    }
}

fn calculate_score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

#[aoc(day22, part1)]
fn part1(cards: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let (mut player_1, mut player_2) = cards.clone();
    play_compat((&mut player_1, &mut player_2));
    if !player_1.is_empty() {
        calculate_score(&player_1)
    } else {
        calculate_score(&player_2)
    }
}

#[aoc(day22, part2)]
fn part2(cards: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
    let (mut player_1, mut player_2) = cards.clone();
    match play_recurse((&mut player_1, &mut player_2)) {
        Winner::Player1 => calculate_score(&player_1),
        Winner::Player2 => calculate_score(&player_2),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static CARDS: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(CARDS),
            (
                vec![9, 2, 6, 3, 1].into_iter().collect(),
                vec![5, 8, 4, 7, 10].into_iter().collect()
            )
        );
    }

    #[test]
    fn running_part1() {
        let cards = parse_input(CARDS);
        assert_eq!(part1(&cards), 306);
    }

    #[test]
    fn running_part2() {
        let cards = parse_input(CARDS);
        assert_eq!(part2(&cards), 291);
    }
}
