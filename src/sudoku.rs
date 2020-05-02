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
            return_str.push_str(" ");
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

    pub fn get_errors_with_solution(&self) -> u8 {
        let mut errors = 0;
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

    fn calculate_nth_square_indices(&self, ind: usize -> Vec<usize> {
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

    fn choose_unique(&self, vec1: &Vec<i16>, vec2: &Vec<i16>, vec3: &Vec<i16>) -> i16 {
        let mut comp1 = Vec::<i16>::new();
        for i in vec1 {
            for j in vec2 {
                if i == j {
                    comp1.push(*i)
                }
            }
        }
        if comp1.len() == 0 {
            return -1;
        }
        let mut comp2 = Vec::<i16>::new();
        for i in comp1 {
            for j in vec3 {
                if i == *j {
                    comp2.push(i)
                }
            }
        }
        println!("Unique {:?}", comp2);
        if comp2.len() != 1 {
            return -1;
        }
        return comp2[0];
    }

    pub fn is_empty(&self, pos_x: usize, pos_y: usize) -> bool {
        let index: usize = pos_y * 9 + pos_x;
        if self.current_board[index] == -1 {
            return true
        }
        return false;
    }
    
    pub fn get_field(&self, pos_x: usize, pos_y: usize) -> i16 {
        let index: usize = pos_y * 9 + pos_x;
        return self.current_board[index];
    }

    pub fn insert_field(&mut self, pos_x: usize, pos_y: usize, value: i16) {
        let index = pos_y * 9 + pos_x;
        self.current_board[index] = value;
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

    pub fn fill_in_field(&mut self, pos_x: usize, pos_y: usize) -> bool {
        let row_options = self.check_row(pos_y);
        let column_options = self.check_column(pos_x);
        let square_options = self.check_square(pos_x, pos_y);
        println!("{:?}",row_options);
        println!("{:?}",column_options);
        println!("{:?}",square_options);
        let unique = self.choose_unique(&row_options, &column_options, &square_options);
        if unique != -1 {
            self.insert_field(pos_x, pos_y, unique);
            return true;
        }
        return false;
    }

    fn find_in(&self, number: i16, numbers: &Vec<i16>) -> i16 {
        for n in numbers {
            if numbers[n] == number {
                return n;
            }
        }
        return -1;
    }

    fn validate_row(&self, ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        for i as usize in ind..(ind+9) {
            if self.current_board[i] != -1 {
                let nmb_index = self.find_in(self.current_board[i] ,&numbers);
                if numb_index == -1 {
                    return false;
                } else {
                    numbers.remove(numb_index as usize);
                }
            }
        }
    }

    fn validate_column(&self, mut ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        while ind < 81 {
            if self.current_board[i] != -1 {
                let nmb_index = self.find_in(self.current_board[i] ,&numbers);
                if numb_index == -1 {
                    return false;
                } else {
                    numbers.remove(numb_index as usize);
                }
            }
            ind += 9;
        }
        return true;
    }

    fn validate_square(&self, ind: usize) -> bool {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9];
        let square_indices = self.calculate_nth_square_indices(ind);
        for i as usize in ind..(ind+9) {
            if self.current_board[i] != -1 {
                let nmb_index = self.find_in(self.current_board[i] ,&numbers);
                if numb_index == -1 {
                    return false;
                } else {
                    numbers.remove(numb_index as usize);
                }
            }
        }
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
}
