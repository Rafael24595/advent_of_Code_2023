use std::{fs, collections::BTreeMap};
use crate::_utils::CONSOLE_COLORS;

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

    let result = find_path(steps, tree);

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
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

fn find_path(steps: &Vec<String>, tree: &BTreeMap<&str, (String, String)>) -> i32 {
    let start = "AAA";
    let end = "ZZZ";

    let mut node = tree.get(start).unwrap();

    let mut count = 0;

    println!("Following path: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(start));

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

        println!(" - Direction: {}, Path: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(step), CONSOLE_COLORS::CONSOLE_RESULT.wrap(key));
        node = tree.get(key).unwrap();

        if key == end {
            println!(" Exit: {}", CONSOLE_COLORS::CONSOLE_SUCESS.wrap("FOUND"));
            return count as i32 + 1;
        }
        count = count + 1;
    }

    return -1;
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

    //TODO: TODO.
    
}