use std::{cmp::Ordering, fs, path::Path};

use anyhow::Error;
use itertools::Itertools;
use nom::IResult;

#[derive(Debug, PartialEq, Hash, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn value(&self) -> u8 {
        use Card::*;
        match self {
            Ace => 12,
            King => 11,
            Queen => 10,
            Jack => 9,
            Ten => 8,
            Nine => 7,
            Eight => 6,
            Seven => 5,
            Six => 4,
            Five => 3,
            Four => 2,
            Three => 1,
            Two => 0,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    points: i32,
}

impl Hand {
    fn is_five(&self) -> bool {
        self.cards.iter().all_equal()
    }

    fn is_four(&self) -> bool {
        let counts = self.cards.iter().counts();
        counts.iter().any(|(_, &num)| num == 4)
    }

    fn is_full_house(&self) -> bool {
        let counts = self.cards.iter().counts();
        counts.iter().any(|(_, &num)| num == 3) && counts.len() == 2
    }

    fn is_three(&self) -> bool {
        let counts = self.cards.iter().counts();
        counts.iter().any(|(_, &num)| num == 3) && counts.len() == 3
    }

    fn is_two_pair(&self) -> bool {
        let counts = self.cards.iter().counts();
        let nums = counts.iter().map(|(_, &num)| num).sorted().collect_vec();
        nums == [1, 2, 2]
    }

    fn is_one_pair(&self) -> bool {
        let counts = self.cards.iter().counts();
        let nums = counts.iter().map(|(_, &num)| num).sorted().collect_vec();
        nums == [1, 1, 1, 2]
    }

    fn is_high_card(&self) -> bool {
        let counts = self.cards.iter().counts();
        counts.len() == 5
    }

    fn hand_value(&self) -> u8 {
        if self.is_five() {
            7
        } else if self.is_four() {
            6
        } else if self.is_full_house() {
            5
        } else if self.is_three() {
            4
        } else if self.is_two_pair() {
            3
        } else if self.is_one_pair() {
            2
        } else if self.is_high_card() {
            1
        } else {
            unreachable!()
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().sorted().collect_vec() == other.cards.iter().sorted().collect_vec()
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let val1 = self.hand_value();
        let val2 = other.hand_value();
        if val1 == val2 {
            // go through each card
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(c1, c2)| {
                    let comparison = c1.cmp(c2);
                    if comparison == Ordering::Equal {
                        None
                    } else {
                        Some(comparison)
                    }
                })
                .unwrap_or(Ordering::Equal)
        } else {
            val1.cmp(&val2)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_card(i: &str) -> IResult<&str, Card> {
    use Card::*;

    let (i, c) = nom::character::complete::one_of("AKQJT98765432")(i)?;
    let card = match c {
        'A' => Ace,
        'K' => King,
        'Q' => Queen,
        'J' => Jack,
        'T' => Ten,
        '9' => Nine,
        '8' => Eight,
        '7' => Seven,
        '6' => Six,
        '5' => Five,
        '4' => Four,
        '3' => Three,
        '2' => Two,
        _ => unreachable!(),
    };
    Ok((i, card))
}

fn parse_hand(i: &str) -> IResult<&str, Hand> {
    use nom::character::complete::*;
    use nom::multi::*;

    let (i, cards) = count(parse_card, 5)(i)?;
    let (i, _) = space1(i)?;
    let (i, points) = i32(i)?;
    let (i, _) = multispace1(i)?;

    let hand = Hand { cards, points };

    Ok((i, hand))
}

fn parse_input(i: &str) -> IResult<&str, Vec<Hand>> {
    nom::multi::many1(parse_hand)(i)
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d7p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_input(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let mut hands = data.1;

    hands.sort();

    let score = hands
        .iter()
        .enumerate()
        .map(|(i, h)| ((i as i32 + 1) * h.points) as u64)
        .sum::<u64>();

    println!("{score}");

    Ok(())
}
