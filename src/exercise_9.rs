use std::fs;
use crate::_utils::CONSOLE_COLORS;

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    println!("\n");
    println!("----------------");
    println!("| EXERCISE 9.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_IX.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = &parse_lines(contents);

    let mut result = 0;

    for line in lines {
        let str_vector: Vec<String> = line.iter().map(|r| r.to_string()).collect();
        println!("Row: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(str_vector.join(" ")));
        let evaluation = evalue(line, None);
        println!(" - New value: {}\n", CONSOLE_COLORS::CONSOLE_SUCESS.wrap(evaluation));
        result = result + evaluation;
    }

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 9.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_IX.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = parse_lines(contents);
    let lines_rev: Vec<Vec<i64>> = lines.iter().cloned().map(|mut l| {l.reverse(); l}).collect();

    let mut result = 0;

    for line in lines_rev {
        let str_vector: Vec<String> = line.iter().map(|r| r.to_string()).collect();
        println!("Row: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(str_vector.join(" ")));
        let evaluation = evalue(&line, Some(-1));
        println!(" - New value: {}\n", CONSOLE_COLORS::CONSOLE_SUCESS.wrap(evaluation));
        result = result + evaluation;
    }

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn evalue(line: &Vec<i64>, fix: Option<i64>) -> i64 {
    let mut v_fix = 1;
    if fix.is_some() {
        v_fix = fix.unwrap()
    }
    let mut run = true;

    let mut differences: Vec<Vec<i64>> = Vec::new();
    differences.push(line.to_vec());

    while run {
        let mut difference: Vec<i64> = Vec::new();

        let e = differences.last().unwrap();
        for i in 0..e.len() - 1 {
            let r = e.get(i + 1).unwrap() - e.get(i).unwrap();
            difference.push(r);
        }

        differences.push(difference.clone());

        let zeroes: usize = difference.iter().filter(|d| **d == 0).count();
        if zeroes == difference.len() {
            run = false;
        }
    }

    return differences.iter().fold(0, |x, y| x + (y.last().unwrap() * v_fix)) * v_fix;
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn parse_lines(contents: String) -> Vec<Vec<i64>> {
    let line_vector: Vec<&str> = contents.split("\n").collect();
    return line_vector.iter()
        .map(|l| l
            .trim()
            .split(" ")
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<i64>().unwrap())
            .collect())
        .collect();
}