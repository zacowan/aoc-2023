use std::{cmp::Ordering, ops::Mul};

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Card {
    label: char,
    value: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandResult {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    cards: Vec<Card>,
    bid: u128,
    result: HandResult,
}

fn get_card_value(card: char, jokers: bool) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => match jokers {
            true => 0,
            false => 11,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card character"),
    }
}

fn build_card(card: char, jokers: bool) -> Card {
    Card {
        label: card,
        value: get_card_value(card, jokers),
    }
}

fn get_of_a_kinds_for_cards(mut cards: Vec<Card>) -> Vec<u8> {
    let mut of_a_kinds: Vec<u8> = vec![];
    let mut prev_card: Option<Card> = None;
    let mut curr_matches: u8 = 0;
    cards.sort_by(|a, b| a.value.cmp(&b.value));
    cards.iter().for_each(|card| {
        if prev_card.is_none() || prev_card.unwrap() != *card {
            if curr_matches > 1 {
                of_a_kinds.push(curr_matches);
            };
            curr_matches = 1;
            prev_card = Some(*card);
            return;
        };

        curr_matches += 1;
    });

    if curr_matches > 1 {
        of_a_kinds.push(curr_matches);
    };

    of_a_kinds
}

fn get_updated_of_a_kinds_with_jokers(of_a_kinds: Vec<u8>, num_jokers: u8) -> Vec<u8> {
    match num_jokers {
        0 => of_a_kinds,
        5 => vec![num_jokers],
        _ => {
            let mut of_a_kinds = of_a_kinds;
            of_a_kinds.sort_by(|a, b| b.cmp(a));
            match of_a_kinds.len() {
                0 => of_a_kinds.push(num_jokers + 1),
                _ => of_a_kinds[0] += num_jokers,
            };
            of_a_kinds
        }
    }
}

fn get_of_a_kinds_for_cards_with_jokers(mut cards: Vec<Card>) -> Vec<u8> {
    let mut of_a_kinds: Vec<u8> = vec![];
    let mut prev_card: Option<Card> = None;
    let mut curr_matches: u8 = 0;
    let mut num_jokers: u8 = 0;
    cards.sort_by(|a, b| a.value.cmp(&b.value));
    cards.iter().for_each(|card| {
        if card.label == 'J' {
            num_jokers += 1;
            return;
        }

        if prev_card.is_none() || prev_card.unwrap() != *card {
            if curr_matches > 1 {
                of_a_kinds.push(curr_matches);
            };
            curr_matches = 1;
            prev_card = Some(*card);
            return;
        };

        curr_matches += 1;
    });

    if curr_matches > 1 {
        of_a_kinds.push(curr_matches);
    };

    get_updated_of_a_kinds_with_jokers(of_a_kinds, num_jokers)
}

fn get_hand_result_for_of_a_kinds(of_a_kinds: &Vec<u8>) -> HandResult {
    if of_a_kinds.is_empty() {
        return HandResult::HighCard;
    };
    if of_a_kinds.len() == 1 {
        return match of_a_kinds.iter().sum() {
            5 => HandResult::FiveOfKind,
            4 => HandResult::FourOfKind,
            3 => HandResult::ThreeOfKind,
            2 => HandResult::OnePair,
            _ => panic!("Invalid sum of of_a_kinds with len() == 1"),
        };
    };
    if of_a_kinds.len() == 2 {
        return match of_a_kinds.iter().sum() {
            4 => HandResult::TwoPair,
            5 => HandResult::FullHouse,
            _ => panic!("Invalid sum of of_a_kinds with len() == 2"),
        };
    };
    panic!("Could not determine hand result");
}

fn get_hand_result(cards: &[Card], jokers: bool) -> HandResult {
    match jokers {
        true => {
            let of_a_kinds = get_of_a_kinds_for_cards_with_jokers(cards.to_vec());
            get_hand_result_for_of_a_kinds(&of_a_kinds)
        }
        false => {
            let of_a_kinds = get_of_a_kinds_for_cards(cards.to_vec());
            get_hand_result_for_of_a_kinds(&of_a_kinds)
        }
    }
}

fn build_hand(input_line: &str, jokers: bool) -> Hand {
    let input: Vec<&str> = input_line.split_whitespace().collect();
    let cards: Vec<Card> = input
        .first()
        .unwrap()
        .chars()
        .map(|s| build_card(s, jokers))
        .collect();
    let bid: u128 = input.get(1).unwrap().parse().unwrap();
    let result = get_hand_result(&cards, jokers);

    Hand { cards, bid, result }
}

fn get_ordering_for_equal_hands(a: &Hand, b: &Hand) -> Ordering {
    let mut result: Ordering = Ordering::Equal;
    for (i, card_a) in a.cards.iter().enumerate() {
        let card_b = b.cards.get(i).unwrap();
        if card_a.value > card_b.value {
            result = Ordering::Greater;
            break;
        };
        if card_a.value < card_b.value {
            result = Ordering::Less;
            break;
        };
    }
    result
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut hands: Vec<Hand> = input.lines().map(|l| build_hand(l, false)).collect();
    hands.sort_by(|a, b| match a.result.cmp(&b.result) {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => get_ordering_for_equal_hands(a, b),
    });
    let winnings: u128 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid.mul((rank + 1) as u128))
        .sum();
    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut hands: Vec<Hand> = input.lines().map(|l| build_hand(l, true)).collect();
    hands.sort_by(|a, b| match a.result.cmp(&b.result) {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => get_ordering_for_equal_hands(a, b),
    });
    let winnings: u128 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid.mul((rank + 1) as u128))
        .sum();
    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
