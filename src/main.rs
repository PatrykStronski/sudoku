mod sudoku;
mod load_puzzle;
mod solver;

fn solve_sudoku_with_check(sdk: &mut sudoku::Sudoku) {
    solver::solve_sudoku(sdk);
}

fn main() -> Result<(), ()> {
    println!("Loading sudokus");
    let mut sudokus = load_puzzle::load()?;
    for mut sdk in sudokus {
        solve_sudoku_with_check(&mut sdk);
    }
    Ok(())
}
