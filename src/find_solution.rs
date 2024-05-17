use crate::common::Word1D;


// Function to combine two vectors of solutions keeping only unique results
fn combine_results(a: &[Vec<Word1D>], b: &[Vec<Word1D>]) -> Vec<Vec<Word1D>> {
    let mut combined_results: Vec<Vec<Word1D>> = Vec::new();

    // Combine a and b into combined_results
    combined_results.extend_from_slice(a);
    combined_results.extend_from_slice(b);
    // Deduplicate the combined results
    combined_results.sort();
    combined_results.dedup();

    return combined_results;
}

fn intersects(a: &Word1D, b: &Word1D) -> bool {
    return (a & b) > 0;
}

fn add(a: &Word1D, b: &Word1D) -> Word1D {
    return a | b;
}

fn is_done(a: &Word1D) -> bool {
    // the board is 6x5 = 30 so full board equals 0011...11 as i32
    return Word1D::MAX >> 2 == *a;
}

pub fn solve(
    words: Vec<&Word1D>,
    solution: &Vec<&Word1D>,
    visited: &Word1D,
    max_result_count: usize,
) -> Vec<Vec<Word1D>> {
    if is_done(&visited) {
        let mut result_vec: Vec<Word1D> = Vec::new();
        for i in solution {
            result_vec.push(**i);
        }
        return vec![result_vec];
    }
    let mut words_left: Vec<&Word1D> = Vec::new();
    for i in 0..words.len() {
        if !intersects(visited, &words[i]) {
            words_left.push(&words[i]);
        }
    }
    if words_left.len() == 0usize {
        return Vec::new();
    }
    let mut results: Vec<Vec<Word1D>> = Vec::new();
    while !words_left.is_empty() {
        let word = words_left.pop().unwrap();
        let mut inner_solution = solution.clone();
        inner_solution.push(&word);
        let inner_visited = add(visited, word);
        let res = solve(
            words_left.clone(),
            &inner_solution,
            &inner_visited,
            max_result_count,
        );
        if res.is_empty() {
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