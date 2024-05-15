use crate::Pos;

pub fn get_neighbors(current: &Pos, visited: &Vec<Vec<bool>>) -> Vec<Pos> {
    let mut results: Vec<Pos> = Vec::new();
    for j in 0..3 {
        if current.y + j == 0 {
            continue;
        }
        let y = current.y + j - 1;
        for i in 0..3 {
            if current.x + i == 0 {
                continue;
            }
            let x = current.x + i - 1;
            if i == 1 && j == 1 {
                continue;
            }
            if x < visited[0].len() && y < visited.len() {
                if !visited[y][x] {
                    results.push(Pos { x, y });
                }
            }
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_visited(rows: usize, cols: usize) -> Vec<Vec<bool>> {
        let mut visited: Vec<Vec<bool>> = Vec::new();
        for _i in 0..rows {
            let row: Vec<bool> = vec![false; cols];
            visited.push(row);
        }
        return visited;
    }

    #[test]
    fn no_visited() {
        let pos = Pos { x: 1, y: 1 };
        let visited = build_visited(6, 5);
        let res = get_neighbors(&pos, &visited);
        assert_eq!(8, res.len());
        println!("{:?}", res);
    }
    #[test]
    fn edges() {
        let visited = build_visited(2, 2);
        assert_eq!(3, get_neighbors(&Pos { x: 0, y: 0 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 0, y: 1 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 1, y: 0 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 1, y: 1 }, &visited).len());
    }
    #[test]
    fn edges_3() {
        let visited = build_visited(3, 3);
        assert_eq!(3, get_neighbors(&Pos { x: 0, y: 0 }, &visited).len());
        assert_eq!(5, get_neighbors(&Pos { x: 0, y: 1 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 0, y: 2 }, &visited).len());
        assert_eq!(5, get_neighbors(&Pos { x: 1, y: 0 }, &visited).len());
        assert_eq!(8, get_neighbors(&Pos { x: 1, y: 1 }, &visited).len());
        assert_eq!(5, get_neighbors(&Pos { x: 1, y: 2 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 2, y: 0 }, &visited).len());
        assert_eq!(5, get_neighbors(&Pos { x: 2, y: 1 }, &visited).len());
        assert_eq!(3, get_neighbors(&Pos { x: 2, y: 2 }, &visited).len());
    }

    #[test]
    fn visited() {
        let visited: Vec<Vec<bool>> = vec![vec![true; 2]; 2];
        assert_eq!(0, get_neighbors(&Pos { x: 0, y: 0 }, &visited).len());
        assert_eq!(0, get_neighbors(&Pos { x: 0, y: 1 }, &visited).len());
        assert_eq!(0, get_neighbors(&Pos { x: 1, y: 0 }, &visited).len());
        assert_eq!(0, get_neighbors(&Pos { x: 1, y: 1 }, &visited).len());
    }
}
