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
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day08.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let input_grid: Array2D<String> = Array2D::parse_2d_input(&raw_input).unwrap();
    let p1 = solve_p1(&input_grid);
    println!("p1: {}", p1);
    let p2 = solve_p2(&input_grid);
    println!("p2: {}", p2);
    assert_eq!(p1, 273);
    assert_eq!(p2, 1017);
}

fn solve_p1(input_grid: &Array2D<String>) -> i64{
    input_grid.display();
    let mut antinodes_positions: HashSet<(i32, i32)> = HashSet::new();
    let possible_frequencies = collect_possible_frequencies();
    println!("possible frequencies: {:?}", possible_frequencies);
    for frequency in possible_frequencies{
        let antinodes = find_antinodes_for_given_frequency(input_grid, frequency);
        antinodes_positions.extend(antinodes);
        //println!("antinodes positions: {:?}", antinodes_positions);
    }
    let mut helping_grid = input_grid.clone();
    for (x, y) in &antinodes_positions{
        helping_grid.set(*x as usize, *y as usize, "X".to_string());
    }
    helping_grid.display();
    antinodes_positions.len() as i64
}

fn find_antinodes_for_given_frequency(input_grid: &Array2D<String>, frequency: String) -> Vec<(i32, i32)>{
    let all_positions = input_grid.where_value(frequency.clone());
    if all_positions.len() == 0{
        return vec![];
    }
    println!("Frequency {} is on positions: {:?}", frequency, all_positions);
    let mut frequency_antinodes = Vec::new();
    for comb in all_positions.iter().combinations(2){
        //println!("Combination {:?} : {:?}", comb[0], comb[1]);
        let (first, second) = find_antinodes_for_given_pair(*comb[0], *comb[1]);
        if is_valid(first, input_grid.get_rows(), input_grid.get_cols()){
            frequency_antinodes.push(first);
        }
        if is_valid(second, input_grid.get_rows(), input_grid.get_cols()){
            frequency_antinodes.push(second);
        }
    }
    frequency_antinodes
}

fn is_valid(antinode: (i32, i32), rows: usize, cols: usize) -> bool{
    let (x, y) = antinode;
    x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct ParametricLine{
    // line in 2d space between points a, b
    // y(y) = a + t * (b - a )
    a: (i32, i32),
    b: (i32, i32),
}
impl ParametricLine {
    pub fn new(a: (i32, i32), b: (i32, i32)) -> ParametricLine {
        ParametricLine{a, b}
    }

    pub fn get(&self, t: i32) -> (i32, i32){
        let x = self.a.0 + t*self.direction_vector().0;
        let y = self.a.1 + t*self.direction_vector().1;
        (x, y)
    }
    pub fn direction_vector(&self) -> (i32, i32){
        (self.b.0 - self.a.0, self.b.1 - self.a.1)
    }

    pub fn angle(&self) -> f32{
        // Compute the angle in radians
        let (b, a) = self.direction_vector();
        let angle_radians = (b as f32).atan2(a as f32);

        // Convert to degrees (optional)
        let angle_degrees = angle_radians * 180.0 / PI;
        angle_degrees
    }
    pub fn get_all_points(&self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Vec<(i32, i32)>{
        vec![]
    }
}

fn find_antinodes_for_given_pair(a: (usize, usize), b: (usize, usize)) -> ((i32, i32), (i32, i32)){
    // having 2 point in 2d compute line between them, compute distance and
    let line = ParametricLine{ a: (a.0 as i32, a.1 as i32), b: (b.0 as i32, b.1 as i32)};
    println!("Line with points {:?} - {:?} has angle {}", a, b, line.angle());
    let first = line.get(-1);
    let second = line.get(2);
    //println!("For pair {:?}-{:?} we found: {:?} and {:?}", a, b, first, second);
    (first, second)
}


fn get_distance(a: (usize, usize), b: (usize, usize)) -> f32{
    let (x_1, y_1) = a;
    let (x_2, y_2) = b;
    let distance_squared = (x_1 as i32 - x_2 as i32).abs() + (y_1 as i32 - y_2 as i32).abs();
    (distance_squared as f32).sqrt()
}

fn collect_possible_frequencies() -> Vec<String>{
    let numbers: Vec<String> = (0..=9).into_iter().map(|el| el.to_string()).collect();
    let lowercase: Vec<String> = ('a'..='z').map(|el| el.to_string()).collect();
    let upper: Vec<String> = ('A'..='Z').map(|el| el.to_string()).collect();

    let mut all = numbers.clone();
    all.extend(lowercase);
    all.extend(upper);

    all
}

fn solve_p2(input_grid: &Array2D<String>) -> i64{
    input_grid.display();
    let mut antinodes_positions: HashSet<(i32, i32)> = HashSet::new();
    let possible_frequencies = collect_possible_frequencies();
    println!("possible frequencies: {:?}", possible_frequencies);
    for frequency in possible_frequencies{
        let antinodes = find_all_antinodes_for_given_frequency(input_grid, frequency);
        antinodes_positions.extend(antinodes);
        //println!("antinodes positions: {:?}", antinodes_positions);
    }
    let mut helping_grid = input_grid.clone();
    for (x, y) in &antinodes_positions{
        helping_grid.set(*x as usize, *y as usize, "X".to_string());
    }
    helping_grid.display();
    antinodes_positions.len() as i64
}

fn find_all_antinodes_for_given_frequency(input_grid: &Array2D<String>, frequency: String) -> Vec<(i32, i32)>{
    let all_positions = input_grid.where_value(frequency.clone());
    if all_positions.len() == 0{
        return vec![];
    }
    println!("Frequency {} is on positions: {:?}", frequency, all_positions);
    let mut frequency_antinodes = Vec::new();
    for comb in all_positions.iter().combinations(2){
        //println!("Combination {:?} : {:?}", comb[0], comb[1]);
        let frequency_antinote = find_all_antinodes_for_given_pair(*comb[0], *comb[1], input_grid.get_rows(), input_grid.get_cols());
        frequency_antinodes.extend(frequency_antinote);

    }
    frequency_antinodes
}

fn find_all_antinodes_for_given_pair(a: (usize, usize), b: (usize, usize), rows: usize, cols: usize) -> Vec<(i32, i32)>{
    // having 2 point in 2d compute line between them, compute distance and
    let line = ParametricLine{ a: (a.0 as i32, a.1 as i32), b: (b.0 as i32, b.1 as i32)};
    let x_diff = (b.0 as i32 - a.0 as i32).abs();
    let y_diff = (b.1 as i32 - a.1 as i32).abs();
    let gcd = gcd(x_diff, y_diff);
    if gcd > 1{
        panic!("Not implemented")
    }
    let mut antinotes = Vec::new();
    let mut t = 0;
    loop {
        let possible_antinote = line.get(t);
        if !is_valid(possible_antinote, rows, cols){
            break;
        }
        antinotes.push(possible_antinote);
        t = t -1;
    }
    t = 1;
    loop {
        let possible_antinote = line.get(t);
        if !is_valid(possible_antinote, rows, cols){
            break;
        }
        antinotes.push(possible_antinote);
        t = t + 1;
    }
    //println!("For line {:?}-{:?}, gcd({},{}) = {}", a, b, x_diff, y_diff, gcd);
    //line.get_all_points(0, rows as i32, 0, cols as i32)
    antinotes
}