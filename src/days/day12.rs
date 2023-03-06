use crate::utils::input;

pub fn run(extra: bool, test: bool) -> String {
    let lines = input::read_raw("12", test);

    format!(
        "{}",
        match extra {
            false => p1::run(lines),
            true => p2::run(lines)
        }
    )
}

mod entities {
    use std::cmp::Ordering;
    use crate::utils::helpers::clamp;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Point(usize, usize);

    impl Point {
        pub fn neighbours(&self, max_y: usize, max_x: usize) -> Vec<Point> {
            let mut neighbours =
                vec![
                    Self(clamp(self.0 as i32 - 1, 0, max_y as i32) as usize, self.1),
                    Self(clamp(self.0 + 1, 0, max_y), self.1),
                    Self(self.0, clamp(self.1 as i32 - 1, 0, max_x as i32) as usize),
                    Self(self.0, clamp(self.1 + 1, 0, max_x))
                ];

            neighbours.retain(|p| p.0 != self.0 && p.1 != self.1);
            neighbours
        }
    }

    #[derive(Eq)]
    pub struct Cell {
        pub value: u8,
        pub parent: Option<Point>,
        pub distance: Option<u32>
    }

    impl Ord for Cell {
        fn cmp(&self, other: &Self) -> Ordering {
            self.distance.cmp(&other.distance)
        }
    }

    impl PartialOrd for Cell {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Cell {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance
        }
    }

    pub struct Grid {
        grid: Vec<Vec<Cell>>,
        pub start: Point,
        pub target: Point,
        pub dim_y: usize,
        pub dim_x: usize
    }

    use std::{str::FromStr, string::ParseError};

    impl FromStr for Grid {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines = s.split('\n').collect::<Vec<&str>>();

            let dim_y = lines.len();
            let dim_x = lines[0].len();
            let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(dim_y);
            let mut start: Option<Point> = None;
            let mut target: Option<Point> = None;

            for (y_indx, line) in lines.iter().enumerate() {
                let cols = line.split("").filter(|e| !e.is_empty()).collect::<Vec<&str>>();
                let mut row: Vec<Cell> = Vec::with_capacity(dim_x);

                for (x_indx, &col) in cols.iter().enumerate() {
                    let val =
                        match col {
                            "S" => {
                                start = Some(Point(y_indx, x_indx));
                                0
                            },
                            "E" => {
                                target = Some(Point(y_indx, x_indx));
                                'z' as usize % 97
                            },
                            c => {
                                c.chars().next().unwrap() as usize % 97
                            }
                        };

                    row.push(Cell {
                        value: val as u8,
                        parent: None,
                        distance: None
                    });
                }

                grid.push(row);
            }

            Ok(Self {
                grid,
                start: start.unwrap(),
                target: target.unwrap(),
                dim_y,
                dim_x
            })
        }
    }

    impl Grid {
        pub fn move_candidates(&mut self, curr: Point) -> Vec<Point> {
            let neighbours = curr.neighbours(self.dim_y, self.dim_x);

            for &neighbour in neighbours.iter() {
                self.visit(&neighbour, &curr);
            }

            neighbours
        }

        pub fn visit(&mut self, neighbour: &Point, curr: &Point) {
            let curr_cell = self.get(curr);
            let curr_dist = curr_cell.distance;
            let curr_value = curr_cell.value;

            let mut neighbour_cell = self.get_mut(neighbour);

            if neighbour_cell.value - curr_value > 1 {
                return;
            }

            if neighbour_cell.distance.is_none()
                || (curr_dist.unwrap() + 1) < neighbour_cell.distance.unwrap() {
                    neighbour_cell.distance = Some(curr_dist.unwrap() + 1);
                    neighbour_cell.parent = Some(*curr);
                }
        }

        pub fn get(&self, at: &Point) -> &Cell {
            &self.grid[at.0][at.1]
        }

        pub fn get_mut(&mut self, at: &Point) -> &mut Cell {
            &mut self.grid[at.0][at.1]
        }
    }

}

mod p1 {
    use std::collections::VecDeque;
    use super::entities::Grid;

    pub fn run(lines: String) -> u32 {
        let mut grid: Grid = lines.parse().unwrap();

        let mut to_visit = VecDeque::from([grid.start]);

        while let Some(curr) = to_visit.pop_back() {
            dbg!(curr);
            for candidate in grid.move_candidates(curr) {
                to_visit.push_back(candidate);
            }
        }

        grid.get(&grid.target).distance.unwrap()
    }
}

mod p2 {
    pub fn run(lines: String) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_p1_test() {
        assert_eq!(run(false, true), "31");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(run(false, false), "361");
    }

    #[test]
    fn test_p2_test() {
        assert_eq!(run(true, true), "29");
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(run(false, true), "354");
    }
}