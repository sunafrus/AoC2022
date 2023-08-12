use grid::{Grid, GridCoord};

mod grid;

fn main() {
    let grid = parse_grid(include_str!("input.txt"));

    let all_coords = (0..grid.height()).flat_map(|y| {
        (0..grid.width()).map(move |x| GridCoord::from((x, y)))
    });

    /*
    let num_visible_cells = all_coords
        .filter(|&coord| {
            let coord_height = grid.cell(coord).unwrap();
            let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            deltas.iter().any(|&(dx, dy)| {
                let mut cells_in_line = (1..).map_while(|i| {
                    let coord = GridCoord {
                        x: coord.x.checked_add_signed(dx * i)?,
                        y: coord.y.checked_add_signed(dy * i)?,
                    };
                    grid.cell(coord)
                });
                cells_in_line.all(|height| height < coord_height)
            })
        })
        .count();
    dbg!(num_visible_cells);
    */

    let best_place = all_coords
        .map(|coord| (coord, scenic_score(&grid, coord)))
        .max_by_key(|(_, score)| *score)
        .unwrap();

    println!("{best_place:?}");
}


fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }

    grid
}

fn visible_trees_in_dir(grid: &Grid<usize>, coord: GridCoord, (dir_x, dir_y): (isize, isize)) -> usize {
    let line = (1..).map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dir_x * i)?,
            y: coord.y.checked_add_signed(dir_y * i)?,
        };
        Some(*grid.cell(coord)?)
    });

    let mut total = 0;
    let our_height = *grid.cell(coord).unwrap();

    for height in line {
        total += 1;
        if height >= our_height {
            break;
        }
    }

    total

}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|(dir_x, dir_y)| visible_trees_in_dir(grid, coord, (dir_x, dir_y)))
        .product()
}