use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::error::Error;
use itertools::Itertools;

const W_DAY: usize = 10;
const W_PART: usize = 10;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn measure_run<S: ?Sized, T, F: Fn(&S) -> T>(f: &F, input: &S) -> f64 {
    let start = std::time::SystemTime::now();
    _ = f(input);
    let duration = start.elapsed().unwrap();
    duration.as_secs_f64()
}

pub fn benchmark_run<S: ?Sized, T, F: Fn(&S) -> T>(f: F, input: &S) -> f64 {
    let first_run = measure_run(&f, input);
    let n = (1. / first_run) as i32;
    if n <= 1 || first_run < 0.000001 {
        return first_run;
    }
    let mut run_times = vec![];
    for _ in 0..n {
        run_times.push(measure_run(&f, input));
    }
    mean(&run_times)
}

pub fn print_header() {
    print!("{:<w$}", "day", w = W_DAY);
    print!("{:<w$}", "part 1", w = W_PART);
    print!("{:<w$}", "part 2", w = W_PART);
    println!();
    println!("{:-<w$}", "", w = W_DAY + W_PART * 2);
}

pub fn print_day(day: u8, p1: f64, p2: f64) {
    print!("{:<w$}", format!("day {:02}", day), w = W_DAY);

    let mut p1_dur = format!("{:.3}", p1 * 1000.).to_string();
    p1_dur = format!("{} ms", &p1_dur[..5]);
    print!("{:<w$}", p1_dur, w = W_PART);

    let mut p2_dur = format!("{:.3}", p2 * 1000.).to_string();
    p2_dur = format!("{} ms", &p2_dur[..5]);
    println!("{:<w$}", p2_dur, w = W_PART);
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Array2D<T:> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug + std::str::FromStr + std::cmp::PartialEq  + ToString> Array2D<T> {
    // Constructor to create a new 2D array with default value
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        let data = vec![vec![default_value.clone(); cols]; rows];
        Array2D { rows, cols, data }
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_row(&self, row: usize) -> Vec<T> {
        self.data[row].clone()
    }

    pub fn get_column(&self, col: usize) -> Vec<T> {
        self.data.iter().map(|row| row[col].clone()).collect()
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }
    // Method to get a value at a specific position
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col].clone()
        // if row < self.rows && col < self.cols {
        //     Some(self.data[row][col])
        // } else {
        //     None
        // }
    }

    pub fn get_4_neighbours(&self, row: usize, col: usize) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        if row > 0 {
            // add upper
            result.push(self.get(row - 1, col));
        }
        if row < self.rows - 1 {
            // add below:
            result.push(self.get(row + 1, col))
        }
        if col > 0 {
            //add left
            result.push(self.get(row, col - 1))
        }
        if col < self.cols - 1 {
            result.push(self.get(row, col + 1))
        }
        result
    }

    pub fn get_4_neighbours_coor(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if row > 0 {
            // add upper
            result.push((row - 1, col));
        }
        if row < self.rows - 1 {
            // add below:
            result.push((row + 1, col))
        }
        if col > 0 {
            //add left
            result.push((row, col - 1))
        }
        if col < self.cols - 1 {
            result.push((row, col + 1))
        }
        result
    }

    pub fn get_8_neighbours_coor(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = self.get_4_neighbours_coor(row, col);
        if row > 0 {
            if col > 0 {
                result.push((row - 1, col - 1));
            }
            if col < self.cols - 1 {
                result.push((row - 1, col + 1))
            }
        }
        if row < self.rows - 1 {
            if col > 0 {
                result.push((row + 1, col - 1));
            }
            if col < self.cols - 1 {
                result.push((row + 1, col + 1))
            }
        }

        result
    }


    // Method to set a value at a specific position
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub fn where_value(& self, value: T) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (i, j) in (0..self.rows).cartesian_product(0..self.cols) {
            if self.get(i, j) == value {
                result.push((i, j));
            }
        }
        result
    }
    pub fn is_on_boundary(& self, index: (usize, usize)) -> bool {
        let (i, j) = index;
        i == 0 || j == 0 || i == self.rows - 1 || j == self.cols - 1


    }
    // pub fn increase_by(&mut self, value: i32) {
    //     for (i, j) in (0..self.rows).cartesian_product(0..self.cols) {
    //         &self.set(i, j, self.get(i, j) + value);
    //     }
    // }

    // Method to display the 2D array
    pub fn display(&self) {
        // for row in &self.data {
        //     println!("\n{:?}", row);
        // }
        let mut buffer: String = "".to_string();
        for row in &self.data {
            buffer += &row.iter().map(|el| el.to_string()).collect::<Vec<String>>().join("");
            buffer += "\n";
        }
        println!("{}", buffer);

    }

    // pub fn replace(&mut self, replaced_number: i32, replace_with: i32) {
    //     for (i, j) in (0..self.rows).cartesian_product(0..self.cols) {
    //         if self.get(i, j) == replaced_number {
    //             &self.set(i, j, replace_with);
    //         }
    //     }
    // }
    // pub fn count(&self, number_of_interest: i32) -> i32 {
    //     let mut _count = 0;
    //     for (i, j) in (0..self.rows).cartesian_product(0..self.cols) {
    //         if self.get(i, j) == number_of_interest {
    //             _count += 1;
    //         }
    //     }
    //     _count
    // }
    // pub fn sum(&self) -> i32 {
    //     let mut _sum = 0;
    //     for row in &self.data {
    //         let _row_sum: i32 = row.iter().sum();
    //         _sum += _row_sum;
    //     }
    //     _sum
    // }

    // Method to push a new row into the 2D array
    pub fn push(&mut self, new_row: Vec<T>) -> Result<(), &'static str> {
        if new_row.len() == self.cols || self.rows == 0 {
            if self.rows == 0 {
                self.cols = new_row.len();
            }
            self.data.push(new_row);
            self.rows += 1;
            Ok(())
        } else {
            Err("Length of the new row does not match the number of columns")
        }
    }
    pub fn parse_2d_input(raw_input: &str) -> Result<Self, T::Err> {
        let data: Vec<Vec<T>> = raw_input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|item| item.to_string().parse::<T>())
                    .collect::<Result<Vec<T>, T::Err>>() // Collect and handle parsing errors
            })
            .collect::<Result<Vec<Vec<T>>, T::Err>>()?;

        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        Ok(Array2D { rows, cols, data })
    }

}

