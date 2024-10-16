use crate::maze_maker::{Cell, Maze, MazeType};
use crate::maze_printer::Printer;
use crossterm::terminal::ClearType;
use crossterm::{execute, terminal};
use std::io::stdout;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

enum Ways {
    Up,
    Down,
    Left,
    Right,
}
impl Ways {
    fn next_way(&self, h: usize, w: usize, maze: &MazeType) -> Option<(usize, usize)> {
        let (p_h, p_w) = Maze::hw(maze);
        match self {
            Ways::Up => {
                if h > 0 {
                    Some((h - 1, w))
                } else {
                    None
                }
            }
            Ways::Down => {
                if h + 1 < p_h {
                    Some((h + 1, w))
                } else {
                    None
                }
            }
            Ways::Left => {
                if w > 0 {
                    Some((h, w - 1))
                } else {
                    None
                }
            }
            Ways::Right => {
                if w + 1 < p_w {
                    Some((h, w + 1))
                } else {
                    None
                }
            }
        }
    }
}
pub type Init<T> = T;
pub struct MazeSolver {
    maze: MazeType,
    steps: i32,
}
impl MazeSolver {
    pub fn make(maze: MazeType) -> MazeSolver {
        MazeSolver { maze, steps: 0 }
    }
    fn sleep() {
        sleep(Duration::from_millis(30));
    }

    pub fn init(self) -> Init<MazeSolver> {
        execute!(stdout(), terminal::Clear(ClearType::All));
        self.render();
        self
    }
    fn step(&mut self) {
        self.steps += 1;
        self.render();
        MazeSolver::sleep();
    }
    fn render(&self) {
        Printer::print_to_screen(&self.maze);
        Printer::print_example(&self.maze);
        Printer::print_status(&self.maze, self.steps);
        Printer::cursor_rst();
    }
    pub fn solve(&mut self, pos_h: usize, pos_w: usize) {
        if self.maze[pos_h][pos_w] != Cell::Air && self.maze[pos_h][pos_w] != Cell::Exit {
            return;
        }
        let mut is_exit = false;
        if self.maze[pos_h][pos_w] == Cell::Exit {
            is_exit = true;
        }
        self.maze[pos_h][pos_w] = Cell::Vising;
        self.step();
        self.maze[pos_h][pos_w] = Cell::Vised;
        if is_exit {
            sleep(Duration::from_secs(10));
            exit(0);
        }
        if let Some((h, w)) = Ways::Up.next_way(pos_h, pos_w, &self.maze) {
            self.solve(h, w);
        }
        if let Some((h, w)) = Ways::Down.next_way(pos_h, pos_w, &self.maze) {
            self.solve(h, w);
        }
        if let Some((h, w)) = Ways::Left.next_way(pos_h, pos_w, &self.maze) {
            self.solve(h, w);
        }
        if let Some((h, w)) = Ways::Right.next_way(pos_h, pos_w, &self.maze) {
            self.solve(h, w);
        }
        self.maze[pos_h][pos_w] = Cell::Vising;
        self.step();
        self.maze[pos_h][pos_w] = Cell::Vised;
    }
}
