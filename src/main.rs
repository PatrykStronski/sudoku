mod sudoku;
mod load_puzzle;
mod solver;

fn solve_sudoku_with_check(sdk: &mut sudoku::Sudoku) {
    println!("Beginning board");
    println!("{}", sdk.print_initial_board());
    solver::solve_sudoku(sdk);
}

fn main() -> Result<(), ()> {
    println!("Loading sudokus");
    let sudokus = load_puzzle::load()?;
    for mut sdk in sudokus {
        solve_sudoku_with_check(&mut sdk);
    }
    Ok(())
}
