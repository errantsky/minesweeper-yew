use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use rand::seq::IteratorRandom;
use rand::thread_rng;

#[derive(Eq, PartialEq)]
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
            if mine_indices.contains(&idx) {
                grid_vec.push(Cell {
                    is_mine: true,
                    is_clicked: false,
                    flag: None,
                    data: CellData::Mine,
                });
            } else {
                // Counting the number of mined neighbors a cell has
                let neighbor_idx = Self::valid_neighbor_indices(idx, n_rows, n_cols);

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

    /// Return indices of all possible neighbors of a cell in a grid
    /// ToDo: Add tests
    pub fn valid_neighbor_indices(idx: usize, n_rows: usize, n_cols: usize) -> Vec<usize> {
        let xy = Grid::idx_to_xy(idx, n_rows, n_cols).unwrap();
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

        neighbor_idx
    }

    pub fn reveal_empty_cells(&mut self, idx: usize) -> usize {
        let mut to_visit: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        to_visit.push(idx);

        while !to_visit.is_empty() {
            let cell_idx = to_visit.pop().unwrap();
            visited.insert(cell_idx);
            self.grid_vec[cell_idx].is_clicked = true;
            if self.grid_vec[cell_idx].data == CellData::MineNeighbor(0) {
                let mut neighbor_indices =
                    Grid::valid_neighbor_indices(cell_idx, self.n_rows, self.n_cols)
                        .into_iter()
                        .filter(|nidx| !visited.contains(nidx))
                        .collect();
                to_visit.append(&mut neighbor_indices);
            }
        }
        visited.len()
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

    pub fn overlay_display(&self) -> String {
        let mut grid_string = String::new();
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                if self.grid_vec[Self::xy_to_idx((i, j), self.n_rows, self.n_cols).unwrap()]
                    .is_clicked
                {
                    match self.grid_vec[Self::xy_to_idx((i, j), self.n_rows, self.n_cols).unwrap()]
                        .data
                    {
                        CellData::Mine => grid_string.push('*'),
                        CellData::MineNeighbor(count) => {
                            grid_string.push_str(count.to_string().as_str())
                        }
                    }
                } else {
                    grid_string.push('?');
                }
                if j != self.n_cols {
                    grid_string.push(' ');
                }
            }
            grid_string.push('\n');
        }
        grid_string
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
        assert_eq!(Grid::idx_to_xy(1, 3, 4), Some((0, 1)));
        assert_eq!(Grid::idx_to_xy(4, 3, 4), Some((1, 0)));
        assert_eq!(Grid::idx_to_xy(5, 3, 4), Some((1, 1)));
        assert_eq!(Grid::idx_to_xy(12, 3, 4), None);
    }

    #[test]
    fn test_xy_to_idx() {
        let grid = Grid {
            n_rows: 3,
            n_cols: 4,
            grid_vec: Vec::new(),
        };
        assert_eq!(Grid::xy_to_idx((0, 0), 3, 4), Some(0));
        assert_eq!(Grid::xy_to_idx((0, 1), 3, 4), Some(1));
        assert_eq!(Grid::xy_to_idx((1, 0), 3, 4), Some(4));
        assert_eq!(Grid::xy_to_idx((1, 1), 3, 4), Some(5));
        assert_eq!(Grid::xy_to_idx((10, 1), 3, 4), None);
    }

    #[test]
    fn test_overlay_display() {
        let grid = Grid::new(3, 3);
        println!("{}", grid.overlay_display());
        assert_eq!(grid.overlay_display(), "? ? ? \n? ? ? \n? ? ? \n");
    }

    #[test]
    /// * 1 0
    /// 1 1 0
    /// 0 0 0
    /// Given the grid above, if the bottom right cell is clicked,
    /// all cells except the top left one should be revealed, or clicked.
    fn test_reveal_empty_cells() {
        let idx = 8;
        let mut grid = Grid {
            n_rows: 3,
            n_cols: 3,
            grid_vec: vec![
                Cell {
                    is_mine: true,
                    is_clicked: false,
                    flag: None,
                    data: CellData::Mine,
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(1),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(0),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(1),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(1),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(0),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(0),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(0),
                },
                Cell {
                    is_mine: false,
                    is_clicked: false,
                    flag: None,
                    data: CellData::MineNeighbor(0),
                },
            ],
        };
        grid.grid_vec[idx].is_clicked = true;
        grid.reveal_empty_cells(idx);
        println!("{}", grid.overlay_display());
    }
}
