use std::fs;
use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("src/input");
    if !content.is_ok() {
        panic!("Couldn't read the file");
    }

    let content = content.unwrap();
    let numbers: Vec<i32> = content.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();

    for n in numbers.into_iter().combinations(3) {
        if n[0] + n[1] + n[2] == 2020 {
            println!("{}", n[0] * n[1] * n[2])
        }
    }
}
