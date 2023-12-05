use std::{fs, collections::BTreeMap};

const CONSOLE_RESET: &str = "\x1b[0m";
const CONSOLE_SUCESS: &str = "\x1b[32m";
const CONSOLE_FAIL: &str = "\x1b[31m";
const CONSOLE_POWER: &str = "\x1b[34m";
const CONSOLE_RESULT: &str = "\x1b[33m";

const MAP_SOIL: &str = "SOIL";
const MAP_FERTILIZER: &str = "FERTILIZER";
const MAP_WATER: &str = "WATER";
const MAP_LIGHT: &str = "LIGHT";
const MAP_TEMPERATURE: &str = "TEMPERATURE";
const MAP_HUMIDITY: &str = "HUMIDITY";
const MAP_LOCATION: &str = "LOCATION";

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

    let sources: Vec<&str> = contents.split("\r\n\r\n").collect();
    let seeds = parse_seeds(sources.get(0).unwrap());
    let maps = parse_maps(sources[1..].to_vec());
    let result = find_simple_lower_location(seeds, maps);

    println!("Result: {}{}{}", CONSOLE_RESULT, result, CONSOLE_RESET);
}

fn parse_maps(string_vector: Vec<&str>) -> BTreeMap<String, Vec<BTreeMap<String, i64>>> {
    let mut collection: BTreeMap<String, Vec<BTreeMap<String, i64>>> = BTreeMap::new();

    let mut lines = string_vector.iter();
    let soil = parse_map(lines.next().unwrap());
    collection.insert(MAP_SOIL.to_owned(), soil);
    let fertilizer = parse_map(lines.next().unwrap());
    collection.insert(MAP_FERTILIZER.to_owned(), fertilizer);
    let water = parse_map(lines.next().unwrap());
    collection.insert(MAP_WATER.to_owned(), water);
    let light = parse_map(lines.next().unwrap());
    collection.insert(MAP_LIGHT.to_owned(), light);
    let temperature = parse_map(lines.next().unwrap());
    collection.insert(MAP_TEMPERATURE.to_owned(), temperature);
    let humidity = parse_map(lines.next().unwrap());
    collection.insert(MAP_HUMIDITY.to_owned(), humidity);
    let location = parse_map(lines.next().unwrap());
    collection.insert(MAP_LOCATION.to_owned(), location);

    return collection;
}

