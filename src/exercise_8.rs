use std::time::SystemTime;
use std::{fs, collections::BTreeMap};
use crate::_utils::CONSOLE_COLORS;
use crate::_utils::lcm_vector;

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    println!("\n");
    println!("----------------");
    println!("| EXERCISE 8.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VIII.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let mut lines = contents.split("\r\n\r\n");

    let steps = &parse_steps(lines.next().unwrap());
    let tree = &parse_tree(lines.next().unwrap());

    let result = find_path("AAA", "ZZZ", steps, tree, Some(true));

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
    println!("| EXERCISE 8.2 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_VIII.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let mut lines = contents.split("\r\n\r\n");

    let steps = parse_steps(lines.next().unwrap());
    let tree = &parse_tree(lines.next().unwrap());
    let start_points: &Vec<&str> = &tree.keys().cloned().filter(|k| k.ends_with("Z")).collect();

    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros();
    let results: Vec<u64> = start_points.iter()
        .map(|p| find_path(p, "Z", &steps, tree, None) as u64)
        .collect();
    let result = lcm_vector(results);
    let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_micros(); 
    
    println!("Result: {} - Time: {} μs", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result), CONSOLE_COLORS::CONSOLE_FAIL.wrap(end - start));    
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn find_path(start: &str, ends: &str, steps: &Vec<String>, tree: &BTreeMap<&str, (String, String)>, show: Option<bool>) -> i32 {
    let mut node = tree.get(start).unwrap();

    let mut count = 0;

    if show.is_some() && show.unwrap() {
        println!("Following path: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(start));
    }

    while count < usize::MAX {
        let index = count - ((count / steps.len()) * steps.len());
        let step = steps.get(index).unwrap();
        let mut key = "";
        if step == "L" {
            key = node.0.as_str();
        }
        if step == "R" {
            key = node.1.as_str();
        }

        if show.is_some() && show.unwrap() {
            println!(" - Direction: {}, Path: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(step), CONSOLE_COLORS::CONSOLE_RESULT.wrap(key));
        }

        node = tree.get(key).unwrap();

        if key.ends_with(ends) {
            if show.is_some() && show.unwrap() {
                println!(" Exit: {}", CONSOLE_COLORS::CONSOLE_SUCESS.wrap("FOUND"));
            }
            return count as i32 + 1;
        }
        count = count + 1;
    }

    return -1;
}

fn parse_steps(line: &str) -> Vec<String> {
    return line
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .map(|i| i.to_string())
        .collect();
}

fn parse_tree(block: &str) -> BTreeMap<&str, (String, String)> {
    let mut tree = BTreeMap::new();

    for line in block.split("\n") {
        let mut splitted = line.split(" = ");
        let code = splitted.next().unwrap().trim();
        let clean = splitted.next().unwrap()
            .replace("(", "")
            .replace(")", "");
        let tuple: Vec<&str> = clean
            .split(",")
            .map(|k| k.trim())
            .collect();
        let left = tuple.get(0).unwrap().clone();
        let right = tuple.get(1).unwrap().clone();
        tree.insert(code, (left.to_string(), right.to_string()));
    }

    return tree;
}