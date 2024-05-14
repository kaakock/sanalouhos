#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Word {
    pub path: Vec<Pos>,
    pub word: String,
}

pub fn print_board(board: &Vec<Vec<char>>, highlight: &Vec<Pos>) {
    for j in 0..board.len() {
        let mut row = String::new();
        for i in 0..board[j].len() {
            let convert_uppercase = highlight.contains(&Pos {
                x: i32::try_from(i).unwrap(),
                y: i32::try_from(j).unwrap(),
            });
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
pub fn print_visited(board: &Vec<Vec<bool>>) {
    println!("Visited:");
    for line in board.iter() {
        println!(
            "{:?}",
            String::from_iter(
                line.iter()
                    .map(|&x| if x { return "X" } else { return "O" })
            )
        );
    }
}

fn path_to_string(path: &Vec<Pos>) -> String {
    let string_vec: Vec<String> = path
        .into_iter()
        .map(|x| x.x.to_string() + &x.y.to_string())
        .collect();
    return string_vec.join("");
}

fn words_to_string(a: &Vec<Word>) -> String {
    let mut as_vec: Vec<String> = a
        .clone()
        .into_iter()
        .map(|w| path_to_string(&w.path))
        .collect();
    as_vec.sort_by_cached_key(|x| x.to_string());
    return as_vec.join("");
}

pub fn sort_result(a: &Vec<Word>) -> Vec<Word> {
    let mut as_vec: Vec<Word> = a.clone();
    as_vec.sort_by_cached_key(|x| path_to_string(&x.path));
    return as_vec;
}