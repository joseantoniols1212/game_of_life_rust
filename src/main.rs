use std::{fmt, thread, time::Duration};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    Alive, Dead
}

#[derive(Clone)]
struct Board {
    cols: i32,
    rows: i32,
    cells: Vec<State>
}

impl Board {

    fn new(cols: i32, rows: i32, cells: Vec<State>) -> Self {
        assert_eq!(cols*rows, cells.len() as i32);
        Board { cols, rows, cells}
    }

    fn transition(&self) -> Self {
        let cells = self.cells
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_updated_state(i))
            .collect::<Vec<_>>();
        Board::new(self.cols, self.rows, cells)
    }

    fn indexate(&self, index: usize, offset: (i32, i32)) -> Option<usize> {
        let index = index as i32;     
        let row = index/self.cols + offset.0;
        let col = index%self.cols + offset.1;
        if row < 0 || col < 0 || row >= self.rows || col >= self.cols {
            return None;
        }
        return Some( (row*self.cols + col) as usize );
    }

    fn get_updated_state(&self, index: usize) -> State {
        let neightbours_offset = [(1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1, 0), (-1, 1), (0, 1)];
        let alive_neightbours = neightbours_offset.iter()
            .filter_map(|&x| self.indexate(index, x))
            .fold(0, |acc, x| acc + if self.cells[x] == State::Alive {1} else {0});
        match self.cells[index] {
            State::Alive if (alive_neightbours == 2 || alive_neightbours == 3) => return State::Alive,
            State::Dead if alive_neightbours == 3 => return State::Alive,
            _ => return State::Dead
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.cols as usize) {
            for &cell in line {
                let symbol = if cell == State::Alive {"█"} else {"\x1b[93m█\x1b[0m"};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let cells = vec![
        State::Dead, State::Alive, State::Dead,
        State::Dead, State::Alive, State::Dead,
        State::Dead, State::Alive, State::Dead
    ];
    let mut board = Board::new(3, 3, cells);
    loop {
        print!("{}", board);
        board = board.transition();
        thread::sleep(Duration::from_millis(500));
        print!("\x1B[2J");
    }
}
