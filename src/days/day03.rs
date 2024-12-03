use std::fs;
use regex::Regex;


pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day03.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let p1 = solve_p1(&raw_input);
    println!("p1: {}", p1);
    let p2 = solve_p2(&raw_input);
    println!("p2: {}", p2);
    assert_eq!(p1, 159833790);
    assert_eq!(p2, 89349241);
}

fn solve_p1(input: &str) -> i32{
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut total = 0;
    for mat in re.find_iter(input) {
        let (left_value, right_value) = parse_to_int(mat.as_str());
        total += left_value * right_value;
    }
    total
}

fn parse_to_int(s: &str) -> (i32, i32){
    let numbers = &s[4..s.len()- 1];
    let splited = numbers.split(",").collect::<Vec<&str>>();
    let left_value: i32 = splited[0].parse().unwrap();
    let right_value: i32 = splited[1].parse().unwrap();
    (left_value, right_value)
}


fn solve_p2(input: &str) -> i32{
    let skip_command = r"don't()";
    let do_command = r"do()";
    let pattern =  format!(r"mul\(\d+,\d+\)|{}|{}", regex::escape(skip_command), regex::escape(do_command));
    let re = Regex::new(&pattern).unwrap();
    let mut execute = true;
    let mut total = 0;
    for ma in re.find_iter(input){
        let match_as_string = ma.as_str();
        if (match_as_string == do_command){
            execute = true;
        }else if (match_as_string == skip_command){
            execute = false;
        }else if execute == true{
            let (left_value, right_value) = parse_to_int(match_as_string);
            total += left_value * right_value;
        }

    }
    total
}