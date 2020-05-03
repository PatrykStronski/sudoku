use crate::sudoku::Sudoku;

fn check_possibilities_fill_in(sdk: &mut Sudoku, x: usize, y: usize) -> bool {
    let solutions = sdk.get_possible_solutions(x, y);
    let mut candidate_solutions = Vec::<i16>::new();
    for s in solutions {
        sdk.insert_field(x, y, s);
        if sdk.validate_solution() {
            candidate_solutions.push(s);
        }
    }
    if candidate_solutions.len() == 1 {
        sdk.insert_field(x, y, candidate_solutions[0]);
        return true;
    }
    sdk.nullify_field(x, y);
    return false;
}

fn fill_in(sdk: &mut Sudoku) -> u8 {
    let mut count: u8 = 0;
    for y in 0..9 {
        for x in 0..9 {
            if !sdk.is_empty(x, y) {
                continue;
            }
            let filled_in = sdk.fill_in_field(x as usize, y as usize);
            if filled_in {
                count += 1;
            } else {
                if check_possibilities_fill_in(sdk, x as usize, y as usize) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn fill_in_till_possible(sdk: &mut Sudoku) {
    let mut counter = 0;
    while counter < 100 {
        let filled_in = fill_in(sdk);
        if  filled_in == 0 {
            break;
        }
        counter += 1;
    }
}

fn guess_of_lowest_options(sdk: &mut Sudoku, threshold: usize) -> u8 {
    let mut changed: u8 = 0;
    for y in 0..9 {
        for x in 0..9 {
            if !sdk.is_empty(x, y) {
                continue;
            }
            let poss_solutions = sdk.get_possible_solutions(x, y);
            if poss_solutions.len() != threshold {
                continue;
            }
            for sol in poss_solutions {
                sdk.insert_field(x, y, sol);
                if sdk.validate_solution() {
                    break;
                }
            }
            changed += 1;
        }
    }
    return changed;
}

fn backtrack_step(sdk: &mut Sudoku, mut pos_x: usize, mut pos_y: usize) -> bool {
    if pos_x == 9 && pos_y == 8 {
        if sdk.get_errors_with_solution() == 0 {
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
    fill_in_till_possible(sdk);
    backtrack(sdk);
    println!("{}", sdk.print_current_board());
    println!("{}", sdk.get_errors_with_solution());
}