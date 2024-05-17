use crate::common::{Pos, Word};
use crate::neighbors::get_neighbors;


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
    // Check exact matches
    for w in words.iter() {
        if *w == current_word {
            results.push(Word {
                path: path.to_vec(),
                word: w.to_string(),
            });
            break;
        }
    }
    for neighbor in get_neighbors(pos, &visited) {
        let mut inner_visited = visited.clone();
        let x = usize::try_from(neighbor.x).unwrap();
        let y = usize::try_from(neighbor.y).unwrap();
        let mut inner_path = path.clone();
        inner_path.push(Pos {
            x: neighbor.x,
            y: neighbor.y,
        });
        inner_visited[y][x] = true;
        let inner_word = current_word.clone();
        let mut inner_words: Vec<String> = Vec::new();
        for w in words.iter() {
            if w.starts_with(&current_word) {
                inner_words.push(w.to_string());
            }
        }
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

pub fn find_words_starting_from(
    board: &Vec<Vec<char>>,
    words: &Vec<String>,
    start_pos: Pos,
) -> Vec<Word> {
    let mut visited: Vec<Vec<bool>> = board.iter().map(|row| vec![false; row.len()]).collect();
    // Set starting position to visited
    let xusize = start_pos.x;
    let yusize = start_pos.y;
    visited[yusize][xusize] = true;
    let mut path: Vec<Pos> = Vec::new();
    path.push(start_pos.clone());
    let res = inner(board, &visited, &words, &start_pos, String::new(), &path);
    return res;
}