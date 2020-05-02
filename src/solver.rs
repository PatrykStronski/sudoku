use crate::sudoku::Sudoku;

fn fill_in(sdk: &mut Sudoku) -> u8 {
    let mut count: u8 = 0;
    for y in 0..9 {
        for x in 0..9 {
            let filled_in = sdk.fill_in_field(x as usize, y as usize);
            if filled_in {
                count += 1;
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
    for x in 0..9 {
        for y in 0..9 {
            if sdk.get_field(x, y) == -1 {
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

pub fn solve_sudoku(sdk: &mut Sudoku) {
    println!("{}", sdk.print_current_board());
    for repeat in 0..1000 {
        fill_in_till_possible(sdk);
        for threshold in 2..9 {
            let changed = guess_of_lowest_options(sdk, threshold);
            if changed > 0 {
                break;
            }
        }
        if sdk.get_errors_with_solution() == 0 {
            break;
        }
    }
    println!("{}", sdk.print_current_board());
}