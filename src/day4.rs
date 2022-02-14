use std::fmt::{Display, Formatter};
use std::ptr::null;

pub fn part1(lines: &[String]) -> String {
    let nbrs = lines[0].split(",").map(|x| x.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = lines[1..]
        .split(|l| l.is_empty())
        .filter(|x| !x.is_empty())
        .map(Board::new)
        .collect();

    for n in nbrs {
        println!("======================");
        println!("Calling number {}!", n);
        println!();
        for b in &mut boards {
            let winner = b.mark(n);
            println!("{}", b);
            if winner {
                println!("we have a wiener!");
                return format!("{}", b.result() * n);
            }
        }
    }

    format!("no one wins :(")
}

pub fn part2(lines: &[String]) -> String {
    let nbrs = lines[0].split(",").map(|x| x.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = lines[1..]
        .split(|l| l.is_empty())
        .filter(|x| !x.is_empty())
        .map(Board::new)
        .collect();

    let mut winners = 0;
    let board_cnt = boards.len();

    for n in nbrs {
        println!("======================");
        println!("Calling number {}!", n);
        println!();
        for b in &mut boards {
            if b.bingo {
                continue;
            }

            b.mark(n);
            println!("{}", b);
            if b.bingo {
                winners += 1;
                println!("we have {} wieners!", winners);
            }

            if winners == board_cnt {
                return format!("{}", b.result() * n);
            }
        }
    }

    format!("no one wins :(")
}

struct Board {
    cells: Vec<Cell>,
    bingo: bool,
}

impl Board {
    pub fn new(lines: &[String]) -> Board {
        let cells = lines
            .iter()
            .flat_map(|l| l.split(" ").filter(|x| !x.is_empty()))
            .map(|x| Cell {
                val: x.parse::<u32>().unwrap(),
                marked: false,
            })
            .collect();

        Board {
            cells,
            bingo: false,
        }
    }

    pub fn mark(&mut self, v: u32) -> bool {
        for i in 0..self.cells.len() {
            let c = &mut self.cells[i];
            if c.val == v {
                c.marked = true;
                self.bingo = self.bingo || self.check_col(i) || self.check_row(i);
            }
        }

        self.bingo
    }

    fn check_row(&self, i: usize) -> bool {
        let row = i / 5;
        for col in 0..5 {
            if !self.cells[row * 5 + col].marked {
                return false;
            }
        }

        true
    }

    fn check_col(&self, i: usize) -> bool {
        let col = i % 5;
        for row in 0..5 {
            if !self.cells[row * 5 + col].marked {
                return false;
            }
        }

        true
    }

    pub fn result(&self) -> u32 {
        self.cells.iter().filter(|c| !c.marked).map(|c| c.val).sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for chunk in self.cells.chunks(5) {
            for c in chunk {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

struct Cell {
    pub val: u32,
    pub marked: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "[{:2}] ", self.val)
        } else {
            write!(f, " {:2}  ", self.val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ];

    #[test]
    fn test_part1() {
        let data: Vec<_> = DATA.iter().map(|x| x.to_string()).collect();
        assert_eq!(part1(&data), "4512");
    }
    #[test]
    fn test_part2() {
        let data: Vec<_> = DATA.iter().map(|x| x.to_string()).collect();
        assert_eq!(&part2(&data), "1924");
    }
}
