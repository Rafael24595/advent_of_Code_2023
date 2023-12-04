use std::fs;
use regex::Regex;

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 1.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_I.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = contents.split("\n");

    let mut result = 0;

    for line in lines {
        let r = Regex::new(r"\d | ").unwrap();
        let numbers = r.find_iter(line);

        let y: Vec<_> = numbers.collect();

        let s_first = y.first();
        let s_fast = y.last();
        
        if s_first.is_some() {
            let first = s_first.unwrap().as_str();
            let last = s_fast.unwrap().as_str();
            let s_combination = first.to_owned() + last;
            let combination = s_combination.parse::<i32>().unwrap();
            result = result + combination;
            println!("First: {} - Last: {} - Combination: {}", first, last, combination);
        }
    }

    println!("Result: {}", result);

}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 1.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_I.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = contents.split("\n");

    let mut result = 0;

    for line in lines {
        result = result + calculate(line);
    }

    println!("Result: {}", result);
}

fn calculate(line: &str) -> i32 {
    let vector = evalue(line);
    let o_first = vector.first();
    let o_last = vector.last();
    if o_first.is_some() {
        let first = o_first.unwrap();
        let last = o_last.unwrap();
        let s_combination = first.to_owned() + last;
        println!("- Line: {}", line);
        println!("First: {} - Last: {} - Combination: {}", first, last, s_combination);
        print!("\n");
        return s_combination.parse::<i32>().unwrap();
    }
    
    return 0;
}

fn evalue(line: &str) -> Vec<String> {
    let mut numbers = Vec::new();
    let mut buffer = Vec::new();
    for char in line.chars() {
        if char.is_ascii_digit() {
            numbers.push(char.to_string());
        }
        if char.is_alphabetic() {
            buffer.push( char.to_string().clone());
        }
        let mirdarust = buffer.join("");
        let sw_number = is_number(mirdarust);
        if sw_number.is_some() {
            numbers.push(sw_number.unwrap().clone());
            buffer = Vec::new();
            buffer.push(char.to_string().clone());
        }
    }
    return numbers;
}

fn is_number(buffer: String) -> Option<String> {
    let string_digits = ["ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE"];
    let upper = buffer.to_uppercase();

    for (i, key) in string_digits.into_iter().enumerate() {
        if upper.contains(key) {
            return Some((i + 1).to_string());
        }
    }

    return None;
}