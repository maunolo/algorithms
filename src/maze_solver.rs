const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum Error {
    NoSolution,
    InvalidMaze,
    OutOfBounds,
    OnWall,
    AlreadySeen,
}

fn walk(
    maze: &mut Vec<Vec<&str>>,
    wall: &str,
    curr: &Point,
    end: &Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> Result<bool, Error> {
    if let Some(first_row) = maze.first() {
        // 1. Base Case
        // off the map
        if curr.x >= first_row.len() || curr.y >= maze.len() {
            return Ok(false);
        }

        // on a wall
        if maze[curr.y][curr.x] == wall {
            return Ok(false);
        }

        // reached the end
        if curr.x == end.x && curr.y == end.y {
            path.push(end.clone());
            return Ok(true);
        }

        // already seen
        if seen[curr.y][curr.x] {
            return Ok(false);
        }

        // 3 recurse
        // pre
        seen[curr.y][curr.x] = true;
        path.push(curr.clone());

        // recurse
        for direction in DIRECTIONS.iter() {
            let new_x: Result<usize, _> = (curr.x as isize + direction.0).try_into();
            let new_y: Result<usize, _> = (curr.y as isize + direction.1).try_into();

            if let (Ok(x), Ok(y)) = (new_x, new_y) {
                let next = Point { x, y };

                if walk(maze, wall, &next, end, seen, path)? {
                    return Ok(true);
                }
            }
        }

        //post
        path.pop();

        Ok(false)
    } else {
        Err(Error::InvalidMaze)
    }
}

pub fn solve(
    mut maze: Vec<Vec<&str>>,
    wall: &str,
    start: Point,
    end: Point,
) -> Result<Vec<Point>, Error> {
    let mut seen: Vec<Vec<bool>> = maze
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    let mut path: Vec<Point> = vec![];

    walk(&mut maze, wall, &start, &end, &mut seen, &mut path)?;

    if path.is_empty() {
        Err(Error::NoSolution)
    } else {
        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn maze_solver_test() {
        let maze = vec![
            vec!["X", "X", "X", "X", "X", "X", "X", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", " ", " ", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", " ", " ", "X", " ", "X"],
            vec!["X", " ", "X", "X", "X", "X", "X", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", " ", " ", " ", " ", "X"],
            vec!["X", " ", "X", "X", "X", "X", "X", "X", "X", "X"],
        ];

        let start = Point { x: 8, y: 0 };
        let end = Point { x: 1, y: 5 };

        let result = solve(maze, "X", start, end);
        assert!(result.is_ok());

        let path = result.unwrap();

        assert_eq!(
            path,
            vec![
                Point { x: 8, y: 0 },
                Point { x: 8, y: 1 },
                Point { x: 8, y: 2 },
                Point { x: 8, y: 3 },
                Point { x: 8, y: 4 },
                Point { x: 7, y: 4 },
                Point { x: 6, y: 4 },
                Point { x: 5, y: 4 },
                Point { x: 4, y: 4 },
                Point { x: 3, y: 4 },
                Point { x: 2, y: 4 },
                Point { x: 1, y: 4 },
                Point { x: 1, y: 5 },
            ]
        );
    }

    #[test]
    fn maze_solver_test_no_solution() {
        let maze = vec![
            vec!["X", "X", "X", "X", "X", "X", "X", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", " ", " ", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", " ", " ", "X", " ", "X"],
            vec!["X", " ", "X", "X", "X", "X", "X", "X", " ", "X"],
            vec!["X", " ", " ", " ", " ", "X", " ", " ", " ", "X"],
            vec!["X", " ", "X", "X", "X", "X", "X", "X", "X", "X"],
        ];

        let start = Point { x: 8, y: 0 };
        let end = Point { x: 1, y: 5 };

        let result = solve(maze, "X", start, end);
        assert!(result.is_err());

        let path = result.unwrap_err();

        assert!(matches!(path, Error::NoSolution));
    }

    #[test]
    fn maze_solver_test_invalid_maze() {
        let maze = vec![];

        let start = Point { x: 8, y: 0 };
        let end = Point { x: 1, y: 5 };

        let result = solve(maze, "X", start, end);
        assert!(result.is_err());

        let path = result.unwrap_err();

        assert!(matches!(path, Error::InvalidMaze));
    }
}
