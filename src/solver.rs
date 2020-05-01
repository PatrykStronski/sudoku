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

pub fn solve_sudoku(sdk: &mut Sudoku) {
    let mut counter = 0;
    while counter < 1000 {
        let filled_in = fill_in(sdk);
        if  filled_in == 0 {
            break;
        }
        counter += 1;
    }
    println!("{}", sdk.print_current_board());
}