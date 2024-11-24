mod common;
mod find_all_words;
mod kotus;
mod neighbors;
mod requests;
mod find_solution;

use chrono::{DateTime, Datelike, FixedOffset, Utc};
use std::env;
use std::time::Instant;

use common::{Pos, Word, Word1D, COLS, ROWS};
use requests::fetch_board_for_date;

use crate::{
    common::{get_index, print_board}, find_all_words::find_words_starting_from, find_solution::solve, kotus::read_csv
};

fn build_board(data: &Vec<char>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec!['X'; COLS]; ROWS];
    for j in 0..ROWS {
        for i in 0..COLS {
            res[j][i] = data[j * COLS + i]; 
        }
    }
    return res;
}

fn get_today_string() -> String {
    let now = Utc::now();
    // offset 3h -> 10800s
    let offset = FixedOffset::east_opt(10800).unwrap();
    let today: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(now.naive_utc(), offset);
    let day = today.day();
    let month = today.month();
    let year = today.year();
    return String::from(day.to_string() + "." + &month.to_string() + "." + &year.to_string());
}

fn filter_words(words: &Vec<String>, allowed_chars: &Vec<char>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for word in words {
        let mut valid = true;
        let word_size = word.len();
        if (word_size < 3) || (word_size > 10) {
            continue;
        }
        for c in word.chars() {
            if !allowed_chars.contains(&c) {
                valid = false;
                break;
            }
        }
        if valid {
            res.push(word.clone());
        }
    }
    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board_date: String = get_today_string();
    let mut max_count: usize = 1usize;

    if args.len() > 1 {
        max_count = usize::from_str_radix(&args[1].to_string(), 10).unwrap_or_default();
    }

    if args.len() > 2 {
        board_date = args[2].to_string();
    }

    println!("Searching results for puzzle {}", board_date);
    println!("Max results, {}", max_count);
    let read_res = read_csv();
    let words: Vec<String> = match read_res {
        Ok(data) => data.into_iter().map(|x| x.to_uppercase()).collect(),
        Err(err) => panic!("Problem opening file, {:?}", err),
    };
    println!("Found {:?} words from the static word list", words.len());
    let board_chars = fetch_board_for_date(&board_date);
    let board = build_board(&board_chars);
    println!("Allowed characters, {:?}", board_chars);
    let filtered_words = filter_words(&words, &board_chars);
    println!("Filtered words, {:?}", filtered_words.len());
    print_board(&board, u32::MAX);
    let mut matches: Vec<Word> = Vec::new();
    println!("Searching for all available words for this board");
    let word_search_start = Instant::now();
    for j in 0..board.len() {
        let row = &board[j];
        for i in 0..row.len() {
            let pos = Pos { x: i, y: j };
            let mut inner_matches = find_words_starting_from(&board, &filtered_words, pos);
            matches.append(&mut inner_matches);
        }
    }
    matches.sort_by_cached_key(|x| x.path.len());
    matches.reverse();

    let mut matches_1d: Vec<Word1D> = Vec::new();
    let rows = board.len();
    let columns = board.get(0).unwrap().len();
    for word in matches.clone() {
        let mut current = u32::MIN;
        for pos in word.path {
            let x = pos.x;
            let y = pos.y;
            let index = get_index(rows, columns, y, x);
            let mask = 1 << index;
            current = current | mask;
        }
        matches_1d.push(current);
    }
    let mut word_vectors: Vec<&Word1D> = Vec::new();
    for i in 0..matches_1d.len() {
        word_vectors.push(&matches_1d[i]);
    }
    println!("Words found, {:?}", matches_1d.len());
    let word_search_end = Instant::now();
    let word_search_duration = word_search_end.duration_since(word_search_start);
    println!(
        "word search took {} milliseconds",
        word_search_duration.as_millis()
    );
    let solution: Vec<&Word1D> = Vec::new();
    let solution_search_start = Instant::now();
    let res = solve(word_vectors, &solution, &u32::MIN, max_count);
    println!(
        "Solutions found in {} milliseconds",
        Instant::now()
            .duration_since(solution_search_start)
            .as_millis(),
    );
    for i in 0..res.len() {
        println!("Result #{:?}", i);
        for w in res[i].clone() {
            for m_idx in 0..matches_1d.len() {
                if w == matches_1d[m_idx] {
                    println!(" ## {} ## ", matches[m_idx].word);
                    break;
                }
            }

            print_board(&board, w);
        }
        print!("\n");
    }
}
