use core::panic::PanicInfo;
use std::collections::HashSet;
use std::{fmt, fs};
use std::f32::consts::PI;
use std::process::id;
use ahash::{HashMap, HashMapExt};
use itertools::{join, Itertools};
use num::traits::real::Real;
use crate::utils::{Array2D};
use strum_macros::EnumIter;
use num::integer::gcd;

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day09.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let parse_input: Vec<i32> = raw_input.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
    let p1 = solve_p1(&parse_input);
    println!("p1: {}", p1);
    //let p2 = solve_p2(&parse_input);
    //println!("p2: {}", p2);
    //assert_eq!(p1, 6390180901651);
    // assert_eq!(p2, );
}



fn solve_p1(input: &Vec<i32>) -> i64{
    println!("input: {:?}", input);
    //parse input as representation
    let representation = create_representation(input);
    println!("representation: {:?}", representation);
    let mut spaces: Vec<usize> = representation
        .iter()
        .enumerate()
        .filter(|(_, &ref x)| *x == ".".to_string())
        .map(|(i, _)| i )
        .rev()
        .collect();
    let mut values: Vec<usize> = representation
        .iter()
        .enumerate()
        .filter(|(_, &ref x)| *x != ".".to_string())
        .map(|(i, _)| i )
        .collect();
    let total_file_size = values.len();
    println!("files have size: {}", values.len());
    //println!("values_indices: {:?}", values);
    let mut representation_copy = representation.clone();
    loop {
        //println!("spaces.len: {}", spaces.len());
        let first_space = spaces.pop().unwrap();
        let value = values.pop().unwrap();

        representation_copy[first_space] = representation[value].clone();
        representation_copy[value] = ".".to_string();
        //println!("representation_copy: {}", representation_copy.join(""));
        if let Some(index) = &representation_copy[..total_file_size].iter().position(|x| x == ".") {

         } else {
           break;
    }

    }
    representation_copy[..total_file_size]
        .iter()
        .enumerate()
        .map(|(i, x)| { (i as i64) * x.parse::<i64>().unwrap()})
        .sum()
}

fn is_sorted(representation_copy: &Vec<String>) -> bool{
    //println!("representation_copy: {:?}", representation_copy.join(""));
    let first_index = representation_copy.iter().position(|x| x == ".").unwrap();
    let has_set: HashSet<String> = representation_copy[first_index..].iter().cloned().collect();

    has_set.len() == 1

}

pub fn create_representation(input: &Vec<i32>) -> Vec<String>{
    let mut representation = Vec::new();
    let mut block_id = 0;
    for index in 0..input.len(){
        let value  = input[index];
        let is_file = index % 2 == 0;
        if is_file{
            let block_representation = vec![block_id.to_string(); value as usize];
            block_id += 1;
            representation.extend(block_representation);
        }else{
            let block_representation = vec![".".to_string(); value as usize];
            representation.extend(block_representation);
        }

    }
    println!("block_id {}", block_id);
    representation

}

fn solve_p2(input: &Vec<i32>) -> i64{
    println!("input: {:?}", input);

    0
}