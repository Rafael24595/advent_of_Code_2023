use lazy_static::lazy_static;

use std::{fs, collections::BTreeMap, cmp::Ordering};
use crate::_utils::CONSOLE_COLORS;

lazy_static! {
    static ref CARDS_RANK: BTreeMap<char, usize> = BTreeMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
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
    hand_results.sort_by(|a, b| compare_cards(&a.0, &b.0));

    let mut result = 0;

    println!("Sorted hands: ");

    for (i, tuple) in hand_results.iter().enumerate() {
        println!(" - Cards: {} - Rank: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(tuple.0.clone()), CONSOLE_COLORS::CONSOLE_SUCESS.wrap(tuple.1));
        result = result + tuple.1 * (i as i32 + 1);
    }

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn compare_cards(cards1: &String, cards2: &String) -> Ordering {
    let mut hand1 = [0;14];
    let mut hand2 = [0;14];

    let mut aux_order: Option<Ordering> = None;

    for i in 0..cards1.len() {
        let card1 = CARDS_RANK.get(&cards1.chars().nth(i).unwrap()).unwrap();
        let card2 = CARDS_RANK.get(&cards2.chars().nth(i).unwrap()).unwrap();

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

    let mut sum1 = 0;
    let mut sum2 = 0;

    for (pos, _) in hand1.iter().enumerate() {
        sum1 = sum1 + hand1.get(pos).unwrap() * hand1.get(pos).unwrap();
        sum2 = sum2 + hand2.get(pos).unwrap() * hand2.get(pos).unwrap();
    }

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

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 7.2 |");
    println!("----------------");
    println!("\n");

    //TODO: TODO.
    
}