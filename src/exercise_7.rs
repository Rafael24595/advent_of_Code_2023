use lazy_static::lazy_static;

use std::{fs, collections::BTreeMap, cmp::Ordering};
use crate::_utils::CONSOLE_COLORS;

lazy_static! {
    static ref CARDS_RANK: BTreeMap<char, usize> = BTreeMap::from([
        ('A', 13), ('K', 12), ('Q', 11),
        ('J', 10), ('T', 9), ('9', 8),
        ('8', 7), ('7', 6), ('6', 5),
        ('5', 4), ('4', 3), ('3', 2),
        ('2', 1),
    ]);
}

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    println!("\n");
    println!("----------------");
    println!("| EXERCISE 7.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VII.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let mut hand_results = parse_results(contents);
    hand_results.sort_by(|a, b| compare_cards(CARDS_RANK.to_owned(), &a.0, &b.0, false));

    let mut result = 0;

    println!("Sorted hands: ");

    for (i, tuple) in hand_results.iter().enumerate() {
        println!(" - Cards: {} - Rank: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(tuple.0.clone()), CONSOLE_COLORS::CONSOLE_SUCESS.wrap(tuple.1));
        result = result + tuple.1 * (i as i32 + 1);
    }

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

lazy_static! {
    static ref CARDS_RANK_JOKER: BTreeMap<char, usize> = BTreeMap::from([
        ('A', 12), ('K', 11), ('Q', 10),
        ('T', 9), ('9', 8), ('8', 7),
        ('7', 6), ('6', 5), ('5', 4),
        ('4', 3), ('3', 2), ('2', 1),
        ('J', 0),
    ]);
}

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 7.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VII.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let mut hand_results = parse_results(contents);
    hand_results.sort_by(|a, b| compare_cards(CARDS_RANK_JOKER.to_owned(), &a.0, &b.0, true));

    let mut result = 0;

    println!("Sorted hands: ");

    for (i, tuple) in hand_results.iter().enumerate() {
        println!("{}. {}", i, CONSOLE_COLORS::CONSOLE_POWER.wrap(tuple.0.clone()));
        result = result + tuple.1 * (i as i32 + 1);
    }

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
    
}

fn manage_joker(mut hand: [i32; 14]) -> [i32; 14] {
    let mut position: (i32, usize) = (0, 0);
    for i in 1..hand.len() {
        let value = *hand.get(i).unwrap();
        if value > position.0 {
            position = (value, i);
        }
    }

    let increment = hand[0];
    hand[position.1] = hand[position.1] + increment;
    hand[0] = hand[0] - increment;

    return hand;
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn compare_cards(cards_map: BTreeMap<char, usize>, cards1: &String, cards2: &String, has_joker: bool) -> Ordering {
    let mut hand1 = [0;14];
    let mut hand2 = [0;14];

    let mut aux_order: Option<Ordering> = None;

    for i in 0..cards1.len() {
        let card1 = cards_map.get(&cards1.chars().nth(i).unwrap()).unwrap();
        let card2 = cards_map.get(&cards2.chars().nth(i).unwrap()).unwrap();

        if aux_order.is_none() {
            if card1 > card2 {
                aux_order = Some(Ordering::Greater);
            }
            if card1 < card2 {
                aux_order = Some(Ordering::Less);
            }
        }

        hand1[*card1] = hand1[*card1] + 1;
        hand2[*card2] = hand2[*card2] + 1;
    }

    if has_joker {
        hand1 = manage_joker(hand1);
        hand2 = manage_joker(hand2);
    }

    let sum1 = hand1.iter().fold(0, |mut r, &val| {r = r + val * val; r});
    let sum2 = hand2.iter().fold(0, |mut r, &val| {r = r + val * val; r});

    if sum1 > sum2 {
        return Ordering::Greater;
    }
    if sum1 < sum2 {
        return Ordering::Less;
    }

    if aux_order.is_none() {
        return Ordering::Equal;
    }

    return aux_order.unwrap();
}

fn parse_results(contents: String) -> Vec<(String, i32)> {
    let mut tuples = Vec::new();
    for line in contents.split("\n") {
        tuples.push(parse_line(line));
    }

    return tuples;
}

fn parse_line(line: &str) -> (String, i32) {
    let values: Vec<&str> = line.trim().split(" ").collect();
    return (
        values.first().unwrap().to_string(), 
        values.last().unwrap().parse::<i32>().unwrap()
    );
}