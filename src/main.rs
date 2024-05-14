mod common;
mod neighbors;

use csv::ReaderBuilder;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::path::Path;

use common::{Pos, Word};
use neighbors::get_neighbors;

use crate::common::{print_board, sort_result};

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(file);
    let mut results: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let word_res = record.get(0);
        match word_res {
            Some(x) => {
                if x.chars().count() > 2 {
                    results.push(x.to_string())
                }
            }
            None => (),
        }
    }
    Ok(results)
}

fn read_board<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut results: Vec<Vec<char>> = Vec::new();
    let data_res = read_to_string(filename);
    let data: Vec<String> = match data_res {
        Ok(d) => d.split("\n").map(|x| x.to_owned()).collect(),
        Err(err) => panic!("{:?}", err),
    };
    for line in data {
        let letters: Vec<char> = line.chars().collect();
        results.push(letters);
    }
    Ok(results)
}

fn inner(
    board: &Vec<Vec<char>>,
    visited: &Vec<Vec<bool>>,
    words: &Vec<String>,
    pos: &Pos,
    initial_word: String,
    path: &Vec<Pos>,
) -> Vec<Word> {
    let x = usize::try_from(pos.x).unwrap();
    let y = usize::try_from(pos.y).unwrap();
    let current_word = initial_word + &board[y][x].to_string();
    let mut results: Vec<Word> = Vec::new();
    let matches: Vec<String> = words
        .clone()
        .into_iter()
        .filter(|word| word.to_uppercase().eq(&current_word.to_uppercase()))
        .collect();
    if !matches.is_empty() {
        let word = matches.first().unwrap().to_string();
        results.push(Word {
            path: path.to_vec(),
            word,
        });
    }
    // println!("Matches: {:?}", matches);
    for neighbor in get_neighbors(pos, &visited) {
        let mut inner_visited = visited.clone();
        let x = usize::try_from(neighbor.x).unwrap();
        let y = usize::try_from(neighbor.y).unwrap();
        // println!("X: {:?}, Y: {:?}", x, y);
        inner_visited[y][x] = true;
        let inner_word = current_word.clone();
        let mut inner_words: Vec<String> = words.clone();
        // println!("inner words 0: {:?}", inner_words.get(0).unwrap());
        inner_words = inner_words
            .into_iter()
            .filter(|word| word.to_uppercase().starts_with(&inner_word.to_uppercase()))
            .collect();
        // println!("inner word: {:?}", inner_word);
        // println!(
        //     "inner words after
        // : {:?}",
        //     inner_words.len()
        // );
        let mut inner_path = path.clone();
        inner_path.push(Pos {
            x: neighbor.x,
            y: neighbor.y,
        });
        if inner_words.len() > 0 {
            let mut neighbor_results = inner(
                board,
                &inner_visited,
                &inner_words,
                &neighbor,
                inner_word,
                &inner_path,
            );
            results.append(&mut neighbor_results)
        }
    }
    return results;
}

fn find_words_starting_from(
    board: &Vec<Vec<char>>,
    words: &Vec<String>,
    start_pos: Pos,
) -> Vec<Word> {
    let mut visited: Vec<Vec<bool>> = board.iter().map(|row| vec![false; row.len()]).collect();
    // Set starting position to visited
    let xusize = usize::try_from(start_pos.x).unwrap();
    let yusize = usize::try_from(start_pos.y).unwrap();
    visited[yusize][xusize] = true;
    let mut path: Vec<Pos> = Vec::new();
    path.push(start_pos.clone());
    let res = inner(board, &visited, &words, &start_pos, String::new(), &path);
    return res;
}

fn vector_contains_hashset(vector: &Vec<HashSet<Pos>>, target_hashset: &HashSet<Pos>) -> bool {
    vector.iter().any(|item| item == target_hashset)
}

// Function to combine two vectors of solutions keeping only unique results
fn combine_results(a: &[Vec<Word>], b: &[Vec<Word>]) -> Vec<Vec<Word>> {
    let mut combined_results: Vec<Vec<Word>> = Vec::new();

    // Combine a and b into combined_results
    combined_results.extend_from_slice(a);
    combined_results.extend_from_slice(b);
    // Deduplicate the combined results
    combined_results.sort();
    combined_results.dedup();

    return combined_results;
}

