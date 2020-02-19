#![feature(nll)]
use std::fmt;

#[derive(Copy, Clone)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(row: {}, column: {})", self.row, self.column)
    }
}

fn possible(grid: Grid, position: Coordinate, number: u8) -> bool {
    // Check row
    for i in 0..9 {
        if grid.value[position.column as usize][i as usize] == number {
            return false;
        }
    }
    // Check column
    for i in 0..9 {
        if grid.value[i as usize][position.row as usize] == number {
            return false;
        }
    }
    // Check box
    let row0: usize = ((position.row / 3) as usize * 3) as usize;
    let col0: usize = ((position.column / 3) as usize * 3) as usize;
    for i in 0..3 {
        for j in 0..3 {
            if grid.value[row0 + i][col0 + j] == number {
                return false;
            }
        }
    }
    return true;
}

#[derive(Copy, Clone)]
struct Grid {
    value: [[u8; 9]; 9],
}
impl Grid {
    pub fn new(
        l1: [u8; 9],
        l2: [u8; 9],
        l3: [u8; 9],
        l4: [u8; 9],
        l5: [u8; 9],
        l6: [u8; 9],
        l7: [u8; 9],
        l8: [u8; 9],
        l9: [u8; 9],
    ) -> Grid {
        let mut g = Grid {
            value: { [[0; 9]; 9] },
        };
        g.value[0] = l1;
        g.value[1] = l2;
        g.value[2] = l3;
        g.value[3] = l4;
        g.value[4] = l5;
        g.value[5] = l6;
        g.value[6] = l7;
        g.value[7] = l8;
        g.value[8] = l9;
        return g;
    }

    fn display(&self) {
        self.show(Coordinate {
            row: 10,
            column: 10,
        });
    }

    fn show(&self, position: Coordinate) {
        println!("----------------------------------------");
        for row in 0..9 {
            let mut line = "".to_string();
            for col in 0..9 {
                if col == 0 {
                    line.push_str("| ");
                }

                if position.row == row && position.column == col {
                    line.push_str("*");
                } else {
                    line.push_str(Box::leak(
                        format!("{}", self.value[row as usize][col as usize] as usize)
                            .into_boxed_str(),
                    ));
                }

                line.push_str(" |");
                if (col % 3) == 2 {
                    line.push_str("|");
                }
                if col != 8 {
                    line.push_str(" ");
                }
            }
            println!("{}", line);
            if (row % 3) == 2 {
                println!("----------------------------------------");
            }
        }
    }
}

fn some_input() -> Grid {
    return Grid::new(
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    );
}

// solution of the above grid
fn some_solution() -> Grid {
    return Grid::new(
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    );
}

fn solve(grid: &mut Grid) -> Grid {
    let mut g = grid.clone();
    for row in 0..9 {
        for col in 0..9 {
            if g.value[col][row] == 0 {
                for n in 1..10 {
                    if possible(
                        g,
                        Coordinate {
                            row: row,
                            column: col,
                        },
                        n,
                    ) {
                        g.value[col][row] = n;
                    }
                }
            }
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_finds_impossible_numbers() {
        assert!(!possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            9
        ));
        assert!(!possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            7
        ));
        assert!(!possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            5
        ));
        assert!(!possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            3
        ));
        assert!(!possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            1
        ));
    }

    #[test]
    fn possible_finds_possible_numbers() {
        assert!(possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            2
        ));
        assert!(possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            6
        ));
        assert!(possible(
            some_input(),
            Coordinate { row: 0, column: 3 }.clone(),
            8
        ));
    }

    #[test]
    fn solves_sudoku_grid() {
        assert_eq!(0, 0);
    }
}

fn main() {
    let input = some_input();
    input.display();
    let n = 9;
    let c = Coordinate { row: 0, column: 3 };
    println!("The grid is shown {:?} position as *:", c);
    input.show(c);
    println!(
        "{} at {:?} is possible? {}",
        n,
        &c,
        possible(input.clone(), c.clone(), n)
    );
    println!("The computed solution grid is:");
    solve(&mut input.clone()).display();
    println!("The actual solution grid is:");
    some_solution().display();
}
