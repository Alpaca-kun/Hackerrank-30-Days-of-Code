use std::io::*;
use std::cmp::*;

fn main() {
    let square_grid_size: usize = 6;
    let mut input = String::new();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..square_grid_size {
        stdin().read_line(&mut input).unwrap();

        let line: Vec<i32> = input.trim().split_whitespace()
                                .map(|number| number.parse().unwrap())
                                .collect();

        grid.push(line);
        input.clear();
    }

    let mut hourglass_max_sum: i32 = i32::MIN;

    for i in 1..square_grid_size - 1 {
        for j in 1..square_grid_size - 1 {
            hourglass_max_sum = max(grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1] +
                                    grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1] + grid[i][j],
                                    hourglass_max_sum);
        }
    }

    println!("{}", hourglass_max_sum);
}
