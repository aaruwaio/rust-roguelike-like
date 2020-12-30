use crate::grid::Grid;

const GRID_SIZE: u32 = 100;
const GRID_SPACE: u32 = 5;
const GRID_OFFSET: u32 = 10;
const FIELD_SIZE_X: usize = 12;
const FIELD_SIZE_Y: usize = 5;

pub struct Field {
    grids: [[Grid; FIELD_SIZE_X]; FIELD_SIZE_Y],
}

impl Field {
    pub fn new() -> Self {
        let mut grids = [[Grid::new(0, 0); FIELD_SIZE_X]; FIELD_SIZE_Y];
        for i in 0..FIELD_SIZE_Y {
            for j in 0..FIELD_SIZE_X {
                grids[i][j] = Grid::new(
                    j as u32 * (GRID_SIZE + GRID_SPACE) + GRID_OFFSET,
                    i as u32 * (GRID_SIZE + GRID_SPACE) + GRID_OFFSET,
                );
            }
        }
        Self { grids }
    }

    pub fn get_grid(&mut self, x: usize, y: usize) -> &mut Grid {
        &mut self.grids[y][x]
    }
}
