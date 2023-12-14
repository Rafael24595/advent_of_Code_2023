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
            let table = &evalue(&a.0, &a.1, None, Some(true));
            l + table.last().unwrap().last().unwrap()
        });

    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
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

    let contents = fs::read_to_string("./resources/EXERCISE_XII.txt")
        .expect("Oh! Something happens! Merry Christmas!");
    
    let springs_map = &parse_input(contents);
    let result = springs_map.iter()
        .fold(0, |l, a| {
            let table = &evalue(&a.0, &a.1, Some(5), None);
            l + table.last().unwrap().last().unwrap()
        });

    println!("Result: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn evalue(o_springs_map: &Vec<char>, o_sizes: &Vec<usize>, op_multiplier: Option<usize>, o_show: Option<bool>) -> Vec<Vec<usize>> {
    let mut show = false;
    if o_show.is_some() {
        show = o_show.unwrap();
    }

    let springs_map = &fix_springs_map(o_springs_map, op_multiplier);
    let sizes = &fix_sizes(o_sizes, op_multiplier);

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

    if show {
        print_table(springs_map, sizes, &table);
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

fn fix_springs_map(o_springs_map: &Vec<char>, op_multiplier: Option<usize>) -> Vec<char> {
    let mut multiplier = 1;
    if op_multiplier.is_some() {
        multiplier = op_multiplier.unwrap();
    }
    let mut springs_map: Vec<char> = o_springs_map.iter().cloned()
        .cycle()
        .take(multiplier * o_springs_map.len())
        .enumerate()
        .flat_map(|(i, ch)| 
            if i % o_springs_map.len() == 0 && i != 0 { 
                vec![POINT_STATELESS, ch] 
            } else { 
                vec![ch] 
            })
        .collect();
    springs_map.reverse();
    springs_map.insert(0, POINT_CONTROL);
    return springs_map;
}

fn fix_sizes(o_sizes: &Vec<usize>, op_multiplier: Option<usize>) -> Vec<usize> {
    let mut multiplier = 1;
    if op_multiplier.is_some() {
        multiplier = op_multiplier.unwrap();
    }
    let mut sizes: Vec<usize> = o_sizes.iter().cloned()
        .cycle()
        .take(multiplier * o_sizes.len()).collect();
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