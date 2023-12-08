use std::{cmp::Ordering, fmt::Debug, str::FromStr};

use crate::Solutions;
use lib_aoc::prelude::*;

#[derive(Clone)]
pub struct Hand<T> {
    cards: Vec<T>,
    strength: CardStrength,
    bid: usize,
}

impl From<Hand<CardLabel>> for Hand<CardLabelPartB> {
    fn from(value: Hand<CardLabel>) -> Self {
        let cards = value
            .cards
            .into_iter()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()
            .unwrap();
        let strength = CardStrength::try_from(&cards).unwrap();
        Self {
            cards,
            strength,
            bid: value.bid,
        }
    }
}

impl<T> Debug for Hand<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            write!(f, "{:?}", c)?;
        }
        write!(f, " ")?;
        write!(f, "{:?}", self.bid)
    }
}

impl<T> FromStr for Hand<T>
where
    T: TryFrom<char, Error = &'static str> + CardLabler,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(" ").collect();
        let labels = split
            .first()
            .unwrap()
            .chars()
            .map(|c| T::try_from(c))
            .collect::<Result<_, _>>()?;
        let strength = CardStrength::try_from(&labels).unwrap();
        let bid = split.last().unwrap().parse().unwrap();
        Ok(Hand {
            cards: labels,
            strength,
            bid,
        })
    }
}

impl<T> Ord for Hand<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .fold(Ordering::Equal, |acc, (s, o)| match acc {
                        Ordering::Equal => s.cmp(&o),
                        fin => fin,
                    })
            }
            o => o,
        }
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Hand<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<T> Eq for Hand<T> where T: Eq {}

trait CardLabler: PartialOrd + PartialEq + Ord + Eq + Debug + Clone {
    fn is_wild(&self) -> bool;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardLabel {
    Num(usize),
    Jack,
    Queen,
    King,
    Ace,
}

impl CardLabler for CardLabel {
    fn is_wild(&self) -> bool {
        false
    }
}

impl Debug for CardLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardLabel::Num(d) if d == &10usize => write!(f, "T"),
            CardLabel::Num(d) => write!(f, "{}", d),
            CardLabel::Jack => write!(f, "J"),
            CardLabel::Queen => write!(f, "Q"),
            CardLabel::King => write!(f, "K"),
            CardLabel::Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for CardLabel {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(CardLabel::Ace),
            'K' => Ok(CardLabel::King),
            'Q' => Ok(CardLabel::Queen),
            'J' => Ok(CardLabel::Jack),
            'T' => Ok(CardLabel::Num(10)),
            v if v.is_digit(10) => Ok(CardLabel::Num(v.to_digit(10).unwrap() as usize)),
            _ => Err("Unknown Card Label"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardLabelPartB {
    Jocker,
    Num(usize),
    Queen,
    King,
    Ace,
}

impl CardLabler for CardLabelPartB {
    fn is_wild(&self) -> bool {
        match self {
            CardLabelPartB::Jocker => true,
            _ => false,
        }
    }
}

impl Debug for CardLabelPartB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardLabelPartB::Num(d) if d == &10usize => write!(f, "T"),
            CardLabelPartB::Num(d) => write!(f, "{}", d),
            CardLabelPartB::Jocker => write!(f, "J"),
            CardLabelPartB::Queen => write!(f, "Q"),
            CardLabelPartB::King => write!(f, "K"),
            CardLabelPartB::Ace => write!(f, "A"),
        }
    }
}

impl From<CardLabel> for CardLabelPartB {
    fn from(value: CardLabel) -> Self {
        match value {
            CardLabel::Num(d) => CardLabelPartB::Num(d),
            CardLabel::Jack => CardLabelPartB::Jocker,
            CardLabel::Queen => CardLabelPartB::Queen,
            CardLabel::King => CardLabelPartB::King,
            CardLabel::Ace => CardLabelPartB::Ace,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum CardStrength {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

impl<T> TryFrom<&Vec<T>> for CardStrength
where
    T: CardLabler,
{
    type Error = &'static str;

    fn try_from(value: &Vec<T>) -> Result<Self, Self::Error> {
        let mut sorted = value.to_owned();
        sorted.sort();
        Ok(CardStrength::from_vec(sorted))
    }
}

impl CardStrength {
    fn from_vec<T: CardLabler>(sorted: Vec<T>) -> CardStrength {
        let wilds = sorted.iter().filter(|s| s.is_wild()).count();
        let (mut counts, _) =
            sorted
                .into_iter()
                .fold((Vec::new(), None), |(mut acc, last), card| match last {
                    None => {
                        acc.push(1);
                        (acc, Some(card))
                    }
                    Some(c) if c == card => {
                        *acc.last_mut().unwrap() += 1;
                        (acc, Some(card))
                    }
                    _ => {
                        acc.push(1);
                        (acc, Some(card))
                    }
                });
        counts.sort();
        counts.reverse();
        match (counts.len(), wilds, counts.get(0).unwrap()) {
            (1, _, _) => CardStrength::FiveOAK,

            (2, 0, 4) => CardStrength::FourOAK,
            (2, 0, 3) => CardStrength::FullHouse,
            (2, _, _) => CardStrength::FiveOAK,

            (3, 0, 3) => CardStrength::ThreeOAK,
            (3, 0, 2) => CardStrength::TwoPairs,
            (3, 1, 2) => CardStrength::FullHouse,
            (3, _, _) => CardStrength::FourOAK,

            (4, 0, _) => CardStrength::OnePair,
            (4, _, _) => CardStrength::ThreeOAK,

            (5, 0, _) => CardStrength::HighCard,
            (5, _, _) => CardStrength::OnePair,
            _ => unreachable!(),
        }
    }
}

impl Solution<DAY_07> for Solutions {
    type Input<'i> = Vec<Hand<CardLabel>>;
    type Output = usize;

    fn parse(puzzle: &str) -> Vec<Hand<CardLabel>> {
        puzzle
            .lines()
            .map(|l| Hand::from_str(l))
            .collect::<Result<_, _>>()
            .unwrap()
    }

    fn part_one(input: &Vec<Hand<CardLabel>>) -> usize {
        let mut ranked = input.to_owned();
        ranked.sort();
        ranked
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank + 1))
            .sum()
    }

    fn part_two(input: &Vec<Hand<CardLabel>>) -> usize {
        let mut ranked = input
            .to_owned()
            .into_iter()
            .map(|h| Hand::<CardLabelPartB>::from(h))
            .collect::<Vec<_>>();
        ranked.sort();
        ranked
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank + 1))
            .sum()
    }
}

impl Test<DAY_07> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 6440,
            PART_TWO => 5905,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_07);
}
