use std::fs;

use crate::_utils::CONSOLE_COLORS;

const POINT_STATEFULL: char = '#';
const POINT_STATELESS: char = '?';
const POINT_CONTROL: char = '_';
const POINT_VOID: char = '.';

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {
    
    /* Thanks to AndreiaCo && marx38 from reddit! */

    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 12.1 |");
    println!("-----------------");
    println!("\n");


    let contents = fs::read_to_string("./resources/EXERCISE_XII.txt")
        .expect("Oh! Something happens! Merry Christmas!");
    
    let springs_map = &parse_input(contents);
    let result = springs_map.iter()
        .fold(0, |l, a| {
            let table = &evalue(&a.0, &a.1);
            print_table(&a.0, &a.1, table);
            l + table.last().unwrap().last().unwrap()
        });

    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn evalue(o_springs_map: &Vec<char>, o_sizes: &Vec<usize>) -> Vec<Vec<usize>> {
    let springs_map = &fix_springs_map(o_springs_map);
    let sizes = &fix_sizes(o_sizes);

    let mut table: Vec<Vec<usize>> = Vec::new();
    table.push(build_base_row(springs_map));

    for size in sizes {
        let row_base = table.last().unwrap();
        let mut row_count = vec![0; springs_map.len() + 1];
        let mut range = 0;
        for (i, character) in springs_map.iter().cloned().enumerate() {
            if character!= POINT_CONTROL && character != POINT_VOID {
                range += 1;
            } else {
                range = 0;
            }

            if character != POINT_STATEFULL {
                row_count[i + 1] += row_count[i];
            }

            if range >= *size && springs_map[i - size] != POINT_STATEFULL {
                row_count[i + 1] += row_base[i - size];
            }
        }
        table.push(row_count);
    }

    return table;
}

fn build_base_row(springs_map: &Vec<char>) -> Vec<usize> {
    let mut row_base = vec![0; springs_map.len() + 1];
    let min_coverage = 0..springs_map.iter().take_while(|&&c| c != POINT_STATEFULL).count();
    for i in min_coverage {
        if i == 0 {
            row_base[i] = 1;
        }
        row_base[i + 1] = 1;
    }
    return row_base;
}

fn fix_springs_map(o_springs_map: &Vec<char>) -> Vec<char> {
    let mut springs_map = o_springs_map.clone();
    springs_map.reverse();
    springs_map.insert(0, POINT_CONTROL);
    return springs_map;
}

fn fix_sizes(o_sizes: &Vec<usize>) -> Vec<usize> {
    let mut sizes = o_sizes.clone();
    sizes.reverse();
    return sizes;
}

fn print_table(springs_map: &Vec<char>, sizes: &Vec<usize>, table: &Vec<Vec<usize>>) {
    let springs: Vec<String> = springs_map.iter().map(|s| s.to_string()).collect();
    println!("             {} - -", springs.join(" "));
    for i in 0..table.len() {
        let mut size = "Base".to_string();
        let mut spaces = "   ".to_string();
        if i > 0 {
            let mut len = size.len();
            if sizes.len() + 1 > len {
                len = sizes.len() + 1;
            }
            spaces = vec!["  ".to_string(); len - i].join("");
            let mut rev_size = sizes.clone();
            rev_size.reverse();
            let sizes_string: Vec<String> = rev_size[0..i].iter().map(|s| s.to_string()).collect();
            size = sizes_string.join(",");
        }
        let mut row_string: Vec<String> = table[i].iter().map(|e| e.to_string()).collect();
        row_string.reverse();
        let row = row_string.join(" ");
        println!("[{}]{} -> {}", size, spaces, row);
    }
    println!("Combinations: {}\n", table.last().unwrap().last().unwrap());
}

fn parse_input(contents: String) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut springs_map: Vec<(Vec<char>, Vec<usize>)> = Vec::new();
    let line_vector: Vec<&str> = contents.split("\n").collect();

    for line in line_vector {
        let tuple: Vec<&str> = line.split(" ").collect();
        let row: Vec<char> = tuple.first().unwrap().chars().collect();

        let springs: Vec<usize> = tuple.last().unwrap()
            .trim()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap() as usize)
            .collect();
        springs_map.push((row, springs));
    }

    return springs_map;
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 12.1 |");
    println!("-----------------");
    println!("\n");

    //TODO: TODO.
}