fn find_solution(
    words: &Vec<Word>,
    solution: &Vec<Word>,
    dead_ends: &mut Vec<HashSet<Pos>>,
    done_count: i32,
    max_result_count: usize,
) -> Vec<Vec<Word>> {
    let visited: Vec<Pos> = solution.clone().into_iter().flat_map(|w| w.path).collect();
    let dead_end: HashSet<Pos> = HashSet::from_iter(visited.clone().into_iter());
    if visited.len() >= done_count.try_into().unwrap() {
        let printable: Vec<String> = solution.clone().into_iter().map(|w| w.word).collect();
        println!("Solution found: {:?}", printable.join(", "));
        return vec![sort_result(solution)];
    }
    if vector_contains_hashset(&dead_ends, &dead_end) {
        return Vec::new();
    }
    let is_outer_loop = solution.len() == 0;
    let mut words_left: Vec<Word> = Vec::new();
    for i in 0..words.len() {
        let collides = words[i]
            .path
            .clone()
            .into_iter()
            .any(|node| visited.contains(&node));
        if !collides {
            words_left.push(words[i].clone());
        }
    }
    if words_left.len() == 0usize {
        dead_ends.push(dead_end);
        return Vec::new();
    }
    let mut results: Vec<Vec<Word>> = Vec::new();
    words_left.reverse();
    while !words_left.is_empty() {
        let word = words_left.pop().unwrap();
        if is_outer_loop {
            println!("Start word: {:?}", word);
        }
        let current_word: Word = word.clone();
        let mut inner_solution = solution.clone();
        inner_solution.push(current_word);
        let res = find_solution(
            &words_left,
            &inner_solution,
            dead_ends,
            done_count,
            max_result_count,
        );
        if res.is_empty() {
            if is_outer_loop {
                println!("Dead end {:?}", inner_solution);
            }
            continue;
        } else {
            results = combine_results(&results, &res);
        }
        if results.len() >= max_result_count {
            break;
        }
    }
    return results;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let word_list_path: String = args[1].to_string();
    let board_path: String = args[2].to_string();
    let max_count: usize = usize::from_str_radix(&args[3].to_string(), 10).unwrap();

    println!("Searching for {}", word_list_path);
    println!("In file {}", board_path);
    println!("Max results, {}", max_count);
    let read_res = read_csv(Path::new(&word_list_path));
    let words: Vec<String> = match read_res {
        Ok(data) => data.into_iter().map(|x| x.to_uppercase()).collect(),
        Err(err) => panic!("Problem opening file, {:?}", err),
    };
    println!("Found {:?} words from the static word list", words.len());
    let board_res = read_board(Path::new(&board_path));
    let board = match board_res {
        Ok(data) => data,
        Err(err) => panic!("Problem reading board, {:?}", err),
    };

    let mut matches: Vec<Word> = Vec::new();
    println!("Searching for all available words for this board");
    for j in 0..board.len() {
        let row = &board[j];
        for i in 0..row.len() {
            let pos = Pos {
                x: i32::try_from(i).unwrap(),
                y: i32::try_from(j).unwrap(),
            };
            let mut inner_matches = find_words_starting_from(&board, &words, pos);
            matches.append(&mut inner_matches);
        }
    }
    matches.sort_by_cached_key(|x| x.path.len());
    matches.reverse();
    println!("Words found, {:?}", words.len());
    let done_count: i32 = i32::try_from(board.len() * board[0].len()).unwrap();
    let dead_ends: &mut Vec<HashSet<Pos>> = &mut Vec::new();
    let res = find_solution(&matches, &Vec::new(), dead_ends, done_count, max_count);
    for i in 0..res.len() {
        println!("Result #{:?}", i);
        for w in res[i].clone() {
            println!("{:?}", w.word);
            print_board(&board, &w.path)
        }
        print!("\n");
    }
}
