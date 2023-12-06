use std::{fs, time::SystemTime};

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

    let race_result = &parse_fixed_results(contents);

    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros();
    let result = evalue(race_result.clone(), None);
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros(); 

    let start_o = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros();
    let left_edge = find_edge_l(race_result.clone());
    let right_edge = find_edge_r(race_result.clone());
    let end_o = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros(); 

    let result_optimized = right_edge -left_edge;
    
    println!("Result brute-force: {} - Time: {} μs", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result), CONSOLE_COLORS::CONSOLE_FAIL.wrap(end - start));
    println!("Result range-optimized: {} - Time: {} μs", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result_optimized), CONSOLE_COLORS::CONSOLE_FAIL.wrap(end_o - start_o));
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

fn find_edge_r(race_result: Vec<i64>) -> i64 {
    let time = *race_result.get(0).unwrap();
    let distance = *race_result.get(1).unwrap();
    for i in (0..time).rev() {
        let speed = i;
        let time_remain = time - i;
        let distance_final = speed * time_remain;
        if distance_final > distance {
            return i;
        }
    }
    return 0;
}

fn find_edge_l(race_result: Vec<i64>) -> i64 {
    let time = *race_result.get(0).unwrap();
    let distance = *race_result.get(1).unwrap();
    for i in 0..time {
        let speed = i;
        let time_remain = time - i;
        let distance_final = speed * time_remain;
        if distance_final > distance {
            return i - 1;
        }
    }
    return 0;
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
    if show.is_some() && show.unwrap() {
        println!("Time: {} - Distance: {}: ", CONSOLE_COLORS::CONSOLE_RESULT.wrap(time), CONSOLE_COLORS::CONSOLE_POWER.wrap(distance));
    }
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