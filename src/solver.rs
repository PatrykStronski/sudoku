use crate::sudoku::Sudoku;

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

fn backtrack(sdk: &mut Sudoku) {
    backtrack_step(sdk, 0 as usize, 0 as usize);
}

pub fn solve_sudoku(sdk: &mut Sudoku) {
    println!("{}", sdk.print_current_board());
    if sdk.validate_solution() {
        backtrack(sdk);
        println!("{}", sdk.print_current_board());
        println!("Errors with solution: {}", sdk.get_errors_with_solution());
    } else {
        println!("INVALID DATA");
    }

}