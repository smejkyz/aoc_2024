use std::fs;
use ahash::{HashMap, HashMapExt};
use crate::utils::{Array2D};

pub fn solve(){
    let input_path = "/Users/tomas/RustroverProjects/aoc_2024/src/inputs/day05.txt";
    let raw_input = fs::read_to_string(input_path).unwrap();
    let (page_rules, page_to_produce) = parse_input(&raw_input);
    let p1 = solve_p1(&page_rules, &page_to_produce);
    println!("p1: {}", p1);
    let p2 = solve_p2(&page_rules, &page_to_produce);
    println!("p2: {}", p2);
    assert_eq!(p1, 4924);
    assert_eq!(p2, 6085);
}

fn parse_input(raw_input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>){
    let mut page_rules: Vec<(i32, i32)> = Vec::new();
    let mut page_to_produce: Vec<Vec<i32>> = Vec::new();
    for line in raw_input.lines(){
        if line == ""{
            continue;
        }
        if line.contains("|"){
            let splited: Vec<&str> = line.split("|").collect();
            let left_value: i32 = splited[0].parse().unwrap();
            let right_value: i32 = splited[1].parse().unwrap();
            page_rules.push((left_value,right_value));
        }
        if line.contains(","){
            let splited: Vec<i32> = line
                .split(",")
                .filter_map(|el| el.parse::<i32>().ok())
                .collect();
            page_to_produce.push(splited.clone());
        }
    }
    println!("page rules: {:?}", page_rules);
    println!("page_to_produce: {:?}", page_to_produce);
    (page_rules, page_to_produce)
}


fn solve_p1(page_rules: &Vec<(i32, i32)>, pages_to_produce: &Vec<Vec<i32>>) -> i32{
    let mut score = 0;
    for page in pages_to_produce {
        if page_is_in_correct_order(page, page_rules) {
            let add_to_score = page[page.len() / 2];
            println!("page {:?} is fine, adding score {}", page, add_to_score);
            score += add_to_score;
        }else {
            println!("page {:?} is not fine, adding score 0", page);
        }
    }
    score
}
fn page_is_in_correct_order(page: &Vec<i32>, page_rules: &Vec<(i32, i32)>) -> bool {
    //println!("page {:?}", page);
    for i in 0..page.len(){
        let base_element = page[i];
        for j in i+1..page.len(){
            let other_element = page[j];
            if !page_rules.contains(&(base_element, other_element)){
                //println!("page rules does not contain {:?}", (base_element, other_element));
                return false
            }
        }
    }
    true
}


fn solve_p2(page_rules: &Vec<(i32, i32)>, pages_to_produce: &Vec<Vec<i32>>) -> i32{
    let mut score = 0;
    for page in pages_to_produce {
        let correct_page = compute_correct_page(page, page_rules);
        if (correct_page == *page){
            println!("page {:?} was fine. Nothing was altered", page);
        }else{
            let add_to_score = correct_page[correct_page.len() / 2];
            println!("page {:?} was not fine. Correct page is {:?} Nothing was altered. Adding score {}", page, correct_page, add_to_score);
            score += add_to_score;

        }

    }
    score
}

fn compute_correct_page(page: &Vec<i32>, page_rules: &Vec<(i32, i32)>) -> Vec<i32> {
    //println!("page {:?}", page);
    let mut correct_page: Vec<i32> = page.clone();
    while !page_is_in_correct_order(&correct_page, page_rules){
        correct_page = single_switch(&correct_page, page_rules);
    }
    correct_page

}

fn single_switch(page: &Vec<i32>, page_rules: &Vec<(i32, i32)>) -> Vec<i32> {
    //println!("page {:?}", page);
    let mut correct_page: Vec<i32> = page.clone();
    for i in 0..page.len(){
        let base_element = page[i];
        for j in i+1..page.len(){
            let other_element = page[j];
            if !page_rules.contains(&(base_element, other_element)){
                //println!("page rules does not contain {:?}", (base_element, other_element));
                correct_page[i] = other_element;
                correct_page[j] = base_element;
                return correct_page;
            }
        }
    }
    correct_page
}