use std::fs;

use crate::_utils::CONSOLE_COLORS;

const POINT_ROCK: char = '#';
const POINT_ASH: char = '.';

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("-----------------");
    println!("| EXERCISE 12.1 |");
    println!("-----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_XIII.txt")
        .expect("Oh! Something happens! Merry Christmas!");
    let mut count = 0;
    let springs_map = &parse_input(contents);
    let result = springs_map.iter()
        .fold(0, |l, b| {
            print!("Position: {} => ", CONSOLE_COLORS::CONSOLE_FAIL.wrap(count));
            count += 1;
            let lines = &evalue(&b, None);
            l + lines
        });

    println!("\nResult: {}\n", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn evalue(block: &Vec<Vec<i32>>, o_ocurrence: Option<i32>) -> usize {
    let mut ocurrence = 1;
    if o_ocurrence.is_some() {
        ocurrence = o_ocurrence.unwrap();
    }

    let columns = evalue_columns(block, ocurrence);
    if columns.is_some() {
        return columns.unwrap();
    }

    let rows = evalue_rows(block, ocurrence);
    if rows.is_some() {
        return rows.unwrap();
    }

    return evalue(block, Some(ocurrence + 1));
}

fn evalue_columns(block: &Vec<Vec<i32>>, ocurrence: i32) -> Option<usize> {
    let trim_points = range_columns(block, ocurrence);
    if trim_points.is_some() {
        let match_points = match_vertical(block, &trim_points.unwrap());
        if match_points.is_some() {
            if match_points.unwrap().0 == 0 && match_points.unwrap().0 == match_points.unwrap().1 {
                return Some(0);
            }

            print!("Columns: [{}, {}] - Ocurrence: {}\n", 
                CONSOLE_COLORS::CONSOLE_SUCESS.wrap(match_points.unwrap().0 + 1),
                CONSOLE_COLORS::CONSOLE_SUCESS.wrap(match_points.unwrap().1 + 1),
                CONSOLE_COLORS::CONSOLE_POWER.wrap(ocurrence));

            return Some(match_points.unwrap().0 + 1);
        }
    }

    return None;
}

fn evalue_rows(block: &Vec<Vec<i32>>, ocurrence: i32) -> Option<usize> {
    let trim_points = range_rows(block, ocurrence);
    if trim_points.is_some() {
        let match_points = match_horizontal(block, &trim_points.unwrap());
        if match_points.is_some() {
            if match_points.unwrap().0 == 0 && match_points.unwrap().0 == match_points.unwrap().1 {
                return Some(0);
            }
            
            print!("Rows: [{}, {}] - Ocurrence: {}\n", 
                CONSOLE_COLORS::CONSOLE_SUCESS.wrap(match_points.unwrap().0 + 1),
                CONSOLE_COLORS::CONSOLE_SUCESS.wrap(match_points.unwrap().1 + 1),
                CONSOLE_COLORS::CONSOLE_POWER.wrap(ocurrence));

            return Some(100 * (match_points.unwrap().0 + 1));
        }
    }

    return None;
}

fn range_columns(block: &Vec<Vec<i32>>, ocurrence: i32) -> Option<(usize, usize)> {
    let column_length = block.first().unwrap().len(); 
    let mut count = 0;
    for i in 0..column_length {
        let column: Vec<i32> = block.iter()
            .map(|v| v[i])
            .collect();
        for k in (0..column_length).rev() {
            let column_rev: Vec<i32> = block.iter()
                .map(|v| v[k])
                .collect();
            if k != i && (i == 0 || k == column_length - 1) && column == column_rev {
                count += 1;
                if ocurrence == count {
                    return Some((i, k));
                }
            }
        }   
    }

    if ocurrence == count {
        return Some((0, 0));
    }

    return None;
}

fn range_rows(block: &Vec<Vec<i32>>, ocurrence: i32) -> Option<(usize, usize)> {
    let mut count = 0;
    for (i, row) in block.iter().enumerate() {
        for (k, row_rev) in block.iter().enumerate().rev() {
            if k != i && (i == 0 || k == block.len() - 1) && row == row_rev {
                count += 1;
                if ocurrence == count {
                    return Some((i, k));
                }
            }
        }
    }

    if ocurrence == count {
        return Some((0, 0));
    }

    return None;
}

fn match_vertical(block: &Vec<Vec<i32>>, points: &(usize, usize)) -> Option<(usize, usize)> {
    let mut index = points.0;
    let mut index_rev = points.1;

    if (index_rev - index) % 2 == 0 {
        return None;
    }

    while (index as i32 - index_rev as i32) < 0 {
        let value: Vec<i32> = block.iter()
            .map(|v| v[index])
            .collect();
        let value_rev: Vec<i32> = block.iter()
                .map(|v| v[index_rev])
                .collect();
        if value != value_rev {
            return None;
        }
        index += 1;
        index_rev -= 1;
    }

    return Some((index_rev, index));
}

fn match_horizontal(block: &Vec<Vec<i32>>, points: &(usize, usize)) -> Option<(usize, usize)> {
    let mut index = points.0;
    let mut index_rev = points.1;

    if (index_rev - index) % 2 == 0 {
        return None;
    }

    while (index as i32 - index_rev as i32) < 0 {
        let value = &block[index];
        let value_rev = &block[index_rev];
        if value != value_rev {
            return None;
        }
        index += 1;
        index_rev -= 1;
    }

    return Some((index_rev, index));
}

fn parse_input(content: String) -> Vec<Vec<Vec<i32>>> {
    let blocks: Vec<&str> = content.split("\r\n\r\n").collect();
    let vectorized_blocks = blocks.iter()
        .map(|b| {
            let rows = b.split("\n");
            return rows.map(|r| {
                return r.trim().chars().map(|c| match c {
                    POINT_ASH => 0,
                    POINT_ROCK => 1,
                    _ => panic!("Oh!")
                })
                .collect();
            })
            .collect();
        })
        .collect();
    return vectorized_blocks;
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