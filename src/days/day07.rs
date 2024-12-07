use core::panic::PanicInfo;
use std::collections::HashSet;
use std::{fmt, fs};
use std::process::id;
use ahash::{HashMap, HashMapExt};
use itertools::{join, Itertools};
use crate::utils::{Array2D};
use strum_macros::EnumIter;

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day07.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let p1 = solve_p1(&raw_input);
    println!("p1: {}", p1);
    let p2 = solve_p2(&raw_input);
    println!("p2: {}", p2);
    assert_eq!(p1, 932137732557);
    assert_eq!(p2, 661823605105500);
}



fn solve_p1(raw_input: &str) -> i64{
    let mut result = 0;
    let possible_operations = vec![Operation::Add, Operation::Multiply];
    for line in raw_input.lines(){
        let result_value = line_can_be_solved(line, &possible_operations);
        result += result_value;
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Add,
    Concatenate,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction_str = match self {
            Operation::Multiply => "*",
            Operation::Add => "+",
            Operation::Concatenate => "||",
        };
        write!(f, "{}", direction_str)
    }
}


fn line_can_be_solved(line: &str, possible_operations: &Vec<Operation>) -> i64{
    let (resulting_value, operands) = parse_line(line);
    //println!("Will try to get {} with operand {:?}", resulting_value, operands);
    let length = operands.len() - 1;
    for perm in (0..length).map(|_| possible_operations.clone()).multi_cartesian_product(){

        let mut perm_with_first_add = perm.clone();
        perm_with_first_add.insert(0, Operation::Add);
        let actual_result = solve_line(&operands, &perm_with_first_add);
        //println!("With perm: {:?}, the solution is {}", perm, actual_result);
        if actual_result == resulting_value{
            return actual_result;
        }
    }
    0
}

fn solve_line(operands: &Vec<i64>, operations: &Vec<Operation>) -> i64{
    //println!("Solving line with {:?} operations and {:?} operations", operands, operations);
    let mut solution = 0;
    for (right, operation) in operands.into_iter().zip(operations) {
        solution = single_operation(solution, *right, operation);
        //println!(" Partial solution: {}", solution);
    }
    solution
}

fn single_operation(left: i64, right: i64, operation: &Operation) -> i64{
    //println!("computing {}, {}, {}", left, operation, right);
    match operation {
        Operation::Add => left + right,
        Operation::Multiply => left * right,
        Operation::Concatenate => [left.to_string(), right.to_string()].join("").parse::<i64>().unwrap(),
    }
}

fn parse_line(line: &str) -> (i64, Vec<i64>){
    let first_split = line.split(":").collect::<Vec<&str>>();
    let resulting_value = first_split[0].parse::<i64>().unwrap();
    let operands: Vec<i64> = first_split[1]
        .split(" ")
        .filter_map(|el| el.parse::<i64>().ok())
        .collect();

    (resulting_value, operands)
}


fn solve_p2(raw_input: &str) -> i64 {
    let mut result = 0;
    let possible_operations = vec![Operation::Add, Operation::Multiply, Operation::Concatenate];
    for line in raw_input.lines(){
        let result_value = line_can_be_solved(line, &possible_operations);
        result += result_value;
    }
    result

}

