use std::fs;
use regex;

struct Policy {}

fn check1(min: &i32, max: &i32, char: &str, line: &str) -> bool{
    let mut counter = 0;
    let char = char.clone();

    for character in line.chars().map(|x| x.to_string()) {
        if character == char {
            counter += 1;
        }
    }

    if counter <= *max && counter >= *min {
       return true
    }

    return false

}

fn check2(position_1: &i32, position_2: &i32, char: &str, line: &str) -> bool{
    let mut counter = 0;

    if let Some(charr) = line.chars().nth(*position_1 as usize - 1) {
        if charr.to_string() == char {
            counter += 1;
        }
    }

    if let Some(charr) = line.chars().nth(*position_2 as usize - 1){
        if charr.to_string() == char {
            counter += 1;
        }
    }

    counter == 1
}

fn main() {
    let policy_regex = regex::Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let data = fs::read_to_string("input").unwrap();
    let data: Vec<&str>= data.split('\n').collect();
    let mut counter = 0;

    for line in &data{
        let cap = policy_regex.captures(line).unwrap();
        let min = &cap[1].parse::<i32>().unwrap();
        let max = &cap[2].parse::<i32>().unwrap();
        let char: &str = &cap[3];
        let password: &str = &cap[4];

        if check2(min, max, char, password) {
            counter += 1;
        }

    }

    println!("{}", counter);
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_check_1(){
        use crate::check1;
        assert_eq!(check1(&1, &2, "a", "aabc"), true);
        assert_eq!(check1(&1, &2, "a", "abc"), true);
        assert_eq!(check1(&1, &2, "a", "bc"), false);
    }

    #[test]
    fn test_check_2(){
        use crate::check2;
        assert_eq!(check2(&1, &2, "a", "aabc"), false);
        assert_eq!(check2(&1, &2, "a", "abbc"), true);
    }
}