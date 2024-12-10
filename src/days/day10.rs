use core::panic::PanicInfo;
use std::collections::HashSet;
use std::fs;
use std::process::id;
use ahash::{HashMap, HashMapExt};
use crate::utils::{find_all_paths_bfs_one_step, multiple_bfs, multiple_bfs_with_one_step, Array2D};

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day10.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let grid_input: Array2D<i32> = Array2D::parse_2d_input(&raw_input).unwrap();
    //println!("grid input, {:?}", grid_input);
    //grid_input.display();
    let p1 = solve_p1(&grid_input);
    println!("p1: {}", p1);
    let p2 = solve_p2(&grid_input);
    println!("p2: {}", p2);
    // assert_eq!(p1, 825);
    // assert_eq!(p2, 1805);
}



fn solve_p1(grid_input: &Array2D<i32>) -> i32{
    grid_input.display();
    let trail_heads = grid_input.where_value(0);
    let trail_tails = grid_input.where_value(9);
    //println!("trailheads {:?}", trail_heads);
    //println!("trail_tails {:?}", trail_tails);
    trail_heads
        .iter()
        .map(|trailhead | compute_score(*trailhead, grid_input, &trail_tails))
        .sum()

}

fn compute_score(trailhead: (usize, usize), grid_input: &Array2D<i32>, trail_tails: &Vec<(usize, usize)>) -> i32 {
    //println!("computing score for trailhead {:?}", trailhead);
    let result = multiple_bfs_with_one_step(trailhead, trail_tails, grid_input);
    // for (path, trail_tail) in result.into_iter().zip(trail_tails) {
    //     println!("for tail {:?} found path {:?}", trail_tail, path);
    // }
    result.iter().map(|path| (path.len() > 0) as i32).sum()
}

fn solve_p2(grid_input: &Array2D<i32>) -> i32 {
    let trail_heads = grid_input.where_value(0);
    let trail_tails = grid_input.where_value(9);

    let mut score = 0;
    for head in trail_heads.iter(){
        let mut head_score = 0;
        for tail in trail_tails.iter(){
            let found_paths = find_all_paths_bfs_one_step(*head, *tail, grid_input);
            //println!("From {:?} to {:?} there are {} paths", head, tail, found_paths.len());
            head_score += found_paths.len();
        }
        //println!("head {:?} score: {}", head, head_score);
        score += head_score;
    }
    //find_all_paths_bfs_one_step(trail_heads[0], trail_tails[2], grid_input);
    score as i32
}

