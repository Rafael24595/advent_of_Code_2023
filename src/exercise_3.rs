use std::{fs, str::Split};
use regex::Regex;

const CONSOLE_RESET: &str = "\x1b[0m";
const CONSOLE_SUCESS: &str = "\x1b[32m";
const CONSOLE_POWER: &str = "\x1b[34m";
const CONSOLE_FAIL: &str = "\x1b[31m";
const CONSOLE_RESULT: &str = "\x1b[33m";

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 3.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_III.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = contents.split("\n");

    let mut result = 0;

    for (i, line) in lines.clone().into_iter().enumerate() {
        let r = Regex::new(r"\d+").unwrap();
        let numbers = r.find_iter(line);
        for number in numbers {
            let start = number.start();
            let end = number.end();
            let value = number.as_str();

            println!("Identifier: {}", value);

            let sw_match = match_area(lines.clone(), i, start, end);
            if sw_match {
                result = result + value.parse::<i32>().unwrap();
            }

            println!("\n");
        }
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);

}

fn match_area(lines: Split<&str>, line: usize, o_start: usize, o_end: usize) -> bool {
    let vector: Vec<&str> = lines.collect();
    let matrix_length = vector.len();
    let line_length = vector.get(0).unwrap().len();

    let mut matrix_start = line;
    if line > 0 {
        matrix_start = line - 1;
    }

    let mut matrix_end = line;
    if line < matrix_length -1 {
        matrix_end = line + 1;
    }

    let mut start = o_start;
    if start > 0 {
        start = o_start - 1;
    }

    let mut end = o_end;
    if end < line_length -1 {
        end = o_end + 1;
    }

    for y in matrix_start..matrix_end + 1 {
        let l = vector.get(y).cloned().unwrap();
        let f = &l.to_string()[start..end];
        let r = Regex::new(r"[-!#$@%^&*()_+|~=`{}\[\]:;'<>?,\/]").unwrap();
        let mtch = r.find(f);
        if mtch.is_some() {
            let symbol = mtch.unwrap().as_str();
            println!(" - Symbol found: {}{}{}.", CONSOLE_SUCESS, symbol, CONSOLE_RESET);
            return true;
        }
    }

    println!(" - Symbol {}not found{}.", CONSOLE_FAIL, CONSOLE_RESET);

    return false;
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 3.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_III.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = contents.split("\n");

    let mut result = 0;

    for (i, line) in lines.clone().into_iter().enumerate() {
        let r = Regex::new(r"\*").unwrap();
        let gears = r.find_iter(line);
        for gear in gears {
            let postion = gear.start();
            
            println!("Gear found in line {}{}{}:\n", CONSOLE_POWER, i, CONSOLE_RESET);

            let power = match_gear(lines.clone(), i, postion);
            result = result + power;
        }
        println!("")
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);
}

fn match_gear(lines: Split<&str>, line_position: usize, position: usize) -> i32 {
    let conections = gear_connections(lines, line_position, position);

    print!(" - Gear match: ");

    if conections.len() < 2 {
        print!("{}NOT FOUND{}.\n\n", CONSOLE_FAIL, CONSOLE_RESET);
        return 0;
    }

    let mut power = 1;
    let mut iter = conections.iter();
    let mut element = iter.next();

    while element.is_some() {
        print!("{}{}{}", CONSOLE_SUCESS, element.unwrap(), CONSOLE_RESET);
        power = power * element.unwrap();
        element = iter.next();
        if element.is_some() {
            print!(" * ");
        }
    }

    print!(" = {}{}{}.\n\n", CONSOLE_POWER, power, CONSOLE_RESET);

    return power;
}

fn gear_connections(lines: Split<&str>, line_position: usize, position: usize) -> Vec<i32> {
    let vector: Vec<&str> = lines.collect();
    let matrix_length = vector.len();

    let mut matrix_start = line_position;
    if line_position > 0 {
        matrix_start = line_position - 1;
    }

    let mut matrix_end = line_position;
    if line_position < matrix_length -1 {
        matrix_end = line_position + 1;
    }

    let mut conections = Vec::new();

    for y in matrix_start..matrix_end + 1 {
        let line = vector.get(y).cloned().unwrap();
        let regex = Regex::new(r"\d+").unwrap();
        let numbers = regex.find_iter(line);
        for number in numbers {
            let mut start = number.start();
            let end = number.end();
            let value = number.as_str();

            if start > 0 {
                start = start - 1;
            }

            if position >= start && position <= end {
                conections.push(value.parse::<i32>().unwrap());
            }
        }
    }

    return conections;
}