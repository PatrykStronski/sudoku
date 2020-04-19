pub struct Sudoku {
    pub id: u8,
    pub difficulty: u8,
    pub solution: Vec<i16>,
    pub initial_board: Vec<i16>,
    pub current_board: Vec<i16>
}

impl Sudoku {
    fn print_board(&self, board: Vec<i16>) -> String {
        let mut cnt = 0;
        let mut return_str = "".to_string();
        for digit in board {
            if cnt % 9 == 0 {
                return_str.push_str("\n");
            }
            return_str.push_str(&digit.to_string());
            cnt += 1;
        }
        return return_str;
    }

    pub fn print_initial_board(&self) -> String {
        return self.print_board(self.initial_board.to_vec());
    }

    pub fn print_current_board(&self) -> String {
        return self.print_board(self.current_board.to_vec());
    }

    pub fn check_with_solution(&self) {
    }
}
