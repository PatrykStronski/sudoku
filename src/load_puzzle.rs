use std::error::Error;
use csv::ReaderBuilder;
use serde_derive::Deserialize;
use crate::sudoku::Sudoku;

const INPUT_FILE: &str = "./resources/Sudoku.csv";

#[derive(Debug, Deserialize)]
struct Record {
    id: u8,
    difficulty: f32,
    puzzle: String,
    solution: String
}

fn extract_board(board: String) -> Vec<i16> {
    let mut board_i = Vec::<i16>::new();
    for digit in board.chars() {
        if digit == '.' { board_i.push(-1); }
        else { 
            let res = i16::from_str_radix(&digit.to_string(), 10);
            match res {
                Ok(number) => board_i.push(number), 
                Err(e) => println!("{}",e)
            };
        }
    }
    return board_i;
}

fn parse_sudoku(input_record: &Record) -> Sudoku {
    let difficulty: u8 = input_record.difficulty.round() as u8;
    let initial_board: Vec<i16> = extract_board(input_record.puzzle.to_string());
    let solution: Vec<i16> = extract_board(input_record.solution.to_string());
    return Sudoku {
        id: input_record.id,
        difficulty: difficulty,
        initial_board: initial_board.to_vec(),
        current_board: initial_board,
        solution: solution
    };
}

fn get_sudokus(recs: Vec<Record>) -> Vec<Sudoku> {
    let mut sudokus = Vec::<Sudoku>::new();
    for input in recs {
        sudokus.push(parse_sudoku(&input));
    }
    return sudokus;
}

fn read_file(input_file: String) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut recs = Vec::<Record>::new();
    let mut rdr = ReaderBuilder::new()
        .from_path(input_file)?;
    for row in rdr.deserialize() {
        let row_record: Record = row?;
        recs.push(row_record);
    }
    Ok(recs)
}

pub fn load() -> Result<Vec<Sudoku>, ()> {
    let res = read_file(INPUT_FILE.to_string());
    match res {
        Ok(recs) => {
            let sudokus = get_sudokus(recs);
            Ok(sudokus)
        },
        Err(e) => {
            println!("{}", e);
            Err(())
        }
    }
}
