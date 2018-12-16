use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct Universe {
    width : u32,
    height: u32,
    cells: Vec<Cell>,
}

#[allow(dead_code)]
#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> usize {
        iproduct!([self.height - 1, 0, 1].iter(), [self.width-1, 0, 1].iter())
            .filter(|(x, y)| **x != 0 || **y != 0)
            .map(|(x, y)| self.cells[self.get_index((x+row) % self.height, (y+column) % self.width)] as usize)
            .fold(0, |acc, x| acc+x)
    }

    //Tick once
    pub fn tick(&mut self) {
        let next = iproduct!(0..self.height, 0..self.width)
            .map(|(row, col)| {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let nbr_cnt = self.live_neighbour_count(row, col);
                match (cell, nbr_cnt) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                }
            }).collect();
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
        }       
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Cell::*;
    #[test]
    fn should_fetch_correct_index() {
        let universe = get_universe();
        assert_eq!(universe.get_index(1, 1), 6);

    }

    #[test]
    fn should_count_correct_neighbrs() {
        let univ = get_universe();
        assert_eq!(univ.live_neighbour_count(0, 0), 2);
        assert_eq!(univ.live_neighbour_count(0, 1), 2);
        assert_eq!(univ.live_neighbour_count(0, 2), 1);
        assert_eq!(univ.live_neighbour_count(0, 3), 2);
        assert_eq!(univ.live_neighbour_count(0, 4), 2);
        assert_eq!(univ.live_neighbour_count(1, 0), 0);
        assert_eq!(univ.live_neighbour_count(1, 1), 3);
        assert_eq!(univ.live_neighbour_count(1, 2), 1);
        assert_eq!(univ.live_neighbour_count(1, 3), 2);
        assert_eq!(univ.live_neighbour_count(1, 4), 1);
        assert_eq!(univ.live_neighbour_count(2, 0), 1);
        assert_eq!(univ.live_neighbour_count(2, 1), 4);
        assert_eq!(univ.live_neighbour_count(2, 2), 2);
        assert_eq!(univ.live_neighbour_count(2, 3), 3);
        assert_eq!(univ.live_neighbour_count(2, 4), 1);
        assert_eq!(univ.live_neighbour_count(3, 0), 1);
        assert_eq!(univ.live_neighbour_count(3, 1), 2);
        assert_eq!(univ.live_neighbour_count(3, 2), 1);
        assert_eq!(univ.live_neighbour_count(3, 3), 3);
        assert_eq!(univ.live_neighbour_count(3, 4), 1);
        assert_eq!(univ.live_neighbour_count(4, 0), 1);
        assert_eq!(univ.live_neighbour_count(4, 1), 1);
        assert_eq!(univ.live_neighbour_count(4, 2), 1);
        assert_eq!(univ.live_neighbour_count(4, 3), 2);
        assert_eq!(univ.live_neighbour_count(4, 4), 0);
    }

    #[test]
    fn should_get_correct_next_tick() {
        let mut univ = get_universe();
        univ.tick();
        assert_eq!(univ.cells, vec![
                Dead,  Dead,  Dead,  Dead,  Dead,
                Dead,  Alive, Dead,  Dead,  Dead,
                Dead,  Dead,  Alive, Alive, Dead,
                Dead,  Dead,  Dead,  Alive, Dead,
                Dead,  Dead,  Dead,  Dead,  Dead,
        ]);
    }

    fn get_universe() -> Universe {
        Universe {
            width: 5,
            height: 5,
            cells: vec![
                Dead,  Dead,  Dead,  Dead,  Dead,
                Alive, Dead,  Alive, Dead,  Dead,
                Dead,  Dead,  Alive, Dead,  Dead,
                Dead,  Dead,  Alive, Dead,  Dead,
                Dead,  Dead,  Dead,  Dead,  Alive,
            ]
        }
    }
}