fn parse_map(map_string: &str) -> Vec<BTreeMap<String, i64>> {
    let mut maps: Vec<BTreeMap<String, i64>> = Vec::new();
    let mut lines = map_string.split("\n");
    lines.next();
    for line in lines {
        let mut map: BTreeMap<String, i64> = BTreeMap::new();
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

fn find_simple_lower_location(seeds: Vec<i64>, maps: BTreeMap<String, Vec<BTreeMap<String, i64>>>) -> i64 {
    let mut result = i64::MAX;

    for seed in seeds {
        let location = find_location(seed, maps.clone());
        if location < result {
            println!("\nNew lower location found [{}{}{}].", CONSOLE_SUCESS, location, CONSOLE_RESET);
            result = location;
        } else {
            println!("\n{}Not found{}.", CONSOLE_FAIL, CONSOLE_RESET);
        }

        println!();
    }

   return result;

}

fn find_location(seed: i64, maps: BTreeMap<String, Vec<BTreeMap<String, i64>>>) -> i64 {
    println!("Seed [{}{}{}]", CONSOLE_POWER, seed, CONSOLE_RESET);
    let l_soil = find_destiny(seed, maps.get(MAP_SOIL).unwrap().clone());
    println!(" - Soil [{}{}{}]", CONSOLE_POWER, l_soil, CONSOLE_RESET);
    let l_fertilizer = find_destiny(l_soil, maps.get(MAP_FERTILIZER).unwrap().clone());
    println!(" - Fertilizer [{}{}{}]", CONSOLE_POWER, l_fertilizer, CONSOLE_RESET);
    let l_water = find_destiny(l_fertilizer, maps.get(MAP_WATER).unwrap().clone());
    println!(" - Water [{}{}{}]", CONSOLE_POWER, l_water, CONSOLE_RESET);
    let l_light = find_destiny(l_water, maps.get(MAP_LIGHT).unwrap().clone());
    println!(" - Light [{}{}{}]", CONSOLE_POWER, l_light, CONSOLE_RESET);
    let l_temperature = find_destiny(l_light, maps.get(MAP_TEMPERATURE).unwrap().clone());
    println!(" - Temperature [{}{}{}]", CONSOLE_POWER, l_temperature, CONSOLE_RESET);
    let l_humidity = find_destiny(l_temperature, maps.get(MAP_HUMIDITY).unwrap().clone());
    println!(" - Humidity [{}{}{}]", CONSOLE_POWER, l_humidity, CONSOLE_RESET);
    let location = find_destiny(l_humidity, maps.get(MAP_LOCATION).unwrap().clone());
    println!("- Location [{}{}{}].", CONSOLE_POWER, location, CONSOLE_RESET);
   return location;
}

fn find_destiny(key: i64, maps: Vec<BTreeMap<String, i64>>) -> i64 {
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

    let contents = fs::read_to_string("./resources/EXERCISE_V.txt")
        .expect("Oh! Something happens! Merry Christmas!");

    let sources: Vec<&str> = contents.split("\r\n\r\n").collect();
    let seeds = parse_seeds(sources.get(0).unwrap());
    let maps = parse_optimized_maps(sources[1..].to_vec(), true);
    let result = find_lower_location(&seeds, &maps);

    println!("Result: {}{}{}", CONSOLE_RESULT, result.unwrap(), CONSOLE_RESET);
}

fn parse_optimized_maps(string_vector: Vec<&str>, sw_reverse: bool) -> Vec<Vec<Vec<i64>>> {
    let mut lines = string_vector.iter();
    
    let mut collection: Vec<Vec<Vec<i64>>> = Vec::new();

    let soil = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(soil);

    let fertilizer = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(fertilizer);

    let water = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(water);

    let light = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(light);

    let temperature = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(temperature);

    let humidity = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(humidity);

    let location = parse_optimized_map(lines.next().unwrap(), sw_reverse);
    collection.push(location);

    if sw_reverse {
        collection.reverse();
    }

    return collection;
}

fn parse_optimized_map(map_string: &str, sw_reverse: bool) -> Vec<Vec<i64>> {
    let mut vectors: Vec<Vec<i64>> = Vec::new();
    let mut lines = map_string.split("\n");
    lines.next();
    for line in lines {
        let mut vector: Vec<i64> = Vec::new();
        let instructions: Vec<i64> = line.split(" ")
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect();
        vector.push(*instructions.get(1).unwrap());
        vector.push(*instructions.get(2).unwrap());
        vector.push(*instructions.get(0).unwrap());
        
        if sw_reverse {
            vector.reverse();
        }

        vectors.push(vector);
    }
    return vectors;
}

fn find_lower_location(seeds: &Vec<i64>, maps: &Vec<Vec<Vec<i64>>>) -> Option<i64> {
    let mut seeds_clone = seeds.clone();
    seeds_clone.sort();

    let max = *seeds_clone.last().unwrap();
    let mut location = 0;
    while location < max {
        let seed = find_seed_optimized(location, maps);
        let mut index = 0;
        while index < seeds.len() {
            let base = *seeds.get(0 + index).unwrap();
            let length = *seeds.get(1 + index).unwrap();
            if base <= seed && seed <= base + length {
                return Some(location);
            }
            index = index + 2;
        }
        location = location + 1;
    }
   return None;
}

fn find_seed_optimized(key: i64, collection: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut result = key;
    let mut i_collection = collection.iter();
    while let Some(map) = i_collection.next() {
        let mut i_map = map.iter();
        while let Some(fields) = i_map.next() {
            let destination = *fields.get(0).unwrap();
            let lenght = *fields.get(1).unwrap();
            let source = *fields.get(2).unwrap();
            if result >= destination && result < destination + lenght {
                result = source + result - destination;
                break;
            }
        }   
    }
   return result;
}

/*
*
* -------------------------------> MISC UTILS <-------------------------------
*
*/

fn parse_seeds(seeds_string: &str) -> Vec<i64> {
    let clean_seeds = seeds_string.split(": ").last().unwrap();
    let seeds: Vec<i64> = clean_seeds.split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect();
    return seeds;
}