use crate::sudoku::Sudoku;
use std::time::{Duration, Instant};

fn backtrack_step(sdk: &mut Sudoku, mut pos_x: usize, mut pos_y: usize) -> bool {
    if pos_x == 9 && pos_y == 8 {
        if sdk.validate_solution() {
            return true;
        }
        return false;
    }
    if pos_x == 9 {
        pos_y += 1;
        pos_x = 0;
    }
    if !sdk.is_empty(pos_x, pos_y) {
        return backtrack_step(sdk, pos_x + 1, pos_y);
    }
    let possible_solutions = sdk.get_possible_solutions(pos_x, pos_y);
    for sol in possible_solutions {
        sdk.insert_field(pos_x, pos_y, sol);
        if backtrack_step(sdk, pos_x + 1, pos_y) {
            return true;
        }
        sdk.nullify_field(pos_x, pos_y);
    }
    return false;
}

fn backtrack(sdk: &mut Sudoku) -> bool {
    if backtrack_step(sdk, 0 as usize, 0 as usize) {
        return true;
    }
    return false;
}

pub fn solve_sudoku(sdk: &mut Sudoku) -> bool {
    println!("{}", sdk.print_current_board());
    let now = Instant::now();
    if sdk.validate_solution() {
        if backtrack(sdk) {
            println!("{}", sdk.print_current_board());
            println!("Errors with solution: {}", sdk.get_errors_with_solution());
            println!("Execution time: {}", now.elapsed().as_millis());
            return true;
        } else {
            println!("UNSOLVABLE");
            println!("Execution time: {}", now.elapsed().as_millis());
            return false;
        }
    }
    println!("INVALID INPUT DATA");
    println!("Execution time: {}", now.elapsed().as_millis());
    return false;
}