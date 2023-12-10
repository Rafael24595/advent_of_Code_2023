use std::fs;
use crate::_utils::CONSOLE_COLORS;

const POINT_OUT: char = '#';
const POINT_START: char = 'S';
const POINT_GROUND: char = '.';
const POINT_PIPE_NS: char = '|';
const POINT_PIPE_WE: char = '-';
const POINT_PIPE_NE: char = 'L';
const POINT_PIPE_NW: char = 'J';
const POINT_PIPE_SW: char = '7';
const POINT_PIPE_SE: char = 'F';

const DIRECTION_VOID: i32 = 0;
const DIRECTION_TOP: i32 = 1;
const DIRECTION_LEFT: i32 = 2;
const DIRECTION_RIGHT: i32 = 3;
const DIRECTION_BOTTOM: i32 = 4;

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

    let contents = fs::read_to_string("./resources/EXERCISE_X.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let matrix = &parse_matrix(contents);
    let start = find_entrance(matrix).unwrap();

    let result = evalue(start, matrix);

    println!("Result: {}", CONSOLE_COLORS::CONSOLE_RESULT.wrap(result));
}

fn evalue(start: (usize, usize), matrix: &Vec<Vec<char>>) -> i64 {
    let mut sw_exit = false;
    let mut steps = 0;

    let mut current = start;
    let mut direction: i32 = DIRECTION_VOID;

    while !sw_exit {
        let result = evalue_step(direction, current, matrix);
        current = result.1;
        direction = result.0;
        steps = steps + 1;
        if current.0 == start.0 && current.1 == start.1 {
            sw_exit = true;
        }
    }

    return steps / 2;
}

fn evalue_step(last_direction: i32, step: (usize, usize), matrix: &Vec<Vec<char>>) -> (i32, (usize, usize)) {
    let mut current = *matrix.get(step.0).unwrap().get(step.1).unwrap();

    if current == POINT_START {
        current = find_start_value(step, matrix);
    }

    let top = last_direction != DIRECTION_BOTTOM && 
        (current == POINT_PIPE_NS || current == POINT_PIPE_NE || current == POINT_PIPE_NW);
    let bottom = last_direction != DIRECTION_TOP &&
        (current == POINT_PIPE_NS || current == POINT_PIPE_SW || current == POINT_PIPE_SE);
    let left = last_direction != DIRECTION_RIGHT &&
        (current == POINT_PIPE_WE || current == POINT_PIPE_SW || current == POINT_PIPE_NW);
    let right = last_direction != DIRECTION_LEFT &&
        (current == POINT_PIPE_WE || current == POINT_PIPE_NE || current == POINT_PIPE_SE);

    if top {
        return (DIRECTION_TOP, (step.0 - 1, step.1));
    }

    if bottom {
        return (DIRECTION_BOTTOM, (step.0 + 1, step.1));
    }

    if left {
        return (DIRECTION_LEFT, (step.0, step.1 - 1));
    }

    if right {
        return (DIRECTION_RIGHT, (step.0, step.1 + 1));
    }

    panic!("Impossible step.");
}

fn find_start_value(step: (usize, usize), matrix: &Vec<Vec<char>>) -> char {
    let characters = [POINT_PIPE_NS, POINT_PIPE_WE, POINT_PIPE_NE,
        POINT_PIPE_NW, POINT_PIPE_SW, POINT_PIPE_SE];

    for character in characters {
        let mut char_matches: Vec<char> = Vec::new();

        if step.1 > 0 {
            let left = *matrix.get(step.0).unwrap().get(step.1 - 1).unwrap();
            if pipes_matches(character, left, DIRECTION_LEFT) {
                char_matches.push(left);
            }
        }
        
        if step.1 < matrix.get(step.0).unwrap().len() {
            let right = *matrix.get(step.0).unwrap().get(step.1 + 1).unwrap();
            if pipes_matches(character, right, DIRECTION_RIGHT) {
                char_matches.push(right);
            }
        }
    
        if step.0 > 0 {
            let top = *matrix.get(step.0 - 1).unwrap().get(step.1).unwrap();
            if pipes_matches(character, top, DIRECTION_TOP) {
                char_matches.push(top);
            }
        }
        
        if step.0 < matrix.len() {
            let bottom = *matrix.get(step.0 + 1).unwrap().get(step.1).unwrap();
            if pipes_matches(character, bottom, DIRECTION_BOTTOM) {
                char_matches.push(bottom);
            }
        }

        if char_matches.len() == 2 {
            return character;
        }

    }


    panic!("Character not found");
}

fn pipes_matches(current: char, pipe: char, direction: i32) -> bool {
    if pipe == POINT_OUT || pipe == POINT_GROUND {
        return false;
    }
    match current {
        POINT_PIPE_NS => match direction {
            DIRECTION_LEFT | DIRECTION_RIGHT => return false,
            DIRECTION_TOP => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_SW ||
                pipe == POINT_PIPE_SE,
            DIRECTION_BOTTOM => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_NW,
            _ => return false
        }
        POINT_PIPE_WE => match direction {
            DIRECTION_TOP | DIRECTION_BOTTOM => return false,
            DIRECTION_LEFT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_SE,
            DIRECTION_RIGHT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NW ||
                pipe == POINT_PIPE_SW,
            _ => return false,
        }
        POINT_PIPE_NE => match direction {
            DIRECTION_BOTTOM | DIRECTION_LEFT => return false,
            DIRECTION_TOP => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_SW ||
                pipe == POINT_PIPE_SE,
            DIRECTION_RIGHT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NW ||
                pipe == POINT_PIPE_SW,
            _ => return false,
        }
        POINT_PIPE_NW => match direction {
            DIRECTION_BOTTOM | DIRECTION_RIGHT => return false,
            DIRECTION_TOP => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_SW ||
                pipe == POINT_PIPE_SE,
            DIRECTION_LEFT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_SE,
            _ => return false,
        }
        POINT_PIPE_SW => match direction {
            DIRECTION_TOP | DIRECTION_RIGHT => return false,
            DIRECTION_LEFT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_SE,
            DIRECTION_BOTTOM => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_NW,
            _ => return false,
        }
        POINT_PIPE_SE => match direction {
            DIRECTION_TOP | DIRECTION_LEFT => return false,
            DIRECTION_RIGHT => return 
                pipe == POINT_PIPE_WE ||
                pipe == POINT_PIPE_NW ||
                pipe == POINT_PIPE_SW,
            DIRECTION_BOTTOM => return 
                pipe == POINT_PIPE_NS ||
                pipe == POINT_PIPE_NE ||
                pipe == POINT_PIPE_NW,
            _ => return false,
        }
        _ => return false,
    }
}

fn parse_matrix(contents: String) -> Vec<Vec<char>> {
    let line_vector: Vec<&str> = contents.split("\n").collect();
    return line_vector.iter()
        .map(|l| l
            .trim()
            .split("")
            .filter(|t| !t.is_empty())
            .map(|s| s.chars().last().unwrap())
            .collect())
        .collect();
}

fn find_entrance(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == POINT_START {
                return Some((y, x));
            }
        }
    }
    return None;
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