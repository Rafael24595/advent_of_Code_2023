use std::fs;

use crate::_utils::CONSOLE_COLORS;

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 6.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VI.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let race_results = parse_results(contents);

    let mut results = Vec::new();
    for race_result in race_results {
        results.push(evalue(race_result, Some(true)));
        println!();
    }

    let result = results.iter()
        .fold(1, |mut r, &val| {r = r * val; r});

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn parse_results(contents: String) -> Vec<Vec<i64>> {
    let mut lines = contents.split("\n");
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    let mut tuples = Vec::new();

    for i in 0..times.len() {
        let mut tuple = Vec::new();
        tuple.push(*times.get(i).unwrap());
        tuple.push(*distances.get(i).unwrap());
        tuples.push(tuple);
    }

    return tuples;
}

fn parse_line(line: &str) -> Vec<i64> {
    let line_vector: Vec<&str> = line.split(":").collect();
    return line_vector.last().unwrap()
        .trim()
        .split(" ")
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<i64>().unwrap())
        .collect();
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 6.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VI.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let race_result = parse_fixed_results(contents);

    let result = evalue(race_result, None);

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn parse_fixed_results(contents: String) -> Vec<i64> {
    let mut lines = contents.split("\n");
    let time = parse_fixed_line(lines.next().unwrap());
    let distance = parse_fixed_line(lines.next().unwrap());

    let mut tuple = Vec::new();
    tuple.push(time);
    tuple.push(distance);

    return tuple;
}

fn parse_fixed_line(line: &str) -> i64 {
    let line_vector: Vec<&str> = line.split(":").collect();
    return line_vector.last().unwrap()
        .replace(" ", "")
        .trim()
        .parse::<i64>().unwrap();
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn evalue(race_results: Vec<i64>, show: Option<bool>) -> i64 {
    let mut count = 0;
    let time = *race_results.get(0).unwrap();
    let distance = *race_results.get(1).unwrap();
    println!("Time: {} - Distance: {}: ", CONSOLE_COLORS::CONSOLE_RESULT.wrap(time), CONSOLE_COLORS::CONSOLE_POWER.wrap(distance));
    for i in 0..time {
        let speed = i;
        let time_remain = time - i;
        let distance_final = speed * time_remain;
        if distance_final > distance {
            if show.is_some() && show.unwrap() {
                println!(" New record found [Time: {} - Speed: {}]: {} ", 
                    CONSOLE_COLORS::CONSOLE_RESULT.wrap(time_remain),
                    CONSOLE_COLORS::CONSOLE_POWER.wrap(speed),
                    CONSOLE_COLORS::CONSOLE_SUCESS.wrap(distance_final));
            }
            count += 1;
        }
    }
    return count;
}