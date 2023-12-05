use std::{fs, collections::HashMap};

const CONSOLE_RESET: &str = "\x1b[0m";
const CONSOLE_SUCESS: &str = "\x1b[32m";
const CONSOLE_FAIL: &str = "\x1b[31m";
const CONSOLE_POWER: &str = "\x1b[34m";
const CONSOLE_RESULT: &str = "\x1b[33m";

const ATTR_SOURCE: &str = "SOURCE";
const ATTR_DESTINATION: &str = "DESTINATION";
const ATTR_LENGTH: &str = "LENGTH";

pub(crate) fn main() {
    exercise_1_1();
    exercise_1_2();
}

fn exercise_1_1() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 5.1 |");
    println!("----------------");
    println!("\n");

    let contents = fs::read_to_string("./resources/EXERCISE_V.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let mut lines = contents.split("\r\n\r\n");

    let seeds = parse_seeds(lines.next().unwrap());
    let soil = parse_map(lines.next().unwrap());
    let fertilizer = parse_map(lines.next().unwrap());
    let water = parse_map(lines.next().unwrap());
    let light = parse_map(lines.next().unwrap());
    let temperature = parse_map(lines.next().unwrap());
    let humidity = parse_map(lines.next().unwrap());
    let location = parse_map(lines.next().unwrap());

    let mut result = i64::MAX;

    for seed in seeds {
        println!("Seed [{}{}{}]", CONSOLE_POWER, seed, CONSOLE_RESET);
        let l_soil = find_destiny(seed, soil.clone());
        println!(" - Soil [{}{}{}]", CONSOLE_POWER, l_soil, CONSOLE_RESET);
        let l_fertilizer = find_destiny(l_soil, fertilizer.clone());
        println!(" - Fertilizer [{}{}{}]", CONSOLE_POWER, l_fertilizer, CONSOLE_RESET);
        let l_water = find_destiny(l_fertilizer, water.clone());
        println!(" - Water [{}{}{}]", CONSOLE_POWER, l_water, CONSOLE_RESET);
        let l_light = find_destiny(l_water, light.clone());
        println!(" - Light [{}{}{}]", CONSOLE_POWER, l_light, CONSOLE_RESET);
        let l_temperature = find_destiny(l_light, temperature.clone());
        println!(" - Temperature [{}{}{}]", CONSOLE_POWER, l_temperature, CONSOLE_RESET);
        let l_humidity = find_destiny(l_temperature, humidity.clone());
        println!(" - Humidity [{}{}{}]", CONSOLE_POWER, l_humidity, CONSOLE_RESET);
        let location = find_destiny(l_humidity, location.clone());
        println!("- Location [{}{}{}].", CONSOLE_POWER, location, CONSOLE_RESET);

        if location < result {
            println!("\nNew lower location found [{}{}{}].", CONSOLE_SUCESS, location, CONSOLE_RESET);
            result = location;
        } else {
            println!("\n{}Not found{}.", CONSOLE_FAIL, CONSOLE_RESET);
        }

        println!()
        //locations.push(l_location);
    }

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);

}

fn find_destiny(key: i64, maps: Vec<HashMap<String, i64>>) -> i64 {
    for map in maps {
        let destination = *map.get(&ATTR_DESTINATION.to_owned()).unwrap();
        let source = *map.get(&ATTR_SOURCE.to_owned()).unwrap();
        let lenght = *map.get(&ATTR_LENGTH.to_owned()).unwrap();
        if key >= source && key < source + lenght {
            let result = key - source;
            return destination + result;
        }
    }
    return key;
}

fn parse_seeds(seeds_string: &str) -> Vec<i64> {
    let clean_seeds = seeds_string.split(": ").last().unwrap();
    let seeds: Vec<i64> = clean_seeds.split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect();
    return seeds;
}

fn parse_map(map_string: &str) -> Vec<HashMap<String, i64>> {
    let mut maps: Vec<HashMap<String, i64>> = Vec::new();
    let mut lines = map_string.split("\n");
    lines.next();
    for line in lines {
        let mut map: HashMap<String, i64> = HashMap::new();
        let instructions: Vec<i64> = line.split(" ")
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect();
        map.insert(ATTR_DESTINATION.to_owned(), *instructions.get(0).unwrap());
        map.insert(ATTR_SOURCE.to_owned(), *instructions.get(1).unwrap());
        map.insert(ATTR_LENGTH.to_owned(), *instructions.get(2).unwrap());
        
        maps.push(map);
    }

    return maps;
}

/*
*
* -----------------------------> SECOND ROUND <-----------------------------
*
*/

fn exercise_1_2() {

    println!("\n");
    println!("----------------");
    println!("| EXERCISE 5.2 |");
    println!("----------------");
    println!("\n");

    //TODO: TODO.

}