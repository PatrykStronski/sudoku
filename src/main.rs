mod sudoku;
mod load_puzzle;

fn solve_sudoku_with_check(sdk: &sudoku::Sudoku) {
    println!("Beginning board");
    println!("{}", sdk.print_initial_board());
}

fn main() -> Result<(), ()> {
    println!("Loading sudokus");
    let sudokus = load_puzzle::load()?;
    for sdk in sudokus {
        solve_sudoku_with_check(&sdk);
    }
    Ok(())
}
