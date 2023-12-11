use std::{fs, collections::HashMap};

use crate::_utils::CONSOLE_COLORS;

const POINT_GROUND: char = '.';

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 10.1 |");
    println!("-----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_XI.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let matrix = &parse_matrix(contents);
    let pointers = &find_pointers(matrix);

    let result = evalue(pointers);

    println!("Number of pairs: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(result.1));
    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result.0));
}

fn evalue(pointers: &Vec<(char, usize, usize)>) -> (i64, usize) {
    let mut result = 0;
    let mut pairs_relation = HashMap::new();
    for pointer_a in pointers {
        for pointer_b in pointers {
            if pointer_a != pointer_b {
                let y_distance = (pointer_a.1 as i64 - pointer_b.1 as i64).abs();
                let x_distance = (pointer_a.2 as i64 - pointer_b.2 as i64).abs();
                if pairs_relation.contains_key(&(pointer_b, pointer_a)) {
                    continue;
                }
                pairs_relation.insert((pointer_a, pointer_b), true);
                result += y_distance + x_distance;
            }
        }   
    }

    return (result, pairs_relation.len());
}

fn parse_matrix(contents: String) -> Vec<Vec<char>> {
    let line_vector: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    let matrix :Vec<Vec<char>> = line_vector.iter()
        .map(|l| l
            .trim()
            .split("")
            .filter(|t| !t.is_empty())
            .map(|s| 
                if s == "#" {
                    count += 1;
                    return count.to_string().chars().last().unwrap(); 
                } else {
                    return s.chars().last().unwrap()
                })
            .collect())
        .collect();
    return fix_spaces(matrix);
}

fn fix_spaces(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut fixed = matrix.clone();

    let mut fix = 0;
    for (y, row) in matrix.iter().enumerate() {
        let sw_empty = row.iter().find(|&&c| c != POINT_GROUND);
        if sw_empty.is_none() {
            fixed.insert(y + fix, row.clone());
            fix += 1;
        }
    }

    fix = 0;
    for (x, _) in matrix.first().unwrap().iter().enumerate() {
        let sw_empty = matrix.iter().find(|r| r[x] != POINT_GROUND);
        if sw_empty.is_none() {
            fixed.iter_mut().for_each(|r| r.insert(x + fix, POINT_GROUND));
            fix += 1;
        }
    }

    return fixed;
}

fn find_pointers(matrix: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let mut pointers: Vec<(char, usize, usize)> = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &character) in row.iter().enumerate() {
            if character != POINT_GROUND {
                pointers.push((character, y, x));
            }
        }
    }

    return pointers;
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for character in row {
            print!("{}", character);
        }
        println!("");
    }
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 10.2 |");
    println!("-----------------");
    println!("\n");

    //TODO: TODO.
}