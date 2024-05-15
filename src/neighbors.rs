use crate::Pos;

pub fn get_neighbors(current: &Pos, visited: &Vec<Vec<bool>>) -> Vec<Pos> {
    let mut results: Vec<Pos> = Vec::new();
    for j in -1..2 {
        let y = current.y + j;
        for i in -1..2 {
            let x = current.x + i;
            if x == 0 && y == 0 {
                continue;
            }
            if x >= 0
                && x < visited[0].len().try_into().unwrap()
                && y >= 0
                && y < visited.len().try_into().unwrap()
            {
                let xusize = usize::try_from(x).unwrap();
                let yusize = usize::try_from(y).unwrap();
                if !visited[yusize][xusize] {
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
