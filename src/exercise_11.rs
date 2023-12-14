use std::{fs, collections::HashMap};

use crate::_utils::CONSOLE_COLORS;

const POINT_SPACE: char = '.';
const POINT_LARGE_SPACE: char = '$';

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 11.1 |");
    println!("-----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_XI.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let expansion_factor = 1;
    let (matrix, spaces_map) = &parse_matrix(contents);
    let pointers = &find_pointers(matrix);

    let result = evalue(pointers, spaces_map, expansion_factor);

    println!("Number of pairs: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(result.1));
    println!("Expansion factor: {}", CONSOLE_COLORS::CONSOLE_SUCESS.wrap(expansion_factor));
    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result.0));
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 11.1 |");
    println!("-----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_XI.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let expansion_factor = 999_999;
    let (matrix, spaces_map) = &parse_matrix(contents);
    let pointers = &find_pointers(matrix);

    let result = evalue(pointers, spaces_map, expansion_factor);

    println!("Number of pairs: {}", CONSOLE_COLORS::CONSOLE_POWER.wrap(result.1));
    println!("Expansion factor: {}", CONSOLE_COLORS::CONSOLE_SUCESS.wrap(expansion_factor));
    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result.0));
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn evalue(pointers: &Vec<(char, usize, usize)>, spaces_map: &(Vec<usize>, Vec<usize>), spaces_fix: i64) -> (i64, usize) {
    let mut result = 0;
    let mut pairs_relation = HashMap::new();
    for pointer_a in pointers {
        for pointer_b in pointers {
            if pointer_a != pointer_b {
                let fix_count = find_spaces(pointer_a, pointer_b, spaces_map) as i64;
                let y_distance = (pointer_a.1 as i64 - pointer_b.1 as i64).abs();
                let x_distance = (pointer_a.2 as i64 - pointer_b.2 as i64).abs();
                if pairs_relation.contains_key(&(pointer_b, pointer_a)) {
                    continue;
                }
                pairs_relation.insert((pointer_a, pointer_b), true);
                result += y_distance + x_distance + (fix_count * spaces_fix);
            }
        }   
    }

    return (result, pairs_relation.len());
}

fn find_pointers(matrix: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let mut pointers: Vec<(char, usize, usize)> = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &character) in row.iter().enumerate() {
            if character != POINT_SPACE && character != POINT_LARGE_SPACE {
                pointers.push((character, y, x));
            }
        }
    }

    return pointers;
}

fn find_spaces(pointer_a: &(char, usize, usize), pointer_b: &(char, usize, usize), spaces_map: &(Vec<usize>, Vec<usize>)) -> usize {
    let x_spaces = find_space(pointer_a.2, pointer_b.2, &spaces_map.1);
    let y_spaces = find_space(pointer_a.1, pointer_b.1, &spaces_map.0);
    return y_spaces + x_spaces;
}


fn find_space(coord_a: usize, coord_b: usize, spaces_map: &Vec<usize>) -> usize {
    let mut higher = 0;
    let mut lower = 0;
    if coord_a > coord_b {
        higher = coord_a;
        lower = coord_b;
    }
    if coord_a < coord_b {
        higher = coord_b;
        lower = coord_a;
    }
    return spaces_map.iter().filter(|&&p| lower < p && p < higher).count();
}

fn parse_matrix(contents: String) -> (Vec<Vec<char>>, (Vec<usize>, Vec<usize>)) {
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

fn fix_spaces(matrix: Vec<Vec<char>>) -> (Vec<Vec<char>>, (Vec<usize>, Vec<usize>)) {
    let mut fixed = matrix.clone();
    let mut y_large = Vec::new();
    let mut x_large = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        let sw_empty = row.iter().find(|&&c| c != POINT_SPACE);
        if sw_empty.is_none() {
            let mut large_row = row.clone();
            large_row.fill(POINT_LARGE_SPACE);
            fixed[y] = large_row;
            y_large.push(y);
        }
    }

    for (x, _) in matrix.first().unwrap().iter().enumerate() {
        let sw_empty = matrix.iter().find(|r| r[x] != POINT_SPACE);
        if sw_empty.is_none() {
            fixed.iter_mut().for_each(|r| r[x] = POINT_LARGE_SPACE);
            x_large.push(x);
        }
    }

    return (fixed, (y_large, x_large));
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for character in row {
            print!("{}", character);
        }
        println!("");
    }
}