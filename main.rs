use std::fs::File;
use std::io::{BufRead, BufReader};
fn file_reader(file_path:&str) -> BufReader<File> {

    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}

fn main() {
    
    let reader = file_reader("C:\\Users\\Yegor\\Advent\\task1\\input.txt");

    let mut max_cal:Vec<i32> = Vec::new();
    let mut vec_of_cal:Vec<i32> = Vec::new();



    for line in reader.lines(){
        let line = line.unwrap();
        if let Ok(result) = line.parse::<i32>() {
            vec_of_cal.push(result);
        } else {
            let their_values: i32 = vec_of_cal.iter().sum();
            max_cal.push(their_values);
            vec_of_cal = Vec::new();
        }
        
    }
    max_cal.sort();


    print!("The result is {:?}", max_cal.iter().rev().take(3).sum::<i32>());
}
