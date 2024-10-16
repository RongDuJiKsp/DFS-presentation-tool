use crossterm::style::{Color, StyledContent, Stylize};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Cell {
    Air,
    Wall,
    Vising,
    Vised,
    Exit,
}
impl Cell {
    pub fn block(&self) -> StyledContent<String> {
        let space = String::from("  ");
        match self {
            Cell::Air => space.clone().with(Color::Black).on(Color::White),
            Cell::Wall => space.clone().with(Color::Black).on(Color::DarkGreen),
            Cell::Vising => space.clone().with(Color::Black).on(Color::Yellow),
            Cell::Vised => space.clone().with(Color::Black).on(Color::DarkYellow),
            Cell::Exit => space.clone().with(Color::Black).on(Color::DarkRed),
        }
    }
}
pub type MazeType = Vec<Vec<Cell>>;

pub struct Maze;
impl Maze {
    pub fn hw(maze: &MazeType) -> (usize, usize) {
        (maze.len(), maze[0].len())
    }
    pub fn screen_hw(maze: &MazeType) -> (usize, usize) {
        (maze.len(), maze[0].len() * 2)
    }
    fn random_directions() -> Vec<(isize, isize)> {
        let mut directions = vec![
            (0, -2), // 上
            (0, 2),  // 下
            (-2, 0), // 左
            (2, 0),  // 右
        ];
        directions.shuffle(&mut thread_rng());
        directions
    }
    fn carve_passages(maze: &mut MazeType, x: usize, y: usize) {
        let (height, width) = Maze::hw(maze);
        maze[y][x] = Cell::Air;
        for (dx, dy) in Maze::random_directions() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx > 0 && ny > 0 && (nx as usize) < width && (ny as usize) < height {
                let nx = nx as usize;
                let ny = ny as usize;
                if maze[ny][nx] == Cell::Wall {
                    maze[(y + ny) / 2][(x + nx) / 2] = Cell::Air;
                    Maze::carve_passages(maze, nx, ny);
                }
            }
        }
    }

    pub fn make(width: usize, height: usize) -> MazeType {
        if width % 2 == 0 || height % 2 == 0 {
            panic!("生成的迷宫边长必须奇数");
        }
        let mut maze = vec![vec![Cell::Wall; width]; height];
        Maze::carve_passages(&mut maze, 1, 1);
        maze[height - 1][width - 2] = Cell::Exit;
        maze
    }
}
