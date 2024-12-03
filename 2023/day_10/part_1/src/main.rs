#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Start,
    Vertical,
    Horizontal,
    NorthEastBend,
    SouthEastBend,
    NorthWestBend,
    SouthWestBend,
}

impl Tile {
    fn new(c: char) -> Self {
        use Tile::*;
        match c {
            '.' => Ground,
            'S' => Start,
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEastBend,
            'F' => SouthEastBend,
            'J' => NorthWestBend,
            '7' => SouthWestBend,
            _ => panic!("Invalid char"),
        }
    }
}

fn flood_fill(
    grid: &Vec<Vec<Tile>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    index: usize,
) -> (usize, bool) {
    if visited[y][x] {
        if grid[y][x] == Tile::Start && index >= 3 {
            return (index + 1, true);
        } else {
            return (index + 1, false);
        }
    }
    visited[y][x] = true;

    // can go north
    if y != 0
        && match grid[y - 1][x] {
            Tile::Vertical | Tile::SouthWestBend | Tile::SouthEastBend | Tile::Start => true,
            _ => false,
        }
    {
        let (i, success) = flood_fill(grid, visited, x, y - 1, width, height, index + 1);
        if success {
            return (i, success);
        }
    }

    // can go south
    if y < height - 1
        && match grid[y + 1][x] {
            Tile::Vertical | Tile::NorthWestBend | Tile::NorthEastBend | Tile::Start => true,
            _ => false,
        }
    {
        let (i, success) = flood_fill(grid, visited, x, y + 1, width, height, index + 1);
        if success {
            return (i, success);
        }
    }

    // can go west
    if x != 0
        && match grid[y][x - 1] {
            Tile::Horizontal | Tile::NorthEastBend | Tile::SouthEastBend | Tile::Start => true,
            _ => false,
        }
    {
        let (i, success) = flood_fill(grid, visited, x - 1, y, width, height, index + 1);
        if success {
            return (i, success);
        }
    }

    // can go east
    if x < width - 1
        && match grid[y][x + 1] {
            Tile::Horizontal | Tile::NorthWestBend | Tile::SouthWestBend | Tile::Start => true,
            _ => false,
        }
    {
        let (i, success) = flood_fill(grid, visited, x + 1, y, width, height, index + 1);
        if success {
            return (i, success);
        }
    }
    (index, false)
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().map(|c| Tile::new(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();
    let start = {
        let mut s = (0usize, 0usize);
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    Tile::Start => s = (x, y),
                    _ => continue,
                };
            }
        }
        s
    };

    let mut visited = {
        let mut v = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(false);
            }
            v.push(row);
        }
        v
    };

    let (count, success) = flood_fill(&grid, &mut visited, start.0, start.1, width, height, 0);

    println!("result: {}, {}", success, count / 2);
}
