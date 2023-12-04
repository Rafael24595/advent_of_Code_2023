use std::fs;

const CONSOLE_RESET: &str = "\x1b[0m";
const CONSOLE_SUCESS: &str = "\x1b[32m";
const CONSOLE_FAIL: &str = "\x1b[31m";
const CONSOLE_RESULT: &str = "\x1b[33m";

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 4.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_IV.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let lines = contents.split("\n");

    let mut result = 0;

    for (_, line) in lines.clone().into_iter().enumerate() {
        let title = line.split(": ").next();
        println!("{}", title.unwrap());
        let card = line.split(": ").last();
        let mut card_tuple = card.unwrap().split(" | ");
        let winners: Vec<i32> = card_tuple.next().unwrap().split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let numbers: Vec<i32> = card_tuple.next().unwrap().split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        result = result + evalue(winners, numbers);

        println!("");
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);

}

fn evalue(winners: Vec<i32>, numbers: Vec<i32>) -> i32 {
    let mut matches = Vec::new();
    for winner in winners {
        if numbers.contains(&winner) {
            matches.push(winner);
        }
    }

    if matches.len() == 0 {
        println!(" - Matches: {}NOTFOUND{}", CONSOLE_FAIL, CONSOLE_RESET);
        return 0;
    }

    let exp = matches.len() as i32 - 1;
    let result = (2 as i32).pow(exp as u32);
    let s_matches: Vec<String> = matches.iter().map(|n| n.to_string()).collect();

    println!(" - Matches: [{}{}{}] => {}{}{}", 
        CONSOLE_SUCESS, s_matches.join(", "), CONSOLE_RESET, 
        CONSOLE_RESULT, result, CONSOLE_RESET);

    return result;
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {
    //TODO: TODO.
}