use std::fs;
use crate::utils::{Array2D};

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day04.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let grid: Array2D<String> = Array2D::parse_2d_input(&raw_input).unwrap();
    println!("grid: {:?}", grid);
    let p1 = solve_p1(&grid);
    println!("p1: {}", p1);
    let p2 = solve_p2(&grid);
    println!("p2: {}", p2);
    assert_eq!(p1, 2654);
    assert_eq!(p2, 1990);
}


fn solve_p1(grid: &Array2D<String>) -> i32{
    let mut nb_of_solutions = 0;
    for i in 0..grid.get_rows(){
        for j in 0..grid.get_cols(){
            let grid_value = grid.get(i, j);
            println!("i: {}, j: {}, grid_value: {}", i, j, grid.get(i, j));
            if grid_value == "X"{
                nb_of_solutions += find_for_given_i_j(&grid, i, j);
            }

        }
    }

    nb_of_solutions as i32
}

fn find_for_given_i_j(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    // ->
    let match_to_the_right = find_to_the_right(grid, i, j);
    let match_to_the_left = find_to_the_left(grid, i, j);
    let match_to_the_up = find_to_the_up(grid, i, j);
    let match_to_the_down = find_to_the_down(grid, i, j);

    let match_the_right_up = find_to_the_right_up(&grid, i, j);
    let match_the_right_down = find_to_the_right_down(&grid, i, j);
    let match_the_left_up = find_to_the_left_up(&grid, i, j);
    let match_the_left_down = find_to_the_left_down(&grid, i, j);

    match_to_the_right + match_to_the_left  + match_to_the_up + match_to_the_down
        + match_the_right_up + match_the_right_down + match_the_left_up + match_the_left_down
}


fn find_to_the_down(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i + 4 > grid.get_rows(){
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for k in i..(i+4){
        chars.push(grid.get(k, j))
    }

    if chars == vector_to_find {
        println!("find chars for down: {:?}, i={:?}, j={}", chars, i..(i+4), j);
        return 1;
    }
    0
}

fn find_to_the_up(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i < 3{
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for k in (i-3..=i).rev(){
        chars.push(grid.get(k, j))
    }

    if chars == vector_to_find {
        println!("chars for up {:?}, i={:?}, j={}", chars, (i-3..=i).rev().collect::<Vec<usize>>(), j);
        return 1;
    }
    0
}
fn find_to_the_left(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if j < 3{
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for k in (j-3..=j).rev(){
        chars.push(grid.get(i, k))
    }

    if chars == vector_to_find {
        println!("find chars left {:?}, i={}, j={:?}", chars, i, (j-3..=j).rev().collect::<Vec<usize>>());
        return 1;
    }
    0
}

fn find_to_the_right(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if j + 4 > grid.get_cols(){
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for k in j..(j+4){
        chars.push(grid.get(i, k))
    }

    if chars == vector_to_find {
        println!("find chars right {:?}, i={}, j={:?}", chars, i, j..(j+4));
        return 1;
    }
    0
}

fn find_to_the_right_up(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i + 4 > grid.get_rows() || j < 3{
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for (k,l) in (i..(i+4)).zip((j-3..=j).rev()){
        chars.push(grid.get(k, l))
    }

    if chars == vector_to_find {
        println!("find chars for right up: {:?}", chars);
        return 1;
    }
    0
}

fn find_to_the_right_down(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i + 4 > grid.get_rows() || j + 4 > grid.get_cols(){
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for (k,l) in (i..(i+4)).zip(j..j+4){
        chars.push(grid.get(k, l))
    }

    if chars == vector_to_find {
        println!("find chars for right down: {:?}", chars);
        return 1;
    }
    0
}

fn find_to_the_left_up(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i < 3 || j < 3{
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for (k,l) in ((i-3..=i).rev()).zip((j-3..=j).rev()){
        chars.push(grid.get(k, l))
    }

    if chars == vector_to_find {
        println!("chars for left up: {:?}", chars);
        return 1;
    }
    0
}

fn find_to_the_left_down(grid: &Array2D<String>, i: usize, j: usize) -> usize {
    let vector_to_find = vec!["X", "M", "A", "S"];
    if i < 3 || j+4 > grid.get_cols(){
        // there is not enough space
        return 0
    }
    let mut chars = Vec::new();
    for (k,l) in ((i-3..=i).rev()).zip(j..j+4){
        chars.push(grid.get(k, l))
    }

    if chars == vector_to_find {
        println!("chars for left down: {:?}", chars);
        return 1;
    }
    0
}


fn parse_to_int(s: &str) -> (i32, i32){
    let numbers = &s[4..s.len()- 1];
    let splited = numbers.split(",").collect::<Vec<&str>>();
    let left_value: i32 = splited[0].parse().unwrap();
    let right_value: i32 = splited[1].parse().unwrap();
    (left_value, right_value)
}


fn solve_p2(grid: &Array2D<String>) -> i32{
    let mut nb_of_solutions = 0;
    for i in 0..grid.get_rows(){
        for j in 0..grid.get_cols(){
            let grid_value = grid.get(i, j);
            println!("i: {}, j: {}, grid_value: {}", i, j, grid.get(i, j));
            if grid_value == "A"{
                nb_of_solutions += find_p2_for_given_i_j(&grid, i, j);
            }

        }
    }

    nb_of_solutions as i32
}

fn find_p2_for_given_i_j(grid: &Array2D<String>, i: usize, j: usize) -> i32{
    if i == 0 || j == 0 || i == grid.get_rows() - 1 || j == grid.get_cols() -1{
        return 0;
    }
    let upper_left = grid.get(i-1, j-1);
    let upper_right = grid.get(i-1, j+1);

    let bottom_left = grid.get(i+1, j-1);
    let bottom_right = grid.get(i+1, j+1);
    println!("upper_left={}, upper_right={}, bottom_left={}, bottom_right={}", upper_left, upper_right, bottom_left, bottom_right);

    if (upper_right == "M" && bottom_left == "S") || (upper_right == "S" && bottom_left == "M"){
        if (upper_left == "M" && bottom_right == "S") || (upper_left == "S" && bottom_right == "M"){
            return 1;
        }
    }

    0
}