use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn create_neighbors(current: &Point) -> [Point; 4] {
    [
        Point {
            x: current.x + 1,
            y: current.y,
        },
        Point {
            x: current.x,
            y: current.y + 1,
        },
        Point {
            x: current.x.checked_sub(1).unwrap_or(0),
            y: current.y,
        },
        Point {
            x: current.x,
            y: current.y.checked_sub(1).unwrap_or(0),
        },
    ]
}

fn dfs(
    maze: &Vec<Vec<char>>,
    current: Point,
    end: &Point,
    visited: &mut HashSet<Point>,
    path: &mut HashSet<Point>,
) -> bool {
    if current == *end {
        path.insert(current.clone());
        return true;
    }

    if visited.contains(&current) || maze[current.y as usize][current.x as usize] == 'x' {
        return false;
    }

    visited.insert(current.clone());
    path.insert(current.clone());

    for neighbor in create_neighbors(&current) {
        if dfs(maze, neighbor, end, visited, path) {
            return true;
        }
    }

    path.remove(&current);
    false
}

fn solve_maze(maze: &Vec<Vec<char>>, start: Point, end: Point) -> Option<HashSet<Point>> {
    let mut visited = HashSet::new();
    let mut path = HashSet::new();

    if dfs(maze, start, &end, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

#[test]
fn solves_maze() {
    let pretty_maze: Vec<&str> = vec![
        "xxxxxxxxxx x",
        "x        x x",
        "x        x x",
        "x xxxxxxxx x",
        "x          x",
        "x xxxxxxxxxx",
    ];

    let maze: Vec<Vec<char>> = pretty_maze
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let maze_result = HashSet::from([
        Point { x: 10, y: 0 },
        Point { x: 10, y: 1 },
        Point { x: 10, y: 2 },
        Point { x: 10, y: 3 },
        Point { x: 10, y: 4 },
        Point { x: 9, y: 4 },
        Point { x: 8, y: 4 },
        Point { x: 7, y: 4 },
        Point { x: 6, y: 4 },
        Point { x: 5, y: 4 },
        Point { x: 4, y: 4 },
        Point { x: 3, y: 4 },
        Point { x: 2, y: 4 },
        Point { x: 1, y: 4 },
        Point { x: 1, y: 5 },
    ]);

    let result = solve_maze(&maze, Point { x: 10, y: 0 }, Point { x: 1, y: 5 });
    println!("RESULT {:#?}", draw_path(&maze, result.clone().unwrap()));
    println!("ANSWER {:#?}", draw_path(&maze, maze_result.clone()));

    assert_eq!(
        draw_path(&maze, result.unwrap()),
        draw_path(&maze, maze_result)
    )
}

fn draw_path(data: &[Vec<char>], path: HashSet<Point>) -> Vec<String> {
    let mut maze = data.to_owned();
    for point in path {
        maze[point.y as usize][point.x as usize] = '*';
    }

    maze.iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect()
}
