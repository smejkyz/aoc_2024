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
pub struct Array2D {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

impl Array2D {
    // Constructor to create a new 2D array with default value
    pub fn new(rows: usize, cols: usize, default_value: i32) -> Self {
        let data = vec![vec![default_value; cols]; rows];
        Array2D { rows, cols, data }
    }

    pub fn get_rows(&self) -> usize{
        self.rows
    }

    pub fn get_row(&self, row: usize) -> Vec<i32>{
        self.data[row].clone()
    }

    pub fn get_column(&self, col: usize) -> Vec<i32>{
        self.data.iter().map(|row| row[col]).collect()
    }

    pub fn get_cols(&self) -> usize{
        self.cols
    }
    // Method to get a value at a specific position
    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row][col]
        // if row < self.rows && col < self.cols {
        //     Some(self.data[row][col])
        // } else {
        //     None
        // }
    }

    pub fn get_4_neighbours(&self, row: usize, col: usize) -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        if row > 0{
            // add upper
            result.push(self.get(row-1, col));
        }
        if row < self.rows - 1{
            // add below:
            result.push(self.get(row+1, col))
        }
        if col > 0{
            //add left
            result.push(self.get(row, col -1))
        }
        if col < self.cols - 1{
            result.push(self.get(row, col +1))
        }
        result
    }

    pub fn get_4_neighbours_coor(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut result: Vec<(usize, usize)> = Vec::new();
        if row > 0{
            // add upper
            result.push((row-1, col));
        }
        if row < self.rows - 1{
            // add below:
            result.push((row+1, col))
        }
        if col > 0{
            //add left
            result.push((row, col -1))
        }
        if col < self.cols - 1{
            result.push((row, col +1))
        }
        result
    }

    pub fn get_8_neighbours_coor(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut result = self.get_4_neighbours_coor(row, col);
        if row > 0{
            if col > 0 {
                result.push((row - 1, col - 1));
            }
            if col < self.cols - 1{
                result.push((row - 1, col + 1))
            }
        }
        if row < self.rows - 1{
            if col > 0 {
                result.push((row + 1, col - 1));
            }
            if col < self.cols - 1{
                result.push((row + 1, col + 1))
            }
        }

        result
    }


    // Method to set a value at a specific position
    pub fn set(&mut self, row: usize, col: usize, value: i32) -> Result<(), &'static str> {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub fn increase_by(&mut self, value: i32){
        for (i, j) in (0..self.rows).cartesian_product(0..self.cols){
            &self.set(i, j, self.get(i, j) + value);
        }
    }

    // Method to display the 2D array
    pub fn display(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }

    pub fn replace(&mut self, replaced_number: i32, replace_with: i32){
        for (i, j) in (0..self.rows).cartesian_product(0..self.cols){
            if self.get(i, j) == replaced_number{
                &self.set(i, j, replace_with);
            }
        }
    }
    pub fn count(&self, number_of_interest: i32) -> i32{
        let mut _count = 0;
        for (i, j) in (0..self.rows).cartesian_product(0..self.cols){
            if self.get(i, j) == number_of_interest{
                _count += 1;
            }
        }
        _count
    }
    pub fn sum(&self) -> i32{
        let mut _sum = 0;
        for row in &self.data{
            let _row_sum: i32 = row.iter().sum();
            _sum += _row_sum;
        }
        _sum
    }

    // Method to push a new row into the 2D array
    pub fn push(&mut self, new_row: Vec<i32>) -> Result<(), &'static str> {
        if new_row.len() == self.cols || self.rows == 0 {
            if self.rows == 0{
                self.cols = new_row.len();
            }
            self.data.push(new_row);
            self.rows += 1;
            Ok(())
        } else {
            Err("Length of the new row does not match the number of columns")
        }
    }
}


pub fn parse_2d_input(raw_input: &str) -> Array2D{
    const RADIX: u32 = 10;
    let mut array_2d = Array2D::new(0, 0, 0);
    for line in raw_input.lines(){
        let mut vector: Vec<i32> = Vec::new();
        for char in line.chars(){
            let digit = char.to_digit(10).unwrap_or(0) as i32;
            vector.push(digit);
        }
        array_2d.push(vector);
        //array_2d.display();
        //println!("solution");
    }

    array_2d
}