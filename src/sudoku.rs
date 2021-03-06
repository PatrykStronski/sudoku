pub struct Sudoku {
    pub id: u8,
    pub difficulty: u8,
    pub solution: Vec<i16>,
    pub initial_board: Vec<i16>,
    pub current_board: Vec<i16>
}

impl Sudoku {
    pub fn calculate_index(&self, pos_x: usize, pos_y: usize) -> usize {
        let index: usize = pos_y * 9 + pos_x;
        return index;
    }

    fn print_board(&self, board: Vec<i16>) -> String {
        let mut cnt = 0;
        let mut return_str = "".to_string();
        for digit in board {
            if cnt % 9 == 0 {
                return_str.push_str("\n");
            }
            return_str.push_str(&digit.to_string());
            return_str.push_str(" ");
            cnt += 1;
        }
        return return_str;
    }

    pub fn nullify_solution(&mut self) {
        self.current_board = self.initial_board.to_vec();
    }

    pub fn print_current_board(&self) -> String {
        return self.print_board(self.current_board.to_vec());
    }

    pub fn get_errors_with_solution(&self) -> u8 {
        let mut errors = 0;
        if self.solution.len() == 0 {
            if self.validate_solution() && self.get_quantity_empty_fields() == 0 {
                return 0;
            } else {
                return 1;
            }
        }
        for i in 0..80 {
            if self.current_board[i] != self.solution[i] {
                errors += 1;
            }
        }
        return errors;
    }

    fn calculate_square_indices(&self, pos_x: usize, pos_y: usize) -> Vec<usize> {
        let x_square: usize = pos_x/3;
        let y_square: usize = pos_y/3;
        let mut indices = Vec::<usize>::new();
        let mut start_row = y_square*3*9 + x_square*3;
        for i in start_row..(start_row+3) {
            indices.push(i);
        }
        start_row += 9;
        for i in start_row..(start_row+3) {
            indices.push(i);
        }
        start_row += 9;
        for i in start_row..(start_row+3) {
            indices.push(i);
        }
        return indices;
    }

    fn calculate_nth_square_indices(&self, ind: usize) -> Vec<usize> {
        let x_square: usize = (ind % 3) * 3;
        let y_square: usize = (ind / 3) * 3;
        return self.calculate_square_indices(x_square, y_square);
    }

    fn check_row(&self, row: usize) -> Vec<i16> {
        let start: usize = row * 9;
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        for i in start..(start+9) {
            if self.current_board[i] != -1 {
                numbers.retain(|&x| x != self.current_board[i]);
            }
        }
        return numbers;
    }

    fn check_column(&self, mut col: usize) -> Vec<i16> {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        while col < 81 {
            if self.current_board[col as usize] != -1 {
                numbers.retain(|&x| x != self.current_board[col]);
            }
            col += 9;
        }
        return numbers;
    }

    fn check_square(&self, pos_x: usize, pos_y: usize) -> Vec<i16> {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        let square_indices = self.calculate_square_indices(pos_x, pos_y);
        for i in square_indices {
            if self.current_board[i as usize] != -1 {
                numbers.retain(|&x| x != self.current_board[i]);
            }
        }
        return numbers;
    }

    fn choose_unique_vec(&self, vec1: &Vec<i16>, vec2: &Vec<i16>, vec3: &Vec<i16>) -> Vec<i16> {
        let mut comp1 = Vec::<i16>::new();
        for i in vec1 {
            for j in vec2 {
                if i == j {
                    comp1.push(*i)
                }
            }
        }
        if comp1.len() == 0 {
            return comp1;
        }
        let mut comp2 = Vec::<i16>::new();
        for i in comp1 {
            for j in vec3 {
                if i == *j {
                    comp2.push(i)
                }
            }
        }
        return comp2;
    }

    pub fn is_empty(&self, pos_x: usize, pos_y: usize) -> bool {
        let index = self.calculate_index(pos_x, pos_y);
        if self.current_board[index] == -1 {
            return true
        }
        return false;
    }

    pub fn insert_field(&mut self, pos_x: usize, pos_y: usize, value: i16) {
        let index = self.calculate_index(pos_x, pos_y);
        self.current_board[index] = value;
    }

    pub fn nullify_field(&mut self, pos_x: usize, pos_y: usize) {
        let index = self.calculate_index(pos_x, pos_y);
        self.current_board[index] = -1;
    }

    pub fn get_quantity_empty_fields(&self) -> u8 {
        let mut qty: u8 = 0;
        for i in 0..81 {
            if self.current_board[i] == -1 {
                qty +=1;
            }
        }
        return qty;
    }

    pub fn get_possible_solutions(&self, pos_x: usize, pos_y: usize) -> Vec<i16> {
        let row_options = self.check_row(pos_y);
        let column_options = self.check_column(pos_x);
        let square_options = self.check_square(pos_x, pos_y);
        return self.choose_unique_vec(&row_options, &column_options, &square_options);
    }

    fn find_in(&self, number: i16, numbers: &Vec<i16>) -> i16 {
        for n in 0..numbers.len() {
            if numbers[n] == number {
                return n as i16;
            }
        }
        return -1;
    }

    fn validate_row(&self, mut ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        ind = ind * 9;
        for i in ind..(ind+9) {
            if self.current_board[i] != -1 {
                let nmb_index = self.find_in(self.current_board[i] ,&numbers);
                if nmb_index == -1 {
                    return false;
                } else {
                    numbers.remove(nmb_index as usize);
                }
            }
        }
        return true;
    }

    fn validate_column(&self, mut ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        while ind < 81 {
            if self.current_board[ind] != -1 {
                let nmb_index = self.find_in(self.current_board[ind] ,&numbers);
                if nmb_index == -1 {
                    return false;
                } else {
                    numbers.remove(nmb_index as usize);
                }
            }
            ind += 9;
        }
        return true;
    }

    fn validate_square(&self, ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        let square_indices = self.calculate_nth_square_indices(ind);
        for i in square_indices {
            if self.current_board[i] != -1 {
                let nmb_index = self.find_in(self.current_board[i] ,&numbers);
                if nmb_index == -1 {
                    return false;
                } else {
                    numbers.remove(nmb_index as usize);
                }
            }
        }
        return true;
    }

    pub fn validate_solution(&self) -> bool {
        for x in 0..9 {
            if !self.validate_column(x) {
                return false;
            }
            if !self.validate_row(x) {
                return false;
            }
            if !self.validate_square(x) {
                return false;
            }
        }
        return true;
    }

    pub fn calculate_domains(&self, base_domains: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
        let mut domains = base_domains.to_vec();
        for y in 0..9 {
            for x in 0..9 {
                if !self.is_empty(x, y) {
                    continue;
                }
                let pos_sols = self.get_possible_solutions(x, y);
                if pos_sols.len() == 0 {
                    return vec![];
                }
                domains[self.calculate_index(x, y)] = pos_sols;
            }
        }
        return domains;
    }

    pub fn create_domains(&self) -> Vec<Vec<i16>> {
        let mut domains = Vec::<Vec<i16>>::with_capacity(81);
        for y in 0..9 {
            for x in 0..9 {
                if !self.is_empty(x, y) {
                    domains.push(vec![-1]);
                    continue;
                }
                let pos_sols = self.get_possible_solutions(x, y);
                domains.push(pos_sols);
            }
        }
        return domains;
    }
}
