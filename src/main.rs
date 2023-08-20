use std::{io};

pub struct CellVal {
    value: usize,
}

impl CellVal {
    pub fn new(value: usize) -> Result<CellVal, &'static str> {
        if value > 2 {
            return Err("Value must me between 0-2!");
        }
        Ok(CellVal{value})
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

fn prompt_user(prompt: &str) -> CellVal {
    loop {
        let mut user_input = String::new();

        println!("{}", prompt);

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        match user_input.trim().parse::<usize>() {
            Ok(value) => {
                match CellVal::new(value) {
                    Ok(value) => return value,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    },
                }
            },
            Err(_) => {
                println!("Enter a number!");
                continue;
            },
        };
    }
}

fn user_cell_get(board: [[char;3];3], player: char) -> (CellVal, CellVal) {
    let mut row: CellVal;
    let mut col: CellVal;

    loop {
        println!("Player {player}'s turn");
        // get row
        row = prompt_user("Enter row: ");

        // get column
        col = prompt_user("Enter col: ");

        if board[row.value()][col.value()] == '-' {
            break;
        } else {
            println!("Yo, that spot is taken!");
        }
    } 

    (row, col)
}

fn print_board(board: [[char;3];3]) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn empty_board() -> [[char; 3];3] {
    [['-', '-', '-'],
     ['-', '-', '-'],
     ['-', '-', '-']
    ]
}

fn check_win(board: [[char;3];3]) -> bool {
    // check rows
    for row in board {
        if row[0] == row[1] && row[0] == row[2] && row[0] != '-' {
            return true;
        }
    }

    // check columns
    for i in 0..2 {
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] && board[0][i] != '-' {
            return true;
        }
    }

    // check diagnals
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] && board[0][0] != '-' {
        return true;
    }
    if board[0][2] == board[1][1] && board[0][2] == board[0][2] && board[2][0] != '-' {
        return true;
    }

    false
}

fn main() {
    let mut board: [[char;3];3] = empty_board();
    let mut player: char = 'x';

    loop {
        print_board(board);

        let (row, col) = user_cell_get(board, player);

        board[row.value()][col.value()] = player;

        if check_win(board) {
            print_board(board);
            println!("Player {player} won :D");
            break;
        }

        player = if player == 'x' { 'o' } else {'x'};
    }
}
