use std::fs;
use std::collections::HashMap;

struct Info<'a>{
    data: &'a str,
    xy: Option<(usize, usize)>,
}

impl<'a> Info<'a>{
    fn new(data: &str) -> Info{
        Info{
            data,
            xy: None
        }
    }

    fn solve(&self) -> Result<(usize, usize), String>{
        let mut row_max = 0;
        let mut row_min = 127;

        for instruction in self.data.chars().take(7){
            let dx = (row_min - row_max + 1 ) / 2;

            if instruction == 'F' {
                row_min -= dx;
            }
            else if instruction == 'B' {
                row_max += dx;
            }
        }

        if row_max != row_min {
            return Err(String::from("row_max and row_min incorrect"));
        }

        let mut seat_min = 0;
        let mut seat_max = 7;

        for instruction in self.data.chars().rev().take(3).collect::<Vec<char>>().into_iter().rev() {
            let dx = (seat_max - seat_min + 1) / 2;

            if instruction == 'L' {
                seat_max -= dx;
            }
            else if instruction == 'R' {
                seat_min += dx;
            }
        }

        if seat_max != seat_min {
            return Err(String::from("seat_max and seat_min incorrect"));
        }

        Ok((row_max, seat_max))
    }

    fn get_id(&mut self) -> usize {
        match self.xy {
            Some(xy) =>{
                xy.0 * 8 + xy.1
            },
            None => {
                let result = self.solve().expect("Nie udało się odpalić .solve()");
                self.xy = Some((result.0, result.1));
                result.0 * 8 + result.1
            }
        }
    }
}
fn main() {
    let data = fs::read_to_string("input").unwrap();
    let mut infos: Vec<Info> = data.split('\n').into_iter().map(|data| Info::new(data)).collect();
    let ids: Vec<usize> = infos.iter_mut().map(|info| info.get_id()).collect();
    println!("{:?}", ids);

    let mut blank = Vec::<usize>::new();

    for info in &mut infos {
        let id = info.get_id();

        if ids.iter().filter(|x| **x == id + 1).count() == 0 {
            println!("{}", id + 1);
        }

    }

    println!("{:?}", blank);
}
