use core::panic::PanicInfo;
use std::collections::HashSet;
use std::{fmt, fs};
use std::cmp::max;
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
    //let raw_input = "12345".to_string();
    let parse_input: Vec<i32> = raw_input.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
    let p1 = solve_p1(&parse_input);
    println!("p1: {}", p1);
    let p2 = solve_p2(&parse_input);
    println!("p2: {}", p2);
    //p2 = 8593662006385 to high ::(

    //assert_eq!(p1, 6390180901651);
    // assert_eq!(p2, );
}



fn solve_p1(input: &Vec<i32>) -> i64{
    println!("input: {:?}", input);
    //parse input as representation
    let representation = create_representation(input);
    println!("representation: {:?}", representation);
    println!("representation: {}", representation.join(""));
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
    println!("representation_copy: {}", representation_copy.join(""));
    representation_copy
        .iter()
        .enumerate()
        .map(|(i, x)| { (i as i64) * x.parse::<i64>().unwrap_or(0)})
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
    //println!("input: {:?}", input);
    //parse input as representation
    let (files, spaces) = create_representation_p2(input);
    let size = max(files[files.len()-1].position.1,  spaces[spaces.len()-1].1);
    let representation = create_representation_from_files_and_spaces(&files, size);
    //println!("files {:?}", files);
    //println!("spaces {:?}", spaces);
    //println!("representation {}", representation.join(""));
    let mut new_files = Vec::new();
    let mut spaces_for_iteration = spaces.clone();
    for file in files.iter().rev(){
        let (new_file, new_spaces) = one_step(&file, &spaces_for_iteration);
        new_files.push(new_file);
        spaces_for_iteration = new_spaces.clone();
        //let new_representtaion = create_representation_from_files_and_spaces(&new_files, size);
        //println!("new_representtaion: {}", new_representtaion.join(""));
        // let file_len = file.position.1 - file.position.0;
        // if file_len == 0{
        //     panic!("file is empty");
        // }
        // //println!("trying to move file: {:?} with len {}", file, file_len);
        // if let Some(possible_space)= spaces.iter_mut().find(|el| (el.1 - el.0) >= file_len){
        //
        //     //println!("can be move to space: {:?}", possible_space);
        //     let new_file = File{_id: file._id, position: (possible_space.0, possible_space.0 + file_len)};
        //     possible_space.0 = possible_space.0 + file_len;
        //
        //     new_files.push(new_file);
        //     //let new_representtaion = create_representation_from_files_and_spaces(&new_files, &spaces);
        //     //println!("spaces: {:?}", spaces);
        // }else{
        //     //println!("can't find space");
        //     new_files.push(file.clone());
        // }
        //println!("new files {:?}", new_files);
    }
    let new_representtaion = create_representation_from_files_and_spaces(&new_files, size);
    println!("new representation: {}", new_representtaion.join(""));
    new_representtaion
        .iter()
        .enumerate()
        .map(|(i, x)| { (i as i64) * x.parse::<i64>().unwrap_or(0)})
        .sum()


}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct File{
    _id: i32,
    position: (usize, usize),
}

pub fn create_representation_p2(input: &Vec<i32>) -> (Vec<File>, Vec<(usize, usize)>){
    //let mut representation = Vec::new();
    let mut spaces = Vec::new();
    let mut files = Vec::new();
    let mut block_id = 0;
    let mut last_position = 0;
    for index in 0..input.len(){
        let value  = input[index];
        let is_file = index % 2 == 0;
        if is_file{
            let file = File{_id: block_id, position: (last_position, last_position+value as usize)};
            files.push(file);
            //let block_representation = vec![block_id.to_string(); value as usize];
            block_id += 1;
            //representation.extend(block_representation);
            last_position = last_position + value as usize;
        }else{
            //if value > 0 {
                let space = (last_position, last_position + value as usize);
                spaces.push(space);
                last_position = last_position + value as usize;
            //}
        }

    }
    (files, spaces)
}

fn create_representation_from_files_and_spaces(files: &Vec<File>, size: usize) -> Vec<String>{

    let mut representation = vec![".".to_string(); size];
    for file in files{
        let file_size= file.position.1 - file.position.0;
        let vector = vec![file._id.to_string(); file_size];
        representation.splice(file.position.0..file.position.1, vector);
    }
    representation

}

fn one_step(file: &File, spaces: &Vec<(usize, usize)>) -> (File, Vec<(usize, usize)>){
    let file_len = file.position.1 - file.position.0;
    for (i, space) in spaces.iter().enumerate() {
        if space.0 > file.position.0{
            break;
        }
        let space_size = space.1 - space.0;
        if space_size >= file_len {
            // file can be moved :tada
            let new_file = File { _id: file._id, position: (space.0, space.0 + file_len) };
            let new_space = (space.0 + file_len, space.1);
            let mut new_spaces = spaces.clone();
            new_spaces[i] = new_space;
            return (new_file, new_spaces);
        }
    }
    // no space found
    (file.clone(), spaces.clone())

}