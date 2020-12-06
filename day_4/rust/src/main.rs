use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Error;
use std::num::ParseIntError;
use std::option::Iter;

use regex::Regex;

#[derive(Debug)]
struct ID<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
}

impl<'a> ID<'a> {
    fn validate_byr(&self) -> bool {
        match self.byr.parse::<i64>() {
            Ok(birth_year) => { birth_year <= 2002 && birth_year >= 1920 }
            Err(_) => false,
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr.parse::<i64>() {
            Ok(issue_year) => { issue_year <= 2020 && issue_year >= 2010 }
            Err(_) => false,
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr.parse::<i64>() {
            Ok(expiration_year) => { expiration_year <= 2030 && expiration_year >= 2020 }
            Err(_) => false,
        }
    }

    fn validate_hgt(&self) -> bool {
        let reg = regex::Regex::new(r"(\d+)(cm|in)").unwrap();
        let captures = reg.captures(self.hgt);
        match captures {
            Some(capture) => {
                let ending = capture.get(2).unwrap().as_str();
                let value = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                if ending == "cm" {
                    return value >= 150 && value <= 193
                }
                else {
                    return value >= 59 && value <= 76
                }
            },
            _ => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        let pat = regex::Regex::new(r"#[0-9a-f]{6}").unwrap();
        pat.is_match(self.hcl)
    }

    fn validate_pid(&self) -> bool {
        match self.pid.parse::<i64>() {
            Ok(pid) => {
                self.pid.len() == 9
            }
            Err(_) => false
        }
    }

    fn validate_ecl(&self) -> bool {
        let allowed_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        allowed_eye_colors.into_iter().any(|x| x == self.ecl)
    }

    fn validate(&self) -> bool {
        let functions: Vec<fn(&Self) -> bool> = vec![ID::validate_byr, ID::validate_iyr, ID::validate_eyr, ID::validate_hgt, ID::validate_hcl, ID::validate_pid, ID::validate_ecl];
        functions.into_iter().map(|fun| fun(&self)).all(|x| x == true)
    }
}

fn load_id(data: &str) -> Option<ID> {
    let mandatory_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut map: HashMap<&str, &str> = HashMap::new();
    data.split('\n').flat_map(|x| x.split(' ')).for_each(|word| {
        let parts: Vec<&str> = word.split(':').collect();
        map.insert(parts.get(0).unwrap(), parts.get(1).unwrap());
    });

    let keys: HashSet<&str> = map.keys().map(|x| *x).collect();
    let mandat: HashSet<&str> = mandatory_keys.iter().map(|x| *x).collect();
    if keys != mandat {
        return None;
    }

    println!("{:?}", &map);
    Some(ID {
        byr: map.get("byr").unwrap(),
        iyr: map.get("iyr").unwrap(),
        eyr: map.get("eyr").unwrap(),
        hcl: map.get("hcl").unwrap(),
        hgt: map.get("hgt").unwrap(),
        ecl: map.get("ecl").unwrap(),
        pid: map.get("pid").unwrap(),
    })
}

fn main() {
    let adsf = "000123".parse::<i32>().unwrap();

    let data = fs::read_to_string("input").unwrap();
    let wut: Vec<Option<ID>> = data.split("\n\n").map(load_id).collect();
    let some_id = wut.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).filter(|x| x.validate()).count();

    println!("{}", some_id);
}


#[cfg(test)]
mod tests{
    #[test]
    fn validate_byr(){
        use crate::ID;
        let mut id = ID{
            byr: "1950",
            pid: "007547618",
            eyr: "2020",
            hcl: "#af1f9f",
            hgt: "180cm",
            ecl: "brn",
            iyr: "2010",
        };

        assert_eq!(id.validate_byr(), true);
        assert_eq!(id.validate_pid(), true);
        assert_eq!(id.validate_eyr(), true);
        assert_eq!(id.validate_hcl(), true);
        assert_eq!(id.validate_hgt(), true);
        assert_eq!(id.validate_iyr(), true);

        assert_eq!(id.validate(), true);

    }
}