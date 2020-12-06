use std::fs;

struct Board<'a> {
    characters: &'a str,
    width: usize,
    height: usize,
}

impl<'a> Board<'a> {
    fn new(data: &'a str) -> Self {
        let size = Board::get_size(data);
        Board { characters: data, width: size.0, height: size.1}
    }

    fn get_size(data: &str) -> (usize, usize) {
        let width = data.find('\n').unwrap();
        let height = data.split('\n').count();

        (width, height)
    }

    fn get_at(&self, x: usize, y: usize) -> char {
        self.characters.chars().filter(|x| *x != '\n').nth(x % self.width + y * self.width).unwrap()
    }

    fn solve(&self, slope: (usize, usize)) -> i64 {
        let mut counter = 0;
        let (dx, dy) = slope;

        for y in 0..self.height {
            if y % dy != 0 {
                continue
            }

            let char = self.get_at((y / dy) * dx, y);
            if char == '#' {
                counter += 1;
            }
        }

        counter
    }
}


fn main() {
    let data = fs::read_to_string("input").unwrap();
    let board = &Board::new(data.as_str());
    let slopes = vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes.into_iter().map(|slope| board.solve(slope)).fold(1, |x, y| x * y);
    println!("{:?}", result);

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_board() {
        use crate::Board;
        let value = Board::get_size("abc\nabc");
        assert_eq!(value.0, 3);
        assert_eq!(value.1, 2);
    }

    #[test]
    fn test_at() {
        use crate::Board;

        let board = Board::new("abc\nabc");
        assert_eq!(board.get_at(0, 0), 'a');
        assert_eq!(board.get_at(1, 0), 'b');
        assert_eq!(board.get_at(3, 0), 'a');
    }
}