use core::panic::PanicInfo;
use std::collections::HashSet;
use std::fs;
use std::process::id;
use ahash::{HashMap, HashMapExt};
use crate::utils::{Array2D};

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day06.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let grid_input: Array2D<String> = Array2D::parse_2d_input(&raw_input).unwrap();
    //println!("grid input, {:?}", grid_input);
    //grid_input.display();
    let p1 = solve_p1(&grid_input);
    println!("p1: {}", p1);
    let p2 = solve_p2(&grid_input);
    println!("p2: {}", p2);
    // assert_eq!(p1, 5145);
    // assert_eq!(p2, 1523);
}



fn solve_p1(grid_input: &Array2D<String>) -> i32{
    let mut direction = direction_from_string("^");
    let mut visited_locations: Array2D<String> = grid_input.clone();
    let start_position = grid_input.where_value(direction.as_str().to_string())[0];
    visited_locations.set(start_position.0, start_position.1, "X".to_string());
    //println!("start_position: ({}, {})", start_position.0, start_position.1);

    let mut position_vector_already_visited = Vec::new();

    let mut current_position = start_position.clone();
    loop {
        current_position = make_move(grid_input, current_position, &direction, &mut visited_locations);
        if grid_input.is_on_boundary(current_position){
            break;
        }

        let direction_as_str = direction.as_str().to_string();
        let id_position = (current_position.0, current_position.1, direction_as_str);
        if position_vector_already_visited.contains(&id_position){
            println!("I have at the same position {:?} => infinite", id_position);
            return -1;
        }
        position_vector_already_visited.push(id_position);

        direction = turn_right(direction);
        //visited_locations.display();
        //println!("current position {}, {}, direction: {:?}", current_position.0, current_position.1, direction);
        //println!("");
    }
    visited_locations.where_value("X".to_string()).len() as i32
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // Method to get the associated string value
    fn as_str(&self) -> &str {
        match self {
            Direction::North => "^",
            Direction::East => ">",
            Direction::South => "v",
            Direction::West => "<",
        }
    }
}
fn direction_from_string(id: &str) -> Direction {
    match id {
         "^" => Direction::North,
         ">" => Direction::East,
         "v" => Direction::South,
         "<" => Direction::West,
        &_ => panic!("Something went wrong, unkown id {}!", id),
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction{
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn make_move(grid_input: &Array2D<String>, current_position: (usize, usize), direction: &Direction, visited_locations: &mut Array2D<String>) -> (usize, usize){
    let mut new_position = current_position.clone();

    while !grid_input.is_on_boundary(new_position) {
        let possible_next_position = make_one_step(new_position, &direction);
        let value = grid_input.get(possible_next_position.0, possible_next_position.1);
        if value == "#" {
            break;
        }
        new_position = possible_next_position;
        visited_locations.set(new_position.0, new_position.1, "X".to_string());
    }
    new_position
}

fn make_one_step(current_position: (usize, usize), direction: &Direction) -> (usize, usize){
    match direction {
        Direction::North => (current_position.0 - 1, current_position.1),
        Direction::East => (current_position.0, current_position.1 + 1),
        Direction::South => (current_position.0 + 1, current_position.1),
        Direction::West => (current_position.0, current_position.1 - 1),
    }
}

fn solve_p2(grid_input: &Array2D<String>) -> i32 {
    // todo: this brute force should be much smaller
    let mut infinite_obtiscles = Vec::new();
    for i in 0..grid_input.get_rows() {
        for j in 0..grid_input.get_cols() {
            let input_symbol = grid_input.get(i, j);
            if input_symbol == "#" || input_symbol == "^" {
                // there is already obsticle nothing to compute
                continue;
            } else {
                // put obstickle on the place and try to solve it
                let mut grid_changed = grid_input.clone();
                grid_changed.set(i, j, "#".to_string());
                let solution = solve_p1(&grid_changed);

                if solution == -1{
                    println!("for i={}, j={} we have loop", i , j);
                    infinite_obtiscles.push((i, j));
                }
            }
        }
    }
    infinite_obtiscles.len() as i32
}

