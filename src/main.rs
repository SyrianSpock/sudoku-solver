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
        println!("The grid is:");

        for i in 0..9 {
            println!("{:?}", self.value[i]);
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
}
