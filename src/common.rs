use core::panic;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Word {
    pub path: Vec<Pos>,
    pub word: String,
}

pub type Word1D = u32;

pub static COLS: usize = 5;
pub static ROWS: usize = 6;

pub fn print_board(board: &Vec<Vec<char>>, highlight: Word1D) {
    for j in 0..board.len() {
        let mut row = String::new();
        for i in 0..board[j].len() {
            let index = get_index(board.len(), board.get(0).unwrap().len(), j, i);

            let convert_uppercase = ((highlight >> index) & 1) == 1;
            let char = board[j][i];
            if convert_uppercase {
                char.to_uppercase().for_each(|m| row.push(m));
            } else {
                char.to_lowercase().for_each(|m| row.push(m));
            }
            row.push(' ');
        }
        println!("{:?}", row)
    }
}
pub fn get_index(rows: usize, cols: usize, y: usize, x: usize) -> usize {
    if x >= cols || y >= rows {
        panic!("Cannot get index");
    }
    return cols * y + x;
}
