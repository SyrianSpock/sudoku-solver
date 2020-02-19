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
    for i in 0..9 {
        if grid.value[position.column as usize][i as usize] == number {
            return false;
        }
    }
    for i in 0..9 {
        if grid.value[i as usize][position.row as usize] == number {
            return false;
        }
    }
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
        println!("-------------------------------------");
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
                if col != 8 {
                    line.push_str(" ");
                }
            }
            println!("{}", line);
        }
        println!("-------------------------------------");
    }
}

fn main() {
    let input: Grid = Grid::new(
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

    input.display();
    let n = 9;
    let c = Coordinate { row: 0, column: 3 };
    println!("The grid is shown {:?} position as *:", c);
    input.show(c);
    println!(
        "{} at {:?} is possible? {}",
        n,
        &c,
        possible(input, c.clone(), n)
    );
}
