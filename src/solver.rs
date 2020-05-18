use crate::sudoku::Sudoku;
use std::time::Instant;

fn backtrack_step(sdk: &mut Sudoku, mut pos_x: usize, mut pos_y: usize) -> bool {
    if pos_x == 9 && pos_y == 8 {
        return sdk.validate_solution();
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
    let now = Instant::now();
    if backtrack_step(sdk, 0 as usize, 0 as usize) {
        //println!("{}", sdk.print_current_board());
        //println!("Errors with solution: {}", sdk.get_errors_with_solution());
        println!("Execution time with basic backtrack: {}", now.elapsed().as_millis());    
        return true;
    }
    println!("UNSOLVABLE");
    println!("Execution time: {}", now.elapsed().as_millis());
    return false;
}

fn backtrack_forward_step(sdk: &mut Sudoku, domains: Vec<Vec<i16>>, mut pos_x: usize, mut pos_y: usize) -> bool {
    if pos_x == 9 && pos_y == 8 {
        return sdk.validate_solution();
    }
    if pos_x == 9 {
        pos_y += 1;
        pos_x = 0;
    }
    if !sdk.is_empty(pos_x, pos_y) {
        return backtrack_forward_step(sdk, domains, pos_x + 1, pos_y);
    }
    let possible_solutions = domains[sdk.calculate_index(pos_x, pos_y)].to_vec();
    for sol in possible_solutions {
        sdk.insert_field(pos_x, pos_y, sol);
        let new_domains = sdk.calculate_domains(&domains);
        if new_domains.len() != 0 && backtrack_forward_step(sdk, new_domains, pos_x + 1, pos_y) {
            return true;
        }
        sdk.nullify_field(pos_x, pos_y);
    }
    return false;
}

fn backtrack_forward_check(sdk: &mut Sudoku) -> bool {
    let now = Instant::now();
    let domains = sdk.create_domains();
    if backtrack_forward_step(sdk, domains, 0 as usize, 0 as usize) {
        //println!("{}", sdk.print_current_board());
        //println!("Errors with solution: {}", sdk.get_errors_with_solution());
        println!("Execution time with forward checking: {}", now.elapsed().as_millis());    
        return true;
    }
    println!("UNSOLVED");
    println!("Execution time: {}", now.elapsed().as_millis());
    return false;
}

pub fn solve_sudoku(sdk: &mut Sudoku) -> bool {
    //println!("{}", sdk.print_current_board());
    let now = Instant::now();
    if sdk.validate_solution() {
        let mut correct = 0;
        if backtrack(sdk) {
            correct += 1;
        }
        sdk.nullify_solution();
        if backtrack_forward_check(sdk) {
            correct +=1;
        }
        println!("{} methods solved the sudoku", correct);
        return correct == 2;
    }
    println!("INVALID INPUT DATA");
    println!("Execution time: {}", now.elapsed().as_millis());
    return false;
}
