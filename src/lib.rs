pub mod coord {
    use std::ops::{Add, Sub, Mul};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Coord {
        pub x: i64,
        pub y: i64,
    }

    impl Coord {
        pub fn new(x: i64, y: i64) -> Self {
            Coord {x, y}
        }

        pub fn new_u(x: usize, y: usize) -> Self {
            Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap()}
        }

        pub fn up() -> Coord {
            Coord::new(0, -1)
        }

        pub fn right() -> Coord {
            Coord::new(1, 0)
        }

        pub fn down() -> Coord {
            Coord::new(0, 1)
        }

        pub fn left() -> Coord {
            Coord::new(-1, 0)
        }

        pub fn dir(d: Dir) -> Coord {
            match d {
                Dir::Right => Self::right(),
                Dir::Down => Self::down(),
                Dir::Left => Self::left(),
                Dir::Up => Self::up(),
            }
        }

        pub fn x_u(&self) -> usize {
            self.x.try_into().unwrap()
        }

        pub fn y_u(&self) -> usize {
            self.y.try_into().unwrap()
        }

        pub fn manhattan(&self) -> usize {
            (self.x.abs() + self.y.abs()).try_into().unwrap()
        }
    }

    impl Add for Coord {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Coord::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for Coord {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Coord::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Mul<i64> for Coord {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Coord::new(self.x * rhs, self.y * rhs)
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    impl Dir {
        pub fn of_id(id: usize) -> Dir {
            match id {
                0 => Dir::Right,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Up,
                _ => panic!("Invalid direction id: {id}"),
            }
        }

        pub fn id(self) -> usize {
            match self {
                Dir::Right => 0,
                Dir::Down => 1,
                Dir::Left => 2,
                Dir::Up => 3,
            }
        }

        pub fn turnLeft(self) -> Dir {
            Self::of_id((self.id() + 3) % 4)
        }

        pub fn turnRight(self) -> Dir {
            Self::of_id((self.id() + 1) % 4)
        }
    }

    pub trait CoordMap<T> {
        fn at(&self, p: Coord) -> &T;
        fn contains(&self, p: Coord) -> bool;
        fn print(&self);
    }

    impl<T> CoordMap<T> for Vec<Vec<T>> where T: std::fmt::Debug + std::fmt::Display {
        fn at(&self, p: Coord) -> &T {
            let y: usize = p.y.try_into().unwrap();
            let x: usize = p.x.try_into().unwrap();
            &self[y][x]
        }

        fn contains(&self, p: Coord) -> bool {
            p.x >= 0 && p.y >= 0 && p.x < self[0].len().try_into().unwrap() && p.y < self.len().try_into().unwrap()
        }

        fn print(&self) {
            for row in self {
                row.iter().for_each(|x| print!("{x}"));
                println!("");
            }
        }
    }
}