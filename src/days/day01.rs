use std::fs;

pub fn solve() {
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day01.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let (left_values, right_values) = parse_input(&raw_input);
    let p1 = solve_part_1(&left_values, &right_values);
    println!("Part 1: {:?}", p1);
    let p2 = solve_part_2(&left_values, &right_values);
    println!("Part 2: {:?}", p2);

    assert_eq!(p1, 1319616);
    assert_eq!(p2, 27267728);
}

fn parse_input(raw_input: &str) -> (Vec<i32>,Vec<i32>){
    let mut left_values: Vec<i32> = Vec::new();
    let mut right_values: Vec<i32> = Vec::new();
    for line in raw_input.lines(){
        let splitted: Vec<&str> = line.split(' ').collect();
        let left_value: i32 = splitted[0].parse().unwrap();
        let right_value: i32 = splitted[3].parse().unwrap();
        left_values.push(left_value);
        right_values.push(right_value);
    }
    (left_values, right_values)
}

fn solve_part_1(left_values: &Vec<i32>, right_values: &Vec<i32>) -> usize{
    let mut left_values_sorted = left_values.clone();
    left_values_sorted.sort();
    let mut right_values_sorted = right_values.clone();
    right_values_sorted.sort();

    let score = left_values_sorted
        .into_iter()
        .zip(right_values_sorted).map(|(a,b)| (a-b).abs())
        .sum::<i32>() as usize;

    score
}



fn solve_part_2(left_values: &Vec<i32>, right_values: &Vec<i32>) -> usize{
    left_values
        .iter()
        .map(|value | value * number_of_occurences(value, right_values))
        .sum::<i32>() as usize
}

fn number_of_occurences(value: &i32, right_values: &Vec<i32>) -> i32 {
    right_values.iter().map(| a | if a == value { 1 } else { 0 }).sum()
}