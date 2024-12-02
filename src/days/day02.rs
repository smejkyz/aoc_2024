use std::fs;

pub fn solve() {
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day02.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let reports = parse_input(&raw_input);
    let p1 = solve_part_1(&reports);
    println!("Part 1: {:?}", p1);
    //let mut test_reports = Vec::new();
    //test_reports.push( vec![12, 15, 11, 9, 5] );
    let p2 = solve_part_2(&reports);
    println!("Part 2: {:?}", p2);
    //
    assert_eq!(p1, 516);
    assert_eq!(p2, 561);
}

fn parse_input(raw_input: &str) -> Vec<Vec<i32>>{
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in raw_input.lines(){
       let levels = line.split(' ').collect::<Vec<&str>>();
        let levels_int = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        reports.push(levels_int);
   }
    reports
}

fn solve_part_1(reports: &Vec<Vec<i32>>) -> usize{
    reports.iter().map(|a| is_safe(a) as usize).sum()
}

fn is_safe(levels: &Vec<i32>) -> bool{
    let diff = levels.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    let max_diff = diff.iter().map(|&a| a.abs()).max().unwrap();
    let are_all_positive = diff.iter().all(|&a| a > 0);
    let are_all_negative = diff.iter().all(|&a| a < 0);
    return (max_diff <= 3) && (are_all_negative || are_all_positive);
}

fn solve_part_2(reports: &Vec<Vec<i32>>) -> usize{
    reports.iter().map(|a| is_safe_with_tolerance(a) as usize).sum()
}

fn is_safe_with_tolerance(levels: &Vec<i32>) -> bool {
    if is_safe(levels) {
        return true;
    }
    for i in 0..(levels.len()) {
        let mut levels_copy = levels.clone();
        levels_copy.remove(i);
        if is_safe(&levels_copy) {
            return true;
        }
    }
    return false;
}