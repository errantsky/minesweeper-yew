use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use rand::seq::IteratorRandom;
use rand::thread_rng;

pub enum CellData {
    /// In a minesweeper grid, each cell either has a mine, or an empty cell
    /// with the number of mines adjacent to it.
    Mine,
    MineNeighbor(usize),
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Flag {
    Mine,
    Empty,
}

pub struct Cell {
    /// A cell can be clicked on or flagged whether it has a mine or not
    pub x: usize,
    pub y: usize,
    pub is_mine: bool,
    pub is_clicked: bool,
    pub flag: Option<Flag>,
    pub data: CellData,
}

pub struct Grid {
    /// A grid has two dimensions, and a sequence of cells
    pub n_rows: usize,
    pub n_cols: usize,
    pub grid_vec: Vec<Cell>,
}

impl Grid {
    /// Returns a randomly generated minesweeper grid
    ///
    /// The number of mines equals the largest dimension. First, mined positions are
    /// randomly chosen from the grid. Then, a loop starts from the top left cell of
    /// the grid and fills out each `Cell`'s `data` field. If the `Cell` is not mined,
    /// All the eight neighbors of that are mined are counted and held in `MineNeighbor(usize)`.
    ///
    /// # Arguments
    /// * `n_rows` - Number of rows in the grid
    /// * `n_cols` - Number of columns in the grid
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        let mut grid_vec: Vec<Cell> = Vec::with_capacity((n_rows * n_cols) as usize);
        let mine_count = max(n_rows, n_cols);
        let mine_indices: HashSet<usize> = HashSet::from_iter(
            (0..(n_rows * n_cols)).choose_multiple(&mut thread_rng(), mine_count),
        );

        for idx in 0..(n_rows * n_cols) {
            let xy = (idx / n_cols, idx.rem_euclid(n_cols));
            if mine_indices.contains(&idx) {
                grid_vec.push(Cell {
                    x: xy.0,
                    y: xy.1,
                    is_mine: true,
                    is_clicked: false,
                    flag: None,
                    data: CellData::Mine,
                });
            } else {
                // All possible neighbors of a cell in a square grid
                let deltas: [(isize, isize); 8] = [
                    (1, 0),
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                // Counting the number of mined neighbors a cell has
                let neighbor_idx: Vec<usize> = deltas
                    .iter()
                    .filter_map(|dxy| {
                        // check for boundary overflow errors
                        if (xy.0 == 0 && dxy.0 == -1)
                            || (xy.0 == n_rows - 1 && dxy.0 == 1)
                            || (xy.1 == 0 && dxy.1 == -1)
                            || (xy.1 == n_cols - 1 && dxy.1 == 1)
                        {
                            None
                        } else {
                            Self::xy_to_idx(
                                (
                                    (xy.0 as isize + dxy.0) as usize,
                                    (xy.1 as isize + dxy.1) as usize,
                                ),
                                n_rows,
                                n_cols,
                            )
                        }
                    })
                    .collect();

                // Count valid neighbors that are mined
                let neighboring_mines_count = neighbor_idx
                    .iter()
                    .filter_map(|idx| {
                        if mine_indices.contains(idx) {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .count();

                grid_vec.push(Cell {
                    x: xy.0,
                    y: xy.1,
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(neighboring_mines_count),
                });
            }
        }

        Grid {
            n_rows,
            n_cols,
            grid_vec,
        }
    }

    /// convert 1D index to a 2D index
    pub fn idx_to_xy(idx: usize, n_rows: usize, n_cols: usize) -> Option<(usize, usize)> {
        if idx < n_rows * n_cols {
            return Some((idx / n_cols, idx.rem_euclid(n_cols)));
        }
        None
    }

    /// convert 2D index to a 1D index
    pub fn xy_to_idx(xy: (usize, usize), n_rows: usize, n_cols: usize) -> Option<usize> {
        if xy.0 < n_rows && xy.1 < n_cols {
            return Some(xy.0 * n_cols + xy.1);
        }
        None
    }
}

impl Display for Grid {
    /// Writes the grid as a `String` in stdin
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid_string = String::new();
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                match self.grid_vec[Self::xy_to_idx((i, j), self.n_rows, self.n_cols).unwrap()].data
                {
                    CellData::Mine => grid_string.push('*'),
                    CellData::MineNeighbor(count) => {
                        grid_string.push_str(count.to_string().as_str())
                    }
                }
                if j != self.n_cols {
                    grid_string.push(' ');
                }
            }
            grid_string.push('\n');
        }

        f.write_str(&grid_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_to_xy() {
        let grid = Grid {
            n_rows: 3,
            n_cols: 4,
            grid_vec: Vec::new(),
        };
        assert_eq!(grid.idx_to_xy(1), Ok((0, 1)));
        assert_eq!(grid.idx_to_xy(4), Ok((1, 0)));
        assert_eq!(grid.idx_to_xy(5), Ok((1, 1)));
        assert_eq!(grid.idx_to_xy(12), Err(()));
    }

    #[test]
    fn test_xy_to_idx() {
        let grid = Grid {
            n_rows: 3,
            n_cols: 4,
            grid_vec: Vec::new(),
        };

        assert_eq!(grid.xy_to_idx((0, 0)), Ok(0));
        assert_eq!(grid.xy_to_idx((0, 1)), Ok(1));
        assert_eq!(grid.xy_to_idx((1, 0)), Ok(4));
        assert_eq!(grid.xy_to_idx((1, 1)), Ok(5));
        assert_eq!(grid.xy_to_idx((10, 1)), Err(()));
    }
}