pub fn multiple_bfs(
    start: (usize, usize),
    goals: &Vec<(usize, usize)>,
    grid: &Array2D<i32>
) -> Vec<Vec<(usize, usize)>>{

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);

    let mut predecessors: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    predecessors.insert(start, None);

    let mut open: Vec<(usize, usize)> = Vec::new();
    open.push(start);
    let mut left_to_visit = goals.clone();
    let mut currect = start.clone();
    while open.len() > 0{
        currect = open.pop().unwrap();
        if left_to_visit.contains(&currect){
            let index = left_to_visit.iter().position(|x| *x == currect).unwrap();
            left_to_visit.remove(index);
        }
        if left_to_visit.len() == 0{
            break;
        }

        for adjacent in grid.get_4_neighbours_coor(currect.0, currect.1){
            if !visited.contains(&adjacent){
                visited.insert(adjacent);
                open.push(adjacent);
                predecessors.insert(adjacent, Some(currect));
            }
        }
    }
    goals.iter().map(|goal| reconstruct_path(*goal, &predecessors)).collect()
}

pub fn multiple_bfs_with_one_step(
    start: (usize, usize),
    goals: &Vec<(usize, usize)>,
    grid: &Array2D<i32>
) -> Vec<Vec<(usize, usize)>>{

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);

    let mut predecessors: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    predecessors.insert(start, None);

    let mut open: Vec<(usize, usize)> = Vec::new();
    open.push(start);
    let mut left_to_visit = goals.clone();
    let mut currect = start.clone();
    while open.len() > 0{
        //println!("open states: {:?}", open);
        //println!("left_to_visit: {:?}", left_to_visit);
        currect = open.pop().unwrap();
        if left_to_visit.contains(&currect){
            let index = left_to_visit.iter().position(|x| *x == currect).unwrap();
            left_to_visit.remove(index);
            //println!("visited goal: {:?}", currect);
        }
        if left_to_visit.len() == 0{
            break;
        }

        for adjacent in grid.get_4_neighbours_coor(currect.0, currect.1){
            let diff_value = grid.get(adjacent.0, adjacent.1) - grid.get(currect.0, currect.1);

            if !visited.contains(&adjacent) && diff_value == 1{
                visited.insert(adjacent);
                open.push(adjacent);
                predecessors.insert(adjacent, Some(currect));
            }
        }
    }
    let mut resulting_paths = Vec::new();
    for goal in goals{
        if !left_to_visit.contains(&goal){
            resulting_paths.push(reconstruct_path(*goal, &predecessors));
        }else {
            //println!("Was not able to find path to {:?}", goal);
            resulting_paths.push(vec![]);
        }
    }
    resulting_paths
    //goals.iter().map(|goal| reconstruct_path(*goal, &predecessors)).collect()
}

pub fn find_all_paths_bfs_one_step(
    start: (usize, usize),
    goal: (usize, usize),
    grid: &Array2D<i32>
) -> Vec<Vec<(usize, usize)>>{
    let mut all_paths: Vec<Vec<(usize, usize)>> = Vec::new();
    // quee to store the paths
    let mut q = VecDeque::new();

    let mut path = Vec::new();
    path.push(start);

    q.push_back(path.clone());

    while !q.is_empty(){
        path = q.pop_front().unwrap();
        let last = path[path.len() - 1];
        // if the last vertex is desired destination, print the path
        if last == goal{
            //println!("path found: {:?}", path);
            all_paths.push(path.clone());
        }

        //Traverse to all the nodes connected to
        //current vertex and push new path to queue
        for adjacent in grid.get_4_neighbours_coor(last.0, last.1){
            let diff_value = grid.get(adjacent.0, adjacent.1) - grid.get(last.0, last.1);
            if !path.contains(&adjacent) && diff_value == 1{
                let mut new_path = path.clone();
                new_path.push(adjacent);
                q.push_back(new_path);
            }
        }
    }
    all_paths
}


pub fn reconstruct_path(
    goal: (usize, usize),
    predecessors: &HashMap<(usize, usize), Option<(usize, usize)>>
) -> Vec<(usize, usize)>{
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = goal;
    loop {
        path.push(current);
        if let Some(value) = predecessors.get(&current) {
            match value {
                Some((x, y)) => current = (x.clone(), y.clone()),
                None => break,
            }
        }
    }
    path.reverse();
    path
}



// pub fn parse_2d_input(raw_input: &str) -> Array2D{
//     const RADIX: u32 = 10;
//     let mut array_2d = Array2D::new(0, 0, 0);
//     for line in raw_input.lines(){
//         let mut vector: Vec<i32> = Vec::new();
//         for char in line.chars(){
//             let digit = char.to_digit(10).unwrap_or(0) as i32;
//             vector.push(digit);
//         }
//         array_2d.push(vector);
//         //array_2d.display();
//         //println!("solution");
//     }
//
//     array_2d
// }

fn elementwise_subtraction(vec_a: Vec<i32>, vec_b: Vec<i32>) -> Vec<i32> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a - b).collect()
}