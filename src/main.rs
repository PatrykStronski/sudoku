mod sudoku;
mod load_puzzle;
mod solver;

fn main() -> Result<(), ()> {
    println!("Loading sudokus");
    let sudokus = load_puzzle::load()?;
    let mut unsolved: u8 = 0;
    for mut sdk in sudokus {
        if !solver::solve_sudoku(&mut sdk) {
            unsolved += 1;
        }
    }
    println!("Unsolved: {} sudokus", unsolved);
    Ok(())
}
