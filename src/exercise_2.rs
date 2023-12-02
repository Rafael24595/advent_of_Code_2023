use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

const CONSOLE_RESET: &str = "\x1b[0m";
const CONSOLE_RESULT: &str = "\x1b[33m";

lazy_static! {
    static ref CONSOLE_COLORS: HashMap<&'static str, &'static str> = HashMap::from([
        ("red", "\x1b[31m"),
        ("green", "\x1b[32m"),
        ("blue", "\x1b[34m"),
    ]);
}

lazy_static! {
    static ref MAX_COLORS: HashMap<&'static str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
}

pub (crate) fn main() {
    exercise_1();
    exercise_2();
}

fn exercise_1() {
    println!("EXERCISE 2.1");

    let contents = fs::read_to_string("./resources/EXERCISE_II.txt")
        .expect("Oh! Something happens! Merry Christmas!");
    let lines = contents.split("\n");

    let mut result = 0;

    for line in lines {
        let id = evalue_1(line);
        let mut id_result = id.to_string();
        if id == 0 {
            id_result = "NONE".to_owned();
        } 
        println!("Identifier: {}{}{}", CONSOLE_RESULT, id_result, CONSOLE_RESET);
        result = result + id;
        println!("\n");
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);
}

fn evalue_1(line: &str) -> i32 {
    let mut sections = line.splitn(2, ":");
    let game_id = sections.next().unwrap();
    let id = game_id.split(" ").nth(1).unwrap();
    
    println!("Line: {}", line);
    
    let game = game_1(sections.next().unwrap());

    if game {
        return id.parse::<i32>().unwrap();
    }

    return 0;
}

fn game_1(game: &str) -> bool {    
    let mut map = HashMap::new();
    for rounds in game.split(";") {
        let hands = rounds.split(", ");
        for hand in hands {
            let mut tuple = hand.trim().split(" ");
            let value = tuple.next().unwrap();
            let key = tuple.next().unwrap();
            let mut position = map.get(key);
            
            let max = MAX_COLORS.get(key);
            if max.is_none() {
                print!("Key is not defined");
            }
            
            let val = value.parse::<i32>().unwrap();
            if val > *max.unwrap() {
                let color = CONSOLE_COLORS.get(key).unwrap();
                println!(" - Failed at: {}{}{}, with: {}{}{}", color, key, CONSOLE_RESET, color, val, CONSOLE_RESET);
                return false;
            }

            if position.is_none() {
                position = Some(&0);
            }
            
            let total = position.unwrap() + val;
            map.insert(key, total);
        }
    }

    return true;
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_2() {
    println!("EXERCISE 2.2");

    let contents = fs::read_to_string("./resources/EXERCISE_II.txt")
        .expect("Oh! Something happens! Merry Christmas!");
    let lines = contents.split("\n");

    let mut result = 0;

    for line in lines {
        let id = evalue_2(line);
        result = result + id;
        println!("\n");
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);
}

fn evalue_2(line: &str) -> i32 {
    let mut sections = line.splitn(2, ":");

    println!("Line: {}", line);
    
    let game = game_2(sections.nth(1).unwrap());
    
    println!(" Power: {}{}{}", CONSOLE_RESULT, game, CONSOLE_RESET);

    return game;
}

fn game_2(game: &str) -> i32 {    
    let mut map = HashMap::new();

    for rounds in game.split(";") {
        let hands = rounds.split(", ");
        for hand in hands {
            let mut tuple = hand.trim().split(" ");
            let s_value = tuple.next().unwrap();
            let key = tuple.next().unwrap();
            let mut position = map.get(key);
            
            let val = s_value.parse::<i32>().unwrap();
            if position.is_none() {
                position = Some(&0);
            }

            if val > *position.unwrap() {
                map.insert(key, val);
            }
        }
    }

    let mut power = 1;
    for key in map.keys() {
        let value = map.get(key).unwrap();
        let color = CONSOLE_COLORS.get(key).unwrap();
        println!(" - Color: {}{}{} - Minimum: {}{}{}", color, key, CONSOLE_RESET, color, value, CONSOLE_RESET);
        power = power * value;
    }

    return power;
}