#![feature(nll)]
use std::fmt;

#[derive(Copy, Clone)]
struct Coordinate {
    row: u8,
    column: u8,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(row: {}, column: {})", self.row, self.column)
    }
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
        for i in 0..9 {
            let mut line = "".to_string();
            for j in 0..9 {
                if j == 0 {
                    line.push_str("| ");
                }

                if position.row == i && position.column == j {
                    line.push_str("*");
                } else {
                    line.push_str(Box::leak(
                        format!("{}", self.value[i as usize][j as usize] as usize).into_boxed_str(),
                    ));
                }

                line.push_str(" |");
                if j != 8 {
                    line.push_str(" ");
                }
            }
            println!("{}", line);
        }
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
